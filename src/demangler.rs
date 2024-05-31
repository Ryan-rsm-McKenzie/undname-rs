// Copyright 2024 Ryan McKenzie
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    cache::{
        ArrayType,
        ConversionOperatorIdentifier,
        CustomType,
        EncodedStringLiteral,
        FunctionSignature,
        FunctionSymbol,
        LiteralOperatorIdentifier,
        LocalStaticGuardVariable,
        Md5Symbol,
        NamedIdentifier,
        NodeArray,
        NodeCache,
        NodeHandle,
        PointerType,
        PrimitiveType,
        QualifiedName,
        SpecialTableSymbol,
        StructorIdentifier,
        TagType,
        VariableSymbol,
    },
    extensions::{
        BStrExt as _,
        U8Ext as _,
    },
    nodes::{
        ArrayTypeNode,
        CallingConv,
        CharKind,
        ConversionOperatorIdentifierNode,
        CustomTypeNode,
        DynamicStructorIdentifierNode,
        EncodedStringLiteralNode,
        FuncClass,
        FunctionRefQualifier,
        FunctionSignatureNode,
        FunctionSymbolNode,
        IIdentifierNode,
        INode,
        ISignatureNode,
        ISymbolNode,
        ITypeNode,
        IdentifierNode,
        IntegerLiteralNode,
        IntrinsicFunctionIdentifierNode,
        IntrinsicFunctionKind,
        LiteralOperatorIdentifierNode,
        LocalStaticGuardIdentifierNode,
        LocalStaticGuardVariableNode,
        Md5SymbolNode,
        NamedIdentifierNode,
        NodeArrayNode,
        PointerAffinity,
        PointerTypeNode,
        PrimitiveKind,
        PrimitiveTypeNode,
        QualifiedNameNode,
        Qualifiers,
        RttiBaseClassDescriptorNode,
        SpecialIntrinsicKind,
        SpecialTableSymbolNode,
        StorageClass,
        StructorIdentifierNode,
        TagKind,
        TagTypeNode,
        TemplateParameterReferenceNode,
        TemplateParameters,
        ThunkSignatureNode,
        TypeNode,
        VariableSymbolNode,
        VcallThunkIdentifierNode,
        WriteableNode as _,
    },
    Allocator,
    Error,
    OutputBuffer,
    OutputFlags,
    Result,
};
use arrayvec::ArrayVec;
use bstr::{
    BStr,
    BString,
    ByteSlice as _,
};
use smallvec::SmallVec;
use std::{
    io::{
        self,
        Write as _,
    },
    mem,
};

#[derive(Default)]
struct BackrefContext {
    function_params: ArrayVec<NodeHandle<ITypeNode>, 10>,

    // The first 10 BackReferences in a mangled name can be back-referenced by
    // special name @[0-9]. This is a storage for the first 10 BackReferences.
    names: ArrayVec<NodeHandle<NamedIdentifier>, 10>,
}

#[derive(Clone, Copy)]
enum QualifierMangleMode {
    Drop,
    Mangle,
    Result,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Default)]
    struct NameBackrefBehavior: u8 {
        const NBB_None = 0;          // don't save any names as backrefs.
        const NBB_Template = 1 << 0; // save template instanations.
        const NBB_Simple = 1 << 1;   // save simple names.
    }
}

impl NameBackrefBehavior {
    #[must_use]
    fn is_template(self) -> bool {
        self.contains(Self::NBB_Template)
    }

    #[must_use]
    fn is_simple(self) -> bool {
        self.contains(Self::NBB_Simple)
    }
}

#[derive(Clone, Copy)]
enum FunctionIdentifierCodeGroup {
    Basic,
    Under,
    DoubleUnder,
}

#[derive(Default)]
pub(crate) struct Demangler<'alloc> {
    cache: NodeCache<'alloc>,
    backrefs: BackrefContext,
}

impl<'alloc, 'string: 'alloc> Demangler<'alloc> {
    pub(crate) fn parse_into(
        &mut self,
        allocator: &'alloc Allocator,
        mut mangled_name: &'string BStr,
        flags: OutputFlags,
        result: &mut BString,
    ) -> Result<()> {
        let ast = self
            .do_parse(allocator, &mut mangled_name)?
            .resolve(&self.cache);
        let mut ob: OutputBuffer = mem::take(result).into();
        ast.output(&self.cache, &mut ob, flags)?;
        *result = ob.into();
        Ok(())
    }

    fn do_parse(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<ISymbolNode>> {
        // Typeinfo names are strings stored in RTTI data. They're not symbol names.
        // It's still useful to demangle them. They're the only demangled entity
        // that doesn't start with a "?" but a ".".
        if mangled_name.starts_with(b".") {
            self.demangle_typeinfo_name(allocator, mangled_name)
                .map(Into::into)
        } else if mangled_name.starts_with(b"??@") {
            self.demangle_md5_name(allocator, mangled_name)
                .map(Into::into)
        } else {
            mangled_name
                .try_consume_byte(b'?')
                .ok_or(Error::Io(io::ErrorKind::UnexpectedEof.into()))?;
            // ?$ is a template instantiation, but all other names that start with ? are
            // operators / special names.
            if let Some(si) = self.demangle_special_intrinsic(allocator, mangled_name)? {
                Ok(si)
            } else {
                self.demangle_declarator(allocator, mangled_name)
            }
        }
    }

    fn demangle_encoded_symbol(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
        name: NodeHandle<QualifiedName>,
    ) -> Result<NodeHandle<ISymbolNode>> {
        let c = mangled_name.first().ok_or(Error::InvalidEncodedSymbol)?;

        // Read a variable.
        if matches!(c, b'0' | b'1' | b'2' | b'3' | b'4') {
            let sc = Self::demangle_variable_storage_class(mangled_name)?;
            let result = self.demangle_variable_encoding(allocator, mangled_name, sc)?;
            return Ok(result.into());
        }
        let fsn = self.demangle_function_encoding(allocator, mangled_name)?;
        let target_type = fsn
            .resolve(&self.cache)
            .signature
            .resolve(&self.cache)
            .as_node()
            .return_type;

        let uqn = name
            .resolve(&self.cache)
            .get_unqualified_identifier(&self.cache)
            .ok_or(Error::InvalidEncodedSymbol)?
            .resolve_mut(&mut self.cache);
        if let IdentifierNode::ConversionOperatorIdentifier(coin) = uqn {
            coin.target_type = target_type;
        }

        Ok(fsn.into())
    }

    fn demangle_declarator(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<ISymbolNode>> {
        // What follows is a main symbol name. This may include namespaces or class
        // back references.
        let qn = self.demangle_fully_qualified_symbol_name(allocator, mangled_name)?;
        let symbol = self.demangle_encoded_symbol(allocator, mangled_name, qn)?;
        symbol.resolve_mut(&mut self.cache).set_name(qn);

        let uqn = qn
            .resolve(&self.cache)
            .get_unqualified_identifier(&self.cache)
            .ok_or(Error::InvalidDeclarator)?
            .resolve(&self.cache);
        if let IdentifierNode::ConversionOperatorIdentifier(coin) = uqn {
            if coin.target_type.is_none() {
                return Err(Error::InvalidDeclarator);
            }
        }

        Ok(symbol)
    }

    fn demangle_md5_name(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<Md5Symbol>> {
        let mangled_copy = *mangled_name;

        // This is an MD5 mangled name. We can't demangle it, just return the mangled name.
        // An MD5 mangled name is ??@ followed by 32 characters and a terminating @.
        let mut stop = {
            let prefix = mangled_name
                .try_consume_str(b"??@")
                .ok_or(Error::InvalidMd5Name)?;
            let stop = mangled_name.find_byte(b'@').ok_or(Error::InvalidMd5Name)?;
            mangled_name
                .try_consume_n(stop + 1)
                .ok_or(Error::InvalidMd5Name)?;
            stop + prefix.len()
        };

        // There are two additional special cases for MD5 names:
        // 1. For complete object locators where the object name is long enough
        //    for the object to have an MD5 name, the complete object locator is
        //    called ??@...@??_R4@ (with a trailing "??_R4@" instead of the usual
        //    leading "??_R4". This is handled here.
        // 2. For catchable types, in versions of MSVC before 2015 (<1900) or after
        //    2017.2 (>= 1914), the catchable type mangling is _CT??@...@??@...@8
        //    instead of_CT??@...@8 with just one MD5 name. Since we don't yet
        //    demangle catchable types anywhere, this isn't handled for MD5 names
        //    either.
        if let Some(postfix) = mangled_name.try_consume_str(b"??_R4@") {
            stop += postfix.len();
        }

        let md5 = &mangled_copy[..=stop];
        let name = QualifiedNameNode::synthesize_from_name(allocator, &mut self.cache, md5);
        let s = Md5SymbolNode {
            name: self.cache.intern(allocator, name),
        };

        Ok(self.cache.intern(allocator, s))
    }

    fn demangle_typeinfo_name(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<VariableSymbol>> {
        mangled_name
            .try_consume_byte(b'.')
            .ok_or(Error::InvalidTypeinfoName)?;
        let t = self.demangle_type(allocator, mangled_name, QualifierMangleMode::Result)?;
        if !mangled_name.is_empty() {
            return Err(Error::InvalidTypeinfoName);
        }

        let variable = VariableSymbolNode::synthesize(
            allocator,
            &mut self.cache,
            t,
            b"`RTTI Type Descriptor Name'".into(),
        );
        Ok(self.cache.intern(allocator, variable))
    }

    // <type-encoding> ::= <storage-class> <variable-type>
    // <storage-class> ::= 0  # private static member
    //                 ::= 1  # protected static member
    //                 ::= 2  # public static member
    //                 ::= 3  # global
    //                 ::= 4  # static local
    fn demangle_variable_encoding(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
        sc: StorageClass,
    ) -> Result<NodeHandle<VariableSymbol>> {
        let r#type = self.demangle_type(allocator, mangled_name, QualifierMangleMode::Drop)?;

        // <variable-type> ::= <type> <cvr-qualifiers>
        //                 ::= <type> <pointee-cvr-qualifiers> # pointers, references
        match r#type.resolve_mut(&mut self.cache) {
            TypeNode::PointerType(ptn) => {
                ptn.quals |= Self::demangle_pointer_ext_qualifiers(mangled_name);
                let class_parent = ptn.class_parent;
                let pointee = ptn.pointee;

                let extra_child_quals = Self::demangle_qualifiers(mangled_name)?.0;
                if class_parent.is_some() {
                    _ = self.demangle_fully_qualified_type_name(allocator, mangled_name)?;
                }

                let mut pointee = pointee.resolve_mut(&mut self.cache);
                pointee.append_quals(extra_child_quals);
            }
            mut r#type => {
                let quals = Self::demangle_qualifiers(mangled_name)?.0;
                r#type.set_quals(quals);
            }
        }

        let vsn = VariableSymbolNode {
            name: None,
            sc: Some(sc),
            r#type: Some(r#type),
        };
        Ok(self.cache.intern(allocator, vsn))
    }

    fn demangle_function_encoding(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<FunctionSymbol>> {
        let mut extra_flags = FuncClass::FC_None;
        if mangled_name.try_consume_str(b"$$J0").is_some() {
            extra_flags |= FuncClass::FC_ExternC;
        }

        if mangled_name.is_empty() {
            return Err(Error::InvalidFunctionEncoding);
        }

        let fc = Self::demangle_function_class(mangled_name)? | extra_flags;

        // integral truncations here are on purpose
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let ttn = if fc.has_static_this_adjust() {
            let mut ttn = ThunkSignatureNode::default();
            ttn.this_adjust.static_offset = Self::demangle_signed(mangled_name)? as _;
            Some(ttn)
        } else if fc.has_virtual_this_adjust() {
            let mut ttn = ThunkSignatureNode::default();
            if fc.has_virtual_this_adjust_ex() {
                ttn.this_adjust.vbptr_offset = Self::demangle_signed(mangled_name)? as _;
                ttn.this_adjust.vboffset_offset = Self::demangle_signed(mangled_name)? as _;
            }
            ttn.this_adjust.vtor_disp_offset = Self::demangle_signed(mangled_name)? as _;
            ttn.this_adjust.static_offset = Self::demangle_signed(mangled_name)? as _;
            Some(ttn)
        } else {
            None
        };

        let fsn = if fc.no_parameter_list() {
            // This is an extern "C" function whose full signature hasn't been mangled.
            // This happens when we need to mangle a local symbol inside of an extern
            // "C" function.
            self.cache
                .intern(allocator, FunctionSignatureNode::default())
        } else {
            let has_this_quals = !fc.intersects(FuncClass::FC_Global | FuncClass::FC_Static);
            self.demangle_function_type(allocator, mangled_name, has_this_quals)?
        };

        let signature: NodeHandle<ISignatureNode> = if let Some(mut ttn) = ttn {
            ttn.function_node = *fsn.resolve(&self.cache);
            self.cache.intern(allocator, ttn).into()
        } else {
            fsn.into()
        };
        signature
            .resolve_mut(&mut self.cache)
            .set_function_class(fc);

        Ok(self.cache.intern(
            allocator,
            FunctionSymbolNode {
                name: None,
                signature,
            },
        ))
    }

    #[must_use]
    fn demangle_pointer_ext_qualifiers(mangled_name: &mut &'string BStr) -> Qualifiers {
        let mut quals = Qualifiers::Q_None;
        if mangled_name.try_consume_byte(b'E').is_some() {
            quals |= Qualifiers::Q_Pointer64;
        }
        if mangled_name.try_consume_byte(b'I').is_some() {
            quals |= Qualifiers::Q_Restrict;
        }
        if mangled_name.try_consume_byte(b'F').is_some() {
            quals |= Qualifiers::Q_Unaligned;
        }
        quals
    }

    // <variable-type> ::= <type> <cvr-qualifiers>
    //                 ::= <type> <pointee-cvr-qualifiers> # pointers, references
    fn demangle_type(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
        qmm: QualifierMangleMode,
    ) -> Result<NodeHandle<ITypeNode>> {
        let quals = match qmm {
            QualifierMangleMode::Mangle => Self::demangle_qualifiers(mangled_name)?.0,
            QualifierMangleMode::Result => {
                if mangled_name.try_consume_byte(b'?').is_some() {
                    Self::demangle_qualifiers(mangled_name)?.0
                } else {
                    Qualifiers::Q_None
                }
            }
            QualifierMangleMode::Drop => Qualifiers::Q_None,
        };

        if mangled_name.is_empty() {
            return Err(Error::InvalidType);
        }

        let ty: NodeHandle<ITypeNode> = if mangled_name.is_tag_type() {
            self.demangle_class_type(allocator, mangled_name)
                .map(Into::into)
        } else if mangled_name.is_pointer_type() {
            match mangled_name.is_member_pointer() {
                Some(true) => self
                    .demangle_member_pointer_type(allocator, mangled_name)
                    .map(Into::into),
                Some(false) => self
                    .demangle_pointer_type(allocator, mangled_name)
                    .map(Into::into),
                None => Err(Error::InvalidType),
            }
        } else if mangled_name.is_array_type() {
            self.demangle_array_type(allocator, mangled_name)
                .map(Into::into)
        } else if mangled_name.is_function_type() {
            if mangled_name.try_consume_str(b"$$A8@@").is_some() {
                self.demangle_function_type(allocator, mangled_name, true)
                    .map(Into::into)
            } else if mangled_name.try_consume_str(b"$$A6").is_some() {
                self.demangle_function_type(allocator, mangled_name, false)
                    .map(Into::into)
            } else {
                Err(Error::InvalidType)
            }
        } else if mangled_name.is_custom_type() {
            self.demangle_custom_type(allocator, mangled_name)
                .map(Into::into)
        } else {
            self.demangle_primitive_type(allocator, mangled_name)
                .map(Into::into)
        }?;

        ty.resolve_mut(&mut self.cache).append_quals(quals);
        Ok(ty)
    }

    fn demangle_primitive_type(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<PrimitiveType>> {
        let kind = if mangled_name.try_consume_str(b"$$T").is_some() {
            PrimitiveKind::Nullptr
        } else {
            let f = mangled_name
                .try_consume()
                .ok_or(Error::InvalidPrimitiveType)?;
            match f {
                b'X' => PrimitiveKind::Void,
                b'D' => PrimitiveKind::Char,
                b'C' => PrimitiveKind::Schar,
                b'E' => PrimitiveKind::Uchar,
                b'F' => PrimitiveKind::Short,
                b'G' => PrimitiveKind::Ushort,
                b'H' => PrimitiveKind::Int,
                b'I' => PrimitiveKind::Uint,
                b'J' => PrimitiveKind::Long,
                b'K' => PrimitiveKind::Ulong,
                b'M' => PrimitiveKind::Float,
                b'N' => PrimitiveKind::Double,
                b'O' => PrimitiveKind::Ldouble,
                b'_' => {
                    let f = mangled_name
                        .try_consume()
                        .ok_or(Error::InvalidPrimitiveType)?;
                    match f {
                        b'N' => PrimitiveKind::Bool,
                        b'J' => PrimitiveKind::Int64,
                        b'K' => PrimitiveKind::Uint64,
                        b'W' => PrimitiveKind::Wchar,
                        b'Q' => PrimitiveKind::Char8,
                        b'S' => PrimitiveKind::Char16,
                        b'U' => PrimitiveKind::Char32,
                        _ => return Err(Error::InvalidPrimitiveType),
                    }
                }
                _ => return Err(Error::InvalidPrimitiveType),
            }
        };
        Ok(self.cache.intern(allocator, PrimitiveTypeNode::new(kind)))
    }

    fn demangle_custom_type(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<CustomType>> {
        mangled_name
            .try_consume_byte(b'?')
            .ok_or(Error::InvalidCustomType)?;

        let ctn = CustomTypeNode {
            quals: Qualifiers::Q_None,
            identifier: self.demangle_unqualified_type_name(allocator, mangled_name, true)?,
        };

        if mangled_name.try_consume_byte(b'@').is_some() {
            Ok(self.cache.intern(allocator, ctn))
        } else {
            Err(Error::InvalidCustomType)
        }
    }

    fn demangle_class_type(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<TagType>> {
        let f = mangled_name.try_consume().ok_or(Error::InvalidClassType)?;
        let tag = match f {
            b'T' => TagKind::Union,
            b'U' => TagKind::Struct,
            b'V' => TagKind::Class,
            b'W' => {
                if mangled_name.try_consume_byte(b'4').is_some() {
                    TagKind::Enum
                } else {
                    return Err(Error::InvalidClassType);
                }
            }
            _ => return Err(Error::InvalidClassType),
        };

        let tt = TagTypeNode {
            quals: Qualifiers::Q_None,
            qualified_name: self.demangle_fully_qualified_type_name(allocator, mangled_name)?,
            tag,
        };

        Ok(self.cache.intern(allocator, tt))
    }

    // <pointer-type> ::= E? <pointer-cvr-qualifiers> <ext-qualifiers> <type>
    //                       # the E is required for 64-bit non-static pointers
    fn demangle_pointer_type(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<PointerType>> {
        let (mut quals, affinity) = Self::demangle_pointer_cv_qualifiers(mangled_name)?;
        let pointee = if mangled_name.try_consume_byte(b'6').is_some() {
            self.demangle_function_type(allocator, mangled_name, false)?
                .into()
        } else {
            let ext_quals = Self::demangle_pointer_ext_qualifiers(mangled_name);
            quals |= ext_quals;
            self.demangle_type(allocator, mangled_name, QualifierMangleMode::Mangle)?
        };

        let pointer = PointerTypeNode {
            quals,
            affinity: Some(affinity),
            class_parent: None,
            pointee,
        };

        Ok(self.cache.intern(allocator, pointer))
    }

    fn demangle_member_pointer_type(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<PointerType>> {
        let (mut quals, affinity) = Self::demangle_pointer_cv_qualifiers(mangled_name)?;
        if affinity != PointerAffinity::Pointer {
            return Err(Error::InvalidMemberPointerType);
        }

        let ext_quals = Self::demangle_pointer_ext_qualifiers(mangled_name);
        quals |= ext_quals;

        // is_member_pointer() only returns true if there is at least one character
        // after the qualifiers.
        let (class_parent, pointee) = if mangled_name.try_consume_byte(b'8').is_some() {
            let class_parent = self.demangle_fully_qualified_type_name(allocator, mangled_name)?;
            let pointee = self.demangle_function_type(allocator, mangled_name, true)?;
            (Some(class_parent), pointee.into())
        } else {
            let (pointee_quals, is_member) = Self::demangle_qualifiers(mangled_name)?;
            if !is_member {
                return Err(Error::InvalidMemberPointerType);
            }
            let class_parent = self.demangle_fully_qualified_type_name(allocator, mangled_name)?;
            let pointee = self.demangle_type(allocator, mangled_name, QualifierMangleMode::Drop)?;
            pointee
                .resolve_mut(&mut self.cache)
                .set_quals(pointee_quals);
            (Some(class_parent), pointee)
        };

        let pointer = PointerTypeNode {
            quals,
            affinity: Some(affinity),
            class_parent,
            pointee,
        };

        Ok(self.cache.intern(allocator, pointer))
    }

    fn demangle_function_type(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
        has_this_quals: bool,
    ) -> Result<NodeHandle<FunctionSignature>> {
        let mut fty = FunctionSignatureNode::default();
        if has_this_quals {
            fty.quals = Self::demangle_pointer_ext_qualifiers(mangled_name);
            fty.ref_qualifier = Self::demangle_function_ref_qualifier(mangled_name);
            fty.quals |= Self::demangle_qualifiers(mangled_name)?.0;
        }

        // Fields that appear on both member and non-member functions.
        fty.call_convention = Self::demangle_calling_convention(mangled_name)?;

        // <return-type> ::= <type>
        //               ::= @ # structors (they have no declared return type)
        let is_structor = mangled_name.try_consume_byte(b'@').is_some();
        if !is_structor {
            fty.return_type =
                Some(self.demangle_type(allocator, mangled_name, QualifierMangleMode::Result)?);
        }

        fty.params =
            self.demangle_function_parameter_list(allocator, mangled_name, &mut fty.is_variadic)?;
        fty.is_noexcept = Self::demangle_throw_specification(mangled_name)?;

        Ok(self.cache.intern(allocator, fty))
    }

    fn demangle_array_type(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<ArrayType>> {
        mangled_name
            .try_consume_byte(b'Y')
            .ok_or(Error::InvalidArrayType)?;

        let (rank, is_negative) = Self::demangle_number(mangled_name)?;
        if is_negative || rank == 0 {
            return Err(Error::InvalidArrayType);
        }

        let dimensions = {
            let mut nodes = SmallVec::<[_; 8]>::new();
            for _ in 0..rank {
                let (value, is_negative) = Self::demangle_number(mangled_name)?;
                if is_negative {
                    return Err(Error::InvalidArrayType);
                }
                let n = IntegerLiteralNode { value, is_negative };
                nodes.push(self.cache.intern(allocator, n).into());
            }
            self.cache.intern(
                allocator,
                NodeArrayNode {
                    nodes: allocator.allocate_slice(&nodes),
                },
            )
        };

        let quals = if mangled_name.try_consume_str(b"$$C").is_some() {
            let (quals, is_member) = Self::demangle_qualifiers(mangled_name)?;
            if is_member {
                return Err(Error::InvalidArrayType);
            }
            quals
        } else {
            Qualifiers::Q_None
        };

        let element_type =
            self.demangle_type(allocator, mangled_name, QualifierMangleMode::Drop)?;
        let aty = ArrayTypeNode {
            quals,
            dimensions,
            element_type,
        };

        Ok(self.cache.intern(allocator, aty))
    }

    // Reads a function's parameters.
    fn demangle_function_parameter_list(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
        is_variadic: &mut bool,
    ) -> Result<Option<NodeHandle<NodeArray>>> {
        // Empty parameter list.
        if mangled_name.try_consume_byte(b'X').is_some() {
            return Ok(None);
        }

        let na = {
            let mut nodes = SmallVec::<[NodeHandle<INode>; 8]>::new();
            // TODO: llvm infinite loops here if mangled_name is ever empty... bug?
            // https://github.com/llvm/llvm-project/blob/bafda89a0944d947fc4b3b5663185e07a397ac30/llvm/lib/Demangle/MicrosoftDemangle.cpp#L2183-L2184
            while !mangled_name
                .first()
                .is_some_and(|&x| matches!(x, b'@' | b'Z'))
            {
                if mangled_name.is_empty() {
                    return Err(Error::InvalidFunctionParameterList);
                } else if let Some(n) = mangled_name.try_consume_byte_if(u8::is_ascii_digit) {
                    let index = usize::from(n - b'0');
                    if let Some(&param) = self.backrefs.function_params.get(index) {
                        nodes.push(param.into());
                    } else {
                        return Err(Error::InvalidFunctionParameterList);
                    }
                } else {
                    let old_len = mangled_name.len();
                    let tn =
                        self.demangle_type(allocator, mangled_name, QualifierMangleMode::Drop)?;
                    nodes.push(tn.into());

                    let chars_consumed = old_len - mangled_name.len();
                    match chars_consumed {
                        0 => return Err(Error::InvalidFunctionParameterList),
                        1 => (), // Single-letter types are ignored for backreferences because memorizing them doesn't save anything.
                        _ => {
                            _ = self.backrefs.function_params.try_push(tn);
                        }
                    };
                }
            }
            Some(self.cache.intern(
                allocator,
                NodeArrayNode {
                    nodes: allocator.allocate_slice(&nodes),
                },
            ))
        };

        // A non-empty parameter list is terminated by either 'Z' (variadic) parameter
        // list or '@' (non variadic). Careful not to consume "@Z", as in that case
        // the following Z could be a throw specifier.
        if mangled_name.try_consume_byte(b'@').is_some() {
            *is_variadic = false;
            Ok(na)
        } else if mangled_name.try_consume_byte(b'Z').is_some() {
            *is_variadic = true;
            Ok(na)
        } else {
            Err(Error::InvalidFunctionParameterList)
        }
    }

    fn demangle_template_parameter_list(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<NodeArray>> {
        // Template parameter lists don't participate in back-referencing.
        let mut nodes = SmallVec::<[NodeHandle<INode>; 8]>::new();

        while mangled_name.try_consume_byte(b'@').is_none() {
            if mangled_name.try_consume_str(b"$S").is_some()
                || mangled_name.try_consume_str(b"$$V").is_some()
                || mangled_name.try_consume_str(b"$$$V").is_some()
                || mangled_name.try_consume_str(b"$$Z").is_some()
            {
                // parameter pack separator
                continue;
            }
            if mangled_name.is_empty() {
                return Err(Error::InvalidTemplateParameterList);
            }

            if mangled_name.try_consume_str(b"$$Y").is_some() {
                // Template alias
                nodes.push(
                    self.demangle_fully_qualified_type_name(allocator, mangled_name)?
                        .into(),
                );
            } else if mangled_name.try_consume_str(b"$$B").is_some() {
                // Array
                nodes.push(
                    self.demangle_type(allocator, mangled_name, QualifierMangleMode::Drop)?
                        .into(),
                );
            } else if mangled_name.try_consume_str(b"$$C").is_some() {
                // Type has qualifiers.
                nodes.push(
                    self.demangle_type(allocator, mangled_name, QualifierMangleMode::Mangle)?
                        .into(),
                );
            } else if let Some(string) = mangled_name
                .try_consume_str(b"$1")
                .or_else(|| mangled_name.try_consume_str(b"$H"))
                .or_else(|| mangled_name.try_consume_str(b"$I"))
                .or_else(|| mangled_name.try_consume_str(b"$J"))
            {
                // Pointer to member
                let mut tprn = TemplateParameterReferenceNode {
                    symbol: if mangled_name.starts_with(b"?") {
                        let symbol = self.do_parse(allocator, mangled_name)?;
                        let identifier = symbol
                            .resolve(&self.cache)
                            .get_name()
                            .and_then(|x| {
                                x.resolve(&self.cache)
                                    .get_unqualified_identifier(&self.cache)
                            })
                            .ok_or(Error::InvalidTemplateParameterList)?;
                        self.memorize_identifier(allocator, identifier)?;
                        Some(symbol)
                    } else {
                        None
                    },
                    affinity: Some(PointerAffinity::Pointer),
                    is_member_pointer: true,
                    ..Default::default()
                };

                // 1 - single inheritance       <name>
                // H - multiple inheritance     <name> <number>
                // I - virtual inheritance      <name> <number> <number>
                // J - unspecified inheritance  <name> <number> <number> <number>
                let inheritance_specifier = string[1];
                let count = match inheritance_specifier {
                    b'1' => 0,
                    b'H' => 1,
                    b'I' => 2,
                    b'J' => 3,
                    _ => return Err(Error::InvalidTemplateParameterList),
                };
                for _ in 0..count {
                    let offset = Self::demangle_signed(mangled_name)?;
                    tprn.thunk_offsets
                        .try_push(offset)
                        .map_err(|_| Error::InvalidTemplateParameterList)?;
                }

                nodes.push(self.cache.intern(allocator, tprn).into());
            } else if mangled_name.starts_with(b"$E?") {
                mangled_name
                    .try_consume_str(b"$E")
                    .ok_or(Error::InvalidTemplateParameterList)?;
                // Reference to symbol
                let tprn = TemplateParameterReferenceNode {
                    symbol: Some(self.do_parse(allocator, mangled_name)?),
                    affinity: Some(PointerAffinity::Reference),
                    ..Default::default()
                };
                nodes.push(self.cache.intern(allocator, tprn).into());
            } else if let Some(string) = mangled_name
                .try_consume_str(b"$F")
                .or_else(|| mangled_name.try_consume_str(b"$G"))
            {
                // Data member pointer.
                let mut tprn = TemplateParameterReferenceNode {
                    is_member_pointer: true,
                    ..Default::default()
                };

                let inheritance_specifier = string[1];
                let count = match inheritance_specifier {
                    b'G' => 3,
                    b'F' => 2,
                    _ => return Err(Error::InvalidTemplateParameterList),
                };
                for _ in 0..count {
                    let offset = Self::demangle_signed(mangled_name)?;
                    tprn.thunk_offsets
                        .try_push(offset)
                        .map_err(|_| Error::InvalidTemplateParameterList)?;
                }

                nodes.push(self.cache.intern(allocator, tprn).into());
            } else if mangled_name.try_consume_str(b"$0").is_some() {
                // Integral non-type template parameter
                let (value, is_negative) = Self::demangle_number(mangled_name)?;
                let node = self
                    .cache
                    .intern(allocator, IntegerLiteralNode { value, is_negative });
                nodes.push(node.into());
            } else {
                let node =
                    self.demangle_type(allocator, mangled_name, QualifierMangleMode::Drop)?;
                nodes.push(node.into());
            }
        }

        // Template parameter lists cannot be variadic, so it can only be terminated
        // by @ (as opposed to 'Z' in the function parameter case).
        Ok(self.cache.intern(
            allocator,
            NodeArrayNode {
                nodes: allocator.allocate_slice(&nodes),
            },
        ))
    }

    // Sometimes numbers are encoded in mangled symbols. For example,
    // "int (*x)[20]" is a valid C type (x is a pointer to an array of
    // length 20), so we need some way to embed numbers as part of symbols.
    // This function parses it.
    //
    // <number>               ::= [?] <non-negative integer>
    //
    // <non-negative integer> ::= <decimal digit> # when 1 <= Number <= 10
    //                        ::= <hex digit>+ @  # when Number == 0 or >= 10
    //
    // <hex-digit>            ::= [A-P]           # A = 0, B = 1, ...
    fn demangle_number(mangled_name: &mut &'string BStr) -> Result<(u64, bool)> {
        let is_negative = mangled_name.try_consume_byte(b'?').is_some();
        let mut c = mangled_name.try_consume().ok_or(Error::InvalidNumber)?;
        if c.is_ascii_digit() {
            let number = c - b'0' + 1;
            Ok((number.into(), is_negative))
        } else {
            let mut number = 0u64;
            loop {
                match c {
                    // TODO: grammar says "<hex digit>+ @", but code says "<hex digit>* @"... bug?
                    // https://github.com/llvm/llvm-project/blob/bafda89a0944d947fc4b3b5663185e07a397ac30/llvm/lib/Demangle/MicrosoftDemangle.cpp#L932-L958
                    b'@' => break Ok((number, is_negative)),
                    c if c.is_rebased_ascii_hexdigit() => {
                        number = number.wrapping_shl(4) + u64::from(c - b'A');
                    }
                    _ => break Err(Error::InvalidNumber),
                }
                c = mangled_name.try_consume().ok_or(Error::InvalidNumber)?;
            }
        }
    }

    fn demangle_unsigned(mangled_name: &mut &'string BStr) -> Result<u64> {
        let (number, is_negative) = Self::demangle_number(mangled_name)?;
        if is_negative {
            Err(Error::InvalidUnsigned)
        } else {
            Ok(number)
        }
    }

    fn demangle_signed(mangled_name: &mut &'string BStr) -> Result<i64> {
        let (number, is_negative) = Self::demangle_number(mangled_name)?;
        number
            .try_into()
            .map(|x: i64| if is_negative { -x } else { x })
            .map_err(|_| Error::InvalidSigned)
    }

    // First 10 strings can be referenced by special BackReferences ?0, ?1, ..., ?9.
    // Memorize it.
    fn memorize_string(&mut self, allocator: &'alloc Allocator, s: &'string BStr) -> Result<()> {
        if self
            .backrefs
            .names
            .iter()
            .any(|x| x.resolve(&self.cache).name == s)
        {
            Ok(())
        } else {
            let name = self.cache.intern(
                allocator,
                NamedIdentifierNode {
                    name: s,
                    ..Default::default()
                },
            );
            self.backrefs
                .names
                .try_push(name)
                .map_err(|_| Error::TooManyBackRefs)
        }
    }

    fn memorize_identifier(
        &mut self,
        allocator: &'alloc Allocator,
        identifier: NodeHandle<IIdentifierNode>,
    ) -> Result<()> {
        // Render this class template name into a string buffer so that we can
        // memorize it for the purpose of back-referencing.
        let mut ob = OutputBuffer::new();
        identifier
            .resolve(&self.cache)
            .output(&self.cache, &mut ob, OutputFlags::default())?;
        self.memorize_string(allocator, allocator.allocate_slice(&ob).into())
    }

    // Parses a type name in the form of A@B@C@@ which represents C::B::A.
    fn demangle_fully_qualified_type_name(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<QualifiedName>> {
        let identifier = self.demangle_unqualified_type_name(allocator, mangled_name, true)?;
        self.demangle_name_scope_chain(allocator, mangled_name, identifier)
    }

    // Parses a symbol name in the form of A@B@C@@ which represents C::B::A.
    // Symbol names have slightly different rules regarding what can appear
    // so we separate out the implementations for flexibility.
    fn demangle_fully_qualified_symbol_name(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<QualifiedName>> {
        // This is the final component of a symbol name (i.e. the leftmost component
        // of a mangled name. Since the only possible template instantiation that
        // can appear in this context is a function template, and since those are
        // not saved for the purposes of name backreferences, only backref simple
        // names.

        let identifier = self.demangle_unqualified_symbol_name(
            allocator,
            mangled_name,
            NameBackrefBehavior::NBB_Simple,
        )?;
        let qn = self.demangle_name_scope_chain(allocator, mangled_name, identifier)?;

        if let Some(sin) = identifier.downcast::<StructorIdentifier>(&self.cache) {
            let class_node = {
                let components = &qn
                    .resolve(&self.cache)
                    .components
                    .resolve(&self.cache)
                    .nodes;
                components
                    .get(components.len().wrapping_sub(2))
                    .and_then(|x| x.downcast(&self.cache))
                    .ok_or(Error::InvalidFullyQualifiedSymbolName)?
            };
            sin.resolve_mut(&mut self.cache).class = Some(class_node);
        }

        Ok(qn)
    }

    fn demangle_unqualified_type_name(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
        memorize: bool,
    ) -> Result<NodeHandle<IIdentifierNode>> {
        if mangled_name.first().is_some_and(u8::is_ascii_digit) {
            self.demangle_back_ref_name(mangled_name).map(Into::into)
        } else if mangled_name.starts_with(b"?$") {
            self.demangle_template_instantiation_name(
                allocator,
                mangled_name,
                NameBackrefBehavior::NBB_Template,
            )
        } else {
            self.demangle_simple_name(allocator, mangled_name, memorize)
                .map(Into::into)
        }
    }

    fn demangle_unqualified_symbol_name(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
        nbb: NameBackrefBehavior,
    ) -> Result<NodeHandle<IIdentifierNode>> {
        if mangled_name.first().is_some_and(u8::is_ascii_digit) {
            self.demangle_back_ref_name(mangled_name).map(Into::into)
        } else if mangled_name.starts_with(b"?$") {
            self.demangle_template_instantiation_name(allocator, mangled_name, nbb)
        } else if mangled_name.starts_with(b"?") {
            self.demangle_function_identifier_code(allocator, mangled_name)
        } else {
            self.demangle_simple_name(allocator, mangled_name, nbb.is_simple())
                .map(Into::into)
        }
    }

    fn demangle_name_scope_chain(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
        unqualified_name: NodeHandle<IIdentifierNode>,
    ) -> Result<NodeHandle<QualifiedName>> {
        let mut nodes = SmallVec::<[_; 8]>::new();
        nodes.push(unqualified_name.into());
        loop {
            if mangled_name.try_consume_byte(b'@').is_some() {
                nodes.reverse();
                let components = self.cache.intern(
                    allocator,
                    NodeArrayNode {
                        nodes: allocator.allocate_slice(&nodes),
                    },
                );
                let qn = QualifiedNameNode { components };
                break Ok(self.cache.intern(allocator, qn));
            } else if mangled_name.is_empty() {
                break Err(Error::InvalidNameScopeChain);
            }
            let node = self.demangle_name_scope_piece(allocator, mangled_name)?;
            nodes.push(node.into());
        }
    }

    fn demangle_name_scope_piece(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<IIdentifierNode>> {
        if mangled_name.first().is_some_and(u8::is_ascii_digit) {
            self.demangle_back_ref_name(mangled_name).map(Into::into)
        } else if mangled_name.starts_with(b"?$") {
            self.demangle_template_instantiation_name(
                allocator,
                mangled_name,
                NameBackrefBehavior::NBB_Template,
            )
        } else if mangled_name.starts_with(b"?A") {
            self.demangle_anonymous_namespace_name(allocator, mangled_name)
                .map(Into::into)
        } else if mangled_name.starts_with_local_scope_pattern() {
            self.demangle_locally_scoped_name_piece(allocator, mangled_name)
                .map(Into::into)
        } else {
            self.demangle_simple_name(allocator, mangled_name, true)
                .map(Into::into)
        }
    }

    fn demangle_back_ref_name(
        &mut self,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<NamedIdentifier>> {
        let c = mangled_name
            .try_consume_byte_if(u8::is_ascii_digit)
            .ok_or(Error::InvalidBackRef)?;
        let i = c - b'0';
        let node = self
            .backrefs
            .names
            .get(i as usize)
            .ok_or(Error::InvalidBackRef)?;
        Ok(*node)
    }

    fn demangle_template_instantiation_name(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
        nbb: NameBackrefBehavior,
    ) -> Result<NodeHandle<IIdentifierNode>> {
        mangled_name
            .try_consume_str(b"?$")
            .ok_or(Error::InvalidTemplateInstantiationName)?;

        let outer_context = mem::take(&mut self.backrefs);
        let identifier = self.demangle_unqualified_symbol_name(
            allocator,
            mangled_name,
            NameBackrefBehavior::NBB_Simple,
        )?;
        {
            let template_params = self.demangle_template_parameter_list(allocator, mangled_name)?;
            identifier
                .resolve_mut(&mut self.cache)
                .set_template_params(template_params);
        }

        _ = mem::replace(&mut self.backrefs, outer_context);
        if nbb.is_template() {
            // NBB_Template is only set for types and non-leaf names ("a::" in "a::b").
            // Structors and conversion operators only makes sense in a leaf name, so
            // reject them in NBB_Template contexts.
            if matches!(
                identifier.resolve(&self.cache),
                IdentifierNode::ConversionOperatorIdentifier(_)
                    | IdentifierNode::StructorIdentifier(_)
            ) {
                return Err(Error::InvalidTemplateInstantiationName);
            }
            self.memorize_identifier(allocator, identifier)?;
        }

        Ok(identifier)
    }

    fn translate_intrinsic_function_code(
        ch: u8,
        group: FunctionIdentifierCodeGroup,
    ) -> Result<Option<IntrinsicFunctionKind>> {
        use crate::nodes::IntrinsicFunctionKind as IFK;
        if ch.is_ascii_digit() || ch.is_ascii_uppercase() {
            let i = if ch.is_ascii_digit() {
                ch - b'0'
            } else {
                ch - b'A' + 10
            };
            let lookup: &[Option<IntrinsicFunctionKind>; 36] = match group {
                FunctionIdentifierCodeGroup::Basic => &[
                    None,                        // ?0 # Foo::Foo()
                    None,                        // ?1 # Foo::~Foo()
                    Some(IFK::New),              // ?2 # operator new
                    Some(IFK::Delete),           // ?3 # operator delete
                    Some(IFK::Assign),           // ?4 # operator=
                    Some(IFK::RightShift),       // ?5 # operator>>
                    Some(IFK::LeftShift),        // ?6 # operator<<
                    Some(IFK::LogicalNot),       // ?7 # operator!
                    Some(IFK::Equals),           // ?8 # operator==
                    Some(IFK::NotEquals),        // ?9 # operator!=
                    Some(IFK::ArraySubscript),   // ?A # operator[]
                    None,                        // ?B # Foo::operator <type>()
                    Some(IFK::Pointer),          // ?C # operator->
                    Some(IFK::Dereference),      // ?D # operator*
                    Some(IFK::Increment),        // ?E # operator++
                    Some(IFK::Decrement),        // ?F # operator--
                    Some(IFK::Minus),            // ?G # operator-
                    Some(IFK::Plus),             // ?H # operator+
                    Some(IFK::BitwiseAnd),       // ?I # operator&
                    Some(IFK::MemberPointer),    // ?J # operator->*
                    Some(IFK::Divide),           // ?K # operator/
                    Some(IFK::Modulus),          // ?L # operator%
                    Some(IFK::LessThan),         // ?M operator<
                    Some(IFK::LessThanEqual),    // ?N operator<=
                    Some(IFK::GreaterThan),      // ?O operator>
                    Some(IFK::GreaterThanEqual), // ?P operator>=
                    Some(IFK::Comma),            // ?Q operator,
                    Some(IFK::Parens),           // ?R operator()
                    Some(IFK::BitwiseNot),       // ?S operator~
                    Some(IFK::BitwiseXor),       // ?T operator^
                    Some(IFK::BitwiseOr),        // ?U operator|
                    Some(IFK::LogicalAnd),       // ?V operator&&
                    Some(IFK::LogicalOr),        // ?W operator||
                    Some(IFK::TimesEqual),       // ?X operator*=
                    Some(IFK::PlusEqual),        // ?Y operator+=
                    Some(IFK::MinusEqual),       // ?Z operator-=
                ],
                FunctionIdentifierCodeGroup::Under => &[
                    Some(IFK::DivEqual),                // ?_0 operator/=
                    Some(IFK::ModEqual),                // ?_1 operator%=
                    Some(IFK::RshEqual),                // ?_2 operator>>=
                    Some(IFK::LshEqual),                // ?_3 operator<<=
                    Some(IFK::BitwiseAndEqual),         // ?_4 operator&=
                    Some(IFK::BitwiseOrEqual),          // ?_5 operator|=
                    Some(IFK::BitwiseXorEqual),         // ?_6 operator^=
                    None,                               // ?_7 # vftable
                    None,                               // ?_8 # vbtable
                    None,                               // ?_9 # vcall
                    None,                               // ?_A # typeof
                    None,                               // ?_B # local static guard
                    None,                               // ?_C # string literal
                    Some(IFK::VbaseDtor),               // ?_D # vbase destructor
                    Some(IFK::VecDelDtor),              // ?_E # vector deleting destructor
                    Some(IFK::DefaultCtorClosure),      // ?_F # default constructor closure
                    Some(IFK::ScalarDelDtor),           // ?_G # scalar deleting destructor
                    Some(IFK::VecCtorIter),             // ?_H # vector constructor iterator
                    Some(IFK::VecDtorIter),             // ?_I # vector destructor iterator
                    Some(IFK::VecVbaseCtorIter),        // ?_J # vector vbase constructor iterator
                    Some(IFK::VdispMap),                // ?_K # virtual displacement map
                    Some(IFK::EHVecCtorIter),           // ?_L # eh vector constructor iterator
                    Some(IFK::EHVecDtorIter),           // ?_M # eh vector destructor iterator
                    Some(IFK::EHVecVbaseCtorIter), // ?_N # eh vector vbase constructor iterator
                    Some(IFK::CopyCtorClosure),    // ?_O # copy constructor closure
                    None,                          // ?_P<name> # udt returning <name>
                    None,                          // ?_Q # <unknown>
                    None,                          // ?_R0 - ?_R4 # RTTI Codes
                    None,                          // ?_S # local vftable
                    Some(IFK::LocalVftableCtorClosure), // ?_T # local vftable constructor closure
                    Some(IFK::ArrayNew),           // ?_U operator new[]
                    Some(IFK::ArrayDelete),        // ?_V operator delete[]
                    None,                          // ?_W <unused>
                    None,                          // ?_X <unused>
                    None,                          // ?_Y <unused>
                    None,                          // ?_Z <unused>
                ],
                FunctionIdentifierCodeGroup::DoubleUnder => &[
                    None,                                  // ?__0 <unused>
                    None,                                  // ?__1 <unused>
                    None,                                  // ?__2 <unused>
                    None,                                  // ?__3 <unused>
                    None,                                  // ?__4 <unused>
                    None,                                  // ?__5 <unused>
                    None,                                  // ?__6 <unused>
                    None,                                  // ?__7 <unused>
                    None,                                  // ?__8 <unused>
                    None,                                  // ?__9 <unused>
                    Some(IFK::ManVectorCtorIter),          // ?__A managed vector ctor iterator
                    Some(IFK::ManVectorDtorIter),          // ?__B managed vector dtor iterator
                    Some(IFK::EHVectorCopyCtorIter),       // ?__C EH vector copy ctor iterator
                    Some(IFK::EHVectorVbaseCopyCtorIter),  // ?__D EH vector vbase copy ctor iter
                    None,                                  // ?__E dynamic initializer for `T'
                    None,                                  // ?__F dynamic atexit destructor for `T'
                    Some(IFK::VectorCopyCtorIter),         // ?__G vector copy constructor iter
                    Some(IFK::VectorVbaseCopyCtorIter),    // ?__H vector vbase copy ctor iter
                    Some(IFK::ManVectorVbaseCopyCtorIter), // ?__I managed vector vbase copy ctor iter
                    None,                                  // ?__J local static thread guard
                    None,                                  // ?__K operator ""_name
                    Some(IFK::CoAwait),                    // ?__L operator co_await
                    Some(IFK::Spaceship),                  // ?__M operator<=>
                    None,                                  // ?__N <unused>
                    None,                                  // ?__O <unused>
                    None,                                  // ?__P <unused>
                    None,                                  // ?__Q <unused>
                    None,                                  // ?__R <unused>
                    None,                                  // ?__S <unused>
                    None,                                  // ?__T <unused>
                    None,                                  // ?__U <unused>
                    None,                                  // ?__V <unused>
                    None,                                  // ?__W <unused>
                    None,                                  // ?__X <unused>
                    None,                                  // ?__Y <unused>
                    None,                                  // ?__Z <unused>
                ],
            };
            // SAFETY: the range contains 36 numbers,
            // and there are 10 ascii digits + 26 ascii uppercase characters
            Ok(unsafe { *lookup.get_unchecked(usize::from(i)) })
        } else {
            Err(Error::InvalidIntrinsicFunctionCode)
        }
    }

    fn demangle_function_identifier_code(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<IIdentifierNode>> {
        mangled_name
            .try_consume_byte(b'?')
            .ok_or(Error::InvalidFunctionIdentifierCode)?;
        if mangled_name.try_consume_str(b"__").is_some() {
            self.demangle_function_identifier_code_group(
                allocator,
                mangled_name,
                FunctionIdentifierCodeGroup::DoubleUnder,
            )
        } else if mangled_name.try_consume_byte(b'_').is_some() {
            self.demangle_function_identifier_code_group(
                allocator,
                mangled_name,
                FunctionIdentifierCodeGroup::Under,
            )
        } else {
            self.demangle_function_identifier_code_group(
                allocator,
                mangled_name,
                FunctionIdentifierCodeGroup::Basic,
            )
        }
    }

    fn demangle_function_identifier_code_group(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
        group: FunctionIdentifierCodeGroup,
    ) -> Result<NodeHandle<IIdentifierNode>> {
        let ch = mangled_name
            .try_consume()
            .ok_or(Error::InvalidFunctionIdentifierCode)?;
        match group {
            FunctionIdentifierCodeGroup::Basic if matches!(ch, b'0' | b'1') => Ok(self
                .demangle_structor_identifier(allocator, ch == b'1')
                .into()),
            FunctionIdentifierCodeGroup::Basic if ch == b'B' => Ok(self
                .demangle_conversion_operator_identifier(allocator)
                .into()),
            FunctionIdentifierCodeGroup::DoubleUnder if ch == b'K' => self
                .demangle_literal_operator_identifier(allocator, mangled_name)
                .map(Into::into),
            _ => {
                let operator = Self::translate_intrinsic_function_code(ch, group)?;
                let node = IntrinsicFunctionIdentifierNode::new(operator);
                Ok(self.cache.intern(allocator, node).into())
            }
        }
    }

    #[must_use]
    fn demangle_structor_identifier(
        &mut self,
        allocator: &'alloc Allocator,
        is_destructor: bool,
    ) -> NodeHandle<StructorIdentifier> {
        self.cache.intern(
            allocator,
            StructorIdentifierNode {
                is_destructor,
                ..Default::default()
            },
        )
    }

    #[must_use]
    fn demangle_conversion_operator_identifier(
        &mut self,
        allocator: &'alloc Allocator,
    ) -> NodeHandle<ConversionOperatorIdentifier> {
        self.cache
            .intern(allocator, ConversionOperatorIdentifierNode::default())
    }

    fn demangle_literal_operator_identifier(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<LiteralOperatorIdentifier>> {
        let name = self.demangle_simple_string(allocator, mangled_name, false)?;
        Ok(self.cache.intern(
            allocator,
            LiteralOperatorIdentifierNode {
                name,
                ..Default::default()
            },
        ))
    }

    fn demangle_special_intrinsic(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<Option<NodeHandle<ISymbolNode>>> {
        let sik = Self::consume_special_intrinsic_kind(mangled_name);
        if let Some(sik) = sik {
            let result = match sik {
                SpecialIntrinsicKind::StringLiteralSymbol => self
                    .demangle_string_literal(allocator, mangled_name)?
                    .into(),
                SpecialIntrinsicKind::Vftable
                | SpecialIntrinsicKind::Vbtable
                | SpecialIntrinsicKind::LocalVftable
                | SpecialIntrinsicKind::RttiCompleteObjLocator => self
                    .demangle_special_table_symbol_node(allocator, mangled_name, sik)?
                    .into(),
                SpecialIntrinsicKind::VcallThunk => self
                    .demangle_vcall_thunk_node(allocator, mangled_name)?
                    .into(),
                SpecialIntrinsicKind::LocalStaticGuard => self
                    .demangle_local_static_guard(allocator, mangled_name, false)?
                    .into(),
                SpecialIntrinsicKind::LocalStaticThreadGuard => self
                    .demangle_local_static_guard(allocator, mangled_name, true)?
                    .into(),
                SpecialIntrinsicKind::RttiTypeDescriptor => {
                    let t =
                        self.demangle_type(allocator, mangled_name, QualifierMangleMode::Result)?;
                    mangled_name
                        .try_consume_str(b"@8")
                        .ok_or(Error::InvalidSpecialIntrinsic)?;
                    if !mangled_name.is_empty() {
                        return Err(Error::InvalidSpecialIntrinsic);
                    }
                    let node = VariableSymbolNode::synthesize(
                        allocator,
                        &mut self.cache,
                        t,
                        b"`RTTI Type Descriptor'".into(),
                    );
                    self.cache.intern(allocator, node).into()
                }
                SpecialIntrinsicKind::RttiBaseClassArray => self
                    .demangle_untyped_variable(
                        allocator,
                        mangled_name,
                        b"`RTTI Base Class Array'".into(),
                    )?
                    .into(),
                SpecialIntrinsicKind::RttiClassHierarchyDescriptor => self
                    .demangle_untyped_variable(
                        allocator,
                        mangled_name,
                        b"`RTTI Class Hierarchy Descriptor'".into(),
                    )?
                    .into(),
                SpecialIntrinsicKind::RttiBaseClassDescriptor => self
                    .demangle_rtti_base_class_descriptor_node(allocator, mangled_name)?
                    .into(),
                SpecialIntrinsicKind::DynamicInitializer => self
                    .demangle_init_fini_stub(allocator, mangled_name, false)?
                    .into(),
                SpecialIntrinsicKind::DynamicAtexitDestructor => self
                    .demangle_init_fini_stub(allocator, mangled_name, true)?
                    .into(),
                SpecialIntrinsicKind::Typeof | SpecialIntrinsicKind::UdtReturning => {
                    // It's unclear which tools produces these manglings, so demangling
                    // support is not (yet?) implemented.
                    return Err(Error::InvalidSpecialIntrinsic);
                }
            };
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    fn demangle_special_table_symbol_node(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
        k: SpecialIntrinsicKind,
    ) -> Result<NodeHandle<SpecialTableSymbol>> {
        let intrinsic_name: &[u8] = match k {
            SpecialIntrinsicKind::Vftable => b"`vftable'",
            SpecialIntrinsicKind::Vbtable => b"`vbtable'",
            SpecialIntrinsicKind::LocalVftable => b"`local vftable'",
            SpecialIntrinsicKind::RttiCompleteObjLocator => b"`RTTI Complete Object Locator'",
            _ => return Err(Error::InvalidSpecialTableSymbolNode),
        };

        let ni = self.cache.intern(
            allocator,
            NamedIdentifierNode {
                name: intrinsic_name.into(),
                ..Default::default()
            },
        );
        let name = self.demangle_name_scope_chain(allocator, mangled_name, ni.into())?;

        mangled_name
            .try_consume_byte_if(|&x| matches!(x, b'6' | b'7'))
            .ok_or(Error::InvalidSpecialTableSymbolNode)?;

        let (quals, _) = Self::demangle_qualifiers(mangled_name)?;
        let target_name = if mangled_name.try_consume_byte(b'@').is_none() {
            Some(self.demangle_fully_qualified_type_name(allocator, mangled_name)?)
        } else {
            None
        };

        Ok(self.cache.intern(
            allocator,
            SpecialTableSymbolNode {
                name,
                target_name,
                quals,
            },
        ))
    }

    fn demangle_local_static_guard(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
        is_thread: bool,
    ) -> Result<NodeHandle<LocalStaticGuardVariable>> {
        let lsgi = self.cache.intern(
            allocator,
            LocalStaticGuardIdentifierNode {
                is_thread,
                ..Default::default()
            },
        );
        let name = self.demangle_name_scope_chain(allocator, mangled_name, lsgi.into())?;

        let is_visible = if mangled_name.try_consume_str(b"4IA").is_some() {
            false
        } else if mangled_name.try_consume_byte(b'5').is_some() {
            true
        } else {
            return Err(Error::InvalidLocalStaticGuard);
        };

        if !mangled_name.is_empty() {
            lsgi.resolve_mut(&mut self.cache).scope_index = Self::demangle_unsigned(mangled_name)?
                .try_into()
                .map_err(|_| Error::InvalidLocalStaticGuard)?;
        }

        Ok(self
            .cache
            .intern(allocator, LocalStaticGuardVariableNode { name, is_visible }))
    }

    fn demangle_untyped_variable(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
        variable_name: &'static BStr,
    ) -> Result<NodeHandle<VariableSymbol>> {
        let ni = self.cache.intern(
            allocator,
            NamedIdentifierNode {
                name: variable_name,
                ..Default::default()
            },
        );
        let name = Some(self.demangle_name_scope_chain(allocator, mangled_name, ni.into())?);
        if mangled_name.try_consume_byte(b'8').is_some() {
            Ok(self.cache.intern(
                allocator,
                VariableSymbolNode {
                    name,
                    ..Default::default()
                },
            ))
        } else {
            Err(Error::InvalidUntypedVariable)
        }
    }

    fn demangle_rtti_base_class_descriptor_node(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<VariableSymbol>> {
        let nv_offset = Self::demangle_unsigned(mangled_name)?
            .try_into()
            .map_err(|_| Error::InvalidRttiBaseClassDescriptorNode)?;
        let vbptr_offset = Self::demangle_signed(mangled_name)?
            .try_into()
            .map_err(|_| Error::InvalidRttiBaseClassDescriptorNode)?;
        let vbtable_offset = Self::demangle_unsigned(mangled_name)?
            .try_into()
            .map_err(|_| Error::InvalidRttiBaseClassDescriptorNode)?;
        let flags = Self::demangle_unsigned(mangled_name)?
            .try_into()
            .map_err(|_| Error::InvalidRttiBaseClassDescriptorNode)?;

        let rbcdn = self.cache.intern(
            allocator,
            RttiBaseClassDescriptorNode {
                nv_offset,
                vbptr_offset,
                vbtable_offset,
                flags,
                ..Default::default()
            },
        );
        let name = Some(self.demangle_name_scope_chain(allocator, mangled_name, rbcdn.into())?);
        mangled_name
            .try_consume_byte(b'8')
            .ok_or(Error::InvalidRttiBaseClassDescriptorNode)?;

        Ok(self.cache.intern(
            allocator,
            VariableSymbolNode {
                name,
                ..Default::default()
            },
        ))
    }

    fn demangle_init_fini_stub(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
        is_destructor: bool,
    ) -> Result<NodeHandle<FunctionSymbol>> {
        let is_known_static_data_member = mangled_name.try_consume_byte(b'?').is_some();
        let symbol = self.demangle_declarator(allocator, mangled_name)?;
        if let Some(variable) = symbol.downcast::<VariableSymbol>(&self.cache) {
            // Older versions of clang mangled this type of symbol incorrectly. They
            // would omit the leading ? and they would only emit a single @ at the end.
            // The correct mangling is a leading ? and 2 trailing @ signs. Handle
            // both cases.
            if is_known_static_data_member {
                mangled_name
                    .try_consume_str(b"@@")
                    .ok_or(Error::InvalidInitFiniStub)?;
            } else {
                mangled_name
                    .try_consume_byte(b'@')
                    .ok_or(Error::InvalidInitFiniStub)?;
            }

            let fsn = self.demangle_function_encoding(allocator, mangled_name)?;
            let dsin = self.cache.intern(
                allocator,
                DynamicStructorIdentifierNode {
                    template_params: TemplateParameters::default(),
                    identifier: variable.into(),
                    is_destructor,
                },
            );
            let name = {
                let x =
                    QualifiedNameNode::synthesize_from_id(allocator, &mut self.cache, dsin.into());
                self.cache.intern(allocator, x)
            };
            fsn.resolve_mut(&mut self.cache).name = Some(name);
            Ok(fsn)
        } else if let Some(fsn) = symbol.downcast::<FunctionSymbol>(&self.cache) {
            if is_known_static_data_member {
                // This was supposed to be a static data member, but we got a function.
                Err(Error::InvalidInitFiniStub)
            } else {
                let dstn = {
                    let fsn = fsn.resolve(&self.cache);
                    let x = DynamicStructorIdentifierNode {
                        template_params: TemplateParameters::default(),
                        identifier: fsn.name.ok_or(Error::InvalidInitFiniStub)?.into(),
                        is_destructor,
                    };
                    self.cache.intern(allocator, x)
                };
                let name = {
                    let x = QualifiedNameNode::synthesize_from_id(
                        allocator,
                        &mut self.cache,
                        dstn.into(),
                    );
                    self.cache.intern(allocator, x)
                };
                fsn.resolve_mut(&mut self.cache).name = Some(name);
                Ok(fsn)
            }
        } else {
            Err(Error::InvalidInitFiniStub)
        }
    }

    fn demangle_simple_name(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
        memorize: bool,
    ) -> Result<NodeHandle<NamedIdentifier>> {
        let name = self.demangle_simple_string(allocator, mangled_name, memorize)?;
        Ok(self.cache.intern(
            allocator,
            NamedIdentifierNode {
                name,
                ..Default::default()
            },
        ))
    }

    fn demangle_anonymous_namespace_name(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<NamedIdentifier>> {
        mangled_name
            .try_consume_str(b"?A")
            .ok_or(Error::InvalidAnonymousNamespaceName)?;
        let pos = mangled_name
            .find_byte(b'@')
            .ok_or(Error::InvalidAnonymousNamespaceName)?;
        let namespace_key = mangled_name
            .try_consume_n(pos)
            .ok_or(Error::InvalidAnonymousNamespaceName)?;
        self.memorize_string(allocator, namespace_key)?;
        mangled_name
            .try_consume_byte(b'@')
            .ok_or(Error::InvalidAnonymousNamespaceName)?;
        Ok(self.cache.intern(
            allocator,
            NamedIdentifierNode {
                name: b"`anonymous namespace'".into(),
                ..Default::default()
            },
        ))
    }

    fn demangle_locally_scoped_name_piece(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<NamedIdentifier>> {
        let mut identifier = NamedIdentifierNode::default();
        mangled_name
            .try_consume_byte(b'?')
            .ok_or(Error::InvalidLocallyScopedNamePiece)?;
        let (number, is_negative) = Self::demangle_number(mangled_name)?;
        if is_negative {
            return Err(Error::InvalidLocallyScopedNamePiece);
        }

        // One ? to terminate the number
        mangled_name
            .try_consume_byte(b'?')
            .ok_or(Error::InvalidLocallyScopedNamePiece)?;
        let scope = self.do_parse(allocator, mangled_name)?.resolve(&self.cache);

        // Render the parent symbol's name into a buffer.
        let mut ob = OutputBuffer::new();
        write!(ob, "`")?;
        scope.output(&self.cache, &mut ob, OutputFlags::default())?;
        write!(ob, "'::`{number}'")?;

        identifier.name = allocator.allocate_slice(&ob).into();
        Ok(self.cache.intern(allocator, identifier))
    }

    fn demangle_string_literal(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<EncodedStringLiteral>> {
        // Prefix indicating the beginning of a string literal
        mangled_name
            .try_consume_str(b"@_")
            .ok_or(Error::InvalidStringLiteral)?;

        let f = mangled_name
            .try_consume()
            .ok_or(Error::InvalidStringLiteral)?;
        let is_wchar_t = match f {
            b'1' => true,
            b'0' => false,
            _ => return Err(Error::InvalidStringLiteral),
        };

        // Encoded Length
        let (mut string_byte_len, is_negative) = Self::demangle_number(mangled_name)?;
        if is_negative || string_byte_len < if is_wchar_t { 2 } else { 1 } {
            return Err(Error::InvalidStringLiteral);
        }

        // CRC 32 (always 8 characters plus a terminator)
        let crc_end_pos = mangled_name
            .find_byte(b'@')
            .ok_or(Error::InvalidStringLiteral)?;
        mangled_name
            .try_consume_n(crc_end_pos + 1)
            .ok_or(Error::InvalidStringLiteral)?;
        if mangled_name.is_empty() {
            return Err(Error::InvalidStringLiteral);
        }

        let mut ob = OutputBuffer::new();
        let (char, is_truncated) = if is_wchar_t {
            let char = CharKind::Wchar;
            let is_truncated = string_byte_len > 64;

            while mangled_name.try_consume_byte(b'@').is_none() {
                if mangled_name.len() < 2 {
                    return Err(Error::InvalidStringLiteral);
                }
                let w = Self::demangle_wchar_literal(mangled_name)?;
                if string_byte_len != 2 || is_truncated {
                    Self::output_escaped_char(&mut ob, w.into())?;
                }
                string_byte_len -= 2;
            }

            (char, is_truncated)
        } else {
            // The max byte length is actually 32, but some compilers mangled strings
            // incorrectly, so we have to assume it can go higher.
            let mut string_bytes = ArrayVec::<u8, { 32 * 4 }>::new();
            while mangled_name.try_consume_byte(b'@').is_none() {
                if mangled_name.is_empty() {
                    return Err(Error::InvalidStringLiteral);
                }
                let char = Self::demangle_char_literal(mangled_name)?;
                string_bytes
                    .try_push(char)
                    .map_err(|_| Error::InvalidStringLiteral)?;
            }

            let is_truncated = string_byte_len > string_bytes.len() as u64;
            let char_bytes = Self::guess_char_byte_size(&string_bytes, string_byte_len)
                .ok_or(Error::InvalidStringLiteral)?;
            let char = match char_bytes {
                1 => CharKind::Char,
                2 => CharKind::Char16,
                4 => CharKind::Char32,
                _ => return Err(Error::InvalidStringLiteral),
            };

            let num_chars = string_bytes.len() / char_bytes;
            for char_index in 0..num_chars {
                let next_char = Self::decode_multi_byte_char(&string_bytes, char_index, char_bytes)
                    .ok_or(Error::InvalidStringLiteral)?;
                if char_index + 1 < num_chars || is_truncated {
                    Self::output_escaped_char(&mut ob, next_char)?;
                }
            }

            (char, is_truncated)
        };

        let result = EncodedStringLiteralNode {
            name: None,
            decoded_string: allocator.allocate_slice(&ob).into(),
            is_truncated,
            char,
        };
        Ok(self.cache.intern(allocator, result))
    }

    fn demangle_vcall_thunk_node(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
    ) -> Result<NodeHandle<FunctionSymbol>> {
        let vtin = self
            .cache
            .intern(allocator, VcallThunkIdentifierNode::default());
        let name = Some(self.demangle_name_scope_chain(allocator, mangled_name, vtin.into())?);

        mangled_name
            .try_consume_str(b"$B")
            .ok_or(Error::InvalidVcallThunkNode)?;
        let vtin = vtin.resolve_mut(&mut self.cache);
        vtin.offset_in_vtable = Self::demangle_unsigned(mangled_name)?;
        mangled_name
            .try_consume_byte(b'A')
            .ok_or(Error::InvalidVcallThunkNode)?;

        let signature = self
            .cache
            .intern(
                allocator,
                ThunkSignatureNode {
                    function_node: FunctionSignatureNode {
                        call_convention: Self::demangle_calling_convention(mangled_name)?,
                        function_class: FuncClass::FC_NoParameterList,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
            .into();
        Ok(self
            .cache
            .intern(allocator, FunctionSymbolNode { name, signature }))
    }

    // Returns mangled_name's prefix before the first '@', or an error if
    // mangled_name contains no '@' or the prefix has length 0.
    fn demangle_simple_string(
        &mut self,
        allocator: &'alloc Allocator,
        mangled_name: &mut &'string BStr,
        memorize: bool,
    ) -> Result<&'string BStr> {
        let pos = mangled_name
            .find_byte(b'@')
            .ok_or(Error::InvalidSimpleString)?;
        if pos == 0 {
            Err(Error::InvalidSimpleString)
        } else {
            let string = mangled_name
                .try_consume_n(pos)
                .ok_or(Error::InvalidSimpleString)?;
            mangled_name
                .try_consume_byte(b'@')
                .ok_or(Error::InvalidSimpleString)?;
            if memorize {
                self.memorize_string(allocator, string)?;
            }
            Ok(string)
        }
    }

    fn demangle_function_class(mangled_name: &mut &'string BStr) -> Result<FuncClass> {
        let f = mangled_name
            .try_consume()
            .ok_or(Error::InvalidFunctionClass)?;
        match f {
            b'9' => Ok(FuncClass::FC_ExternC | FuncClass::FC_NoParameterList),
            b'A' => Ok(FuncClass::FC_Private),
            b'B' => Ok(FuncClass::FC_Private | FuncClass::FC_Far),
            b'C' => Ok(FuncClass::FC_Private | FuncClass::FC_Static),
            b'D' => Ok(FuncClass::FC_Private | FuncClass::FC_Static | FuncClass::FC_Far),
            b'E' => Ok(FuncClass::FC_Private | FuncClass::FC_Virtual),
            b'F' => Ok(FuncClass::FC_Private | FuncClass::FC_Virtual | FuncClass::FC_Far),
            b'G' => Ok(FuncClass::FC_Private | FuncClass::FC_StaticThisAdjust),
            b'H' => Ok(FuncClass::FC_Private | FuncClass::FC_StaticThisAdjust | FuncClass::FC_Far),
            b'I' => Ok(FuncClass::FC_Protected),
            b'J' => Ok(FuncClass::FC_Protected | FuncClass::FC_Far),
            b'K' => Ok(FuncClass::FC_Protected | FuncClass::FC_Static),
            b'L' => Ok(FuncClass::FC_Protected | FuncClass::FC_Static | FuncClass::FC_Far),
            b'M' => Ok(FuncClass::FC_Protected | FuncClass::FC_Virtual),
            b'N' => Ok(FuncClass::FC_Protected | FuncClass::FC_Virtual | FuncClass::FC_Far),
            b'O' => Ok(FuncClass::FC_Protected
                | FuncClass::FC_Virtual
                | FuncClass::FC_StaticThisAdjust),
            b'P' => Ok(FuncClass::FC_Protected
                | FuncClass::FC_Virtual
                | FuncClass::FC_StaticThisAdjust
                | FuncClass::FC_Far),
            b'Q' => Ok(FuncClass::FC_Public),
            b'R' => Ok(FuncClass::FC_Public | FuncClass::FC_Far),
            b'S' => Ok(FuncClass::FC_Public | FuncClass::FC_Static),
            b'T' => Ok(FuncClass::FC_Public | FuncClass::FC_Static | FuncClass::FC_Far),
            b'U' => Ok(FuncClass::FC_Public | FuncClass::FC_Virtual),
            b'V' => Ok(FuncClass::FC_Public | FuncClass::FC_Virtual | FuncClass::FC_Far),
            b'W' => {
                Ok(FuncClass::FC_Public | FuncClass::FC_Virtual | FuncClass::FC_StaticThisAdjust)
            }
            b'X' => Ok(FuncClass::FC_Public
                | FuncClass::FC_Virtual
                | FuncClass::FC_StaticThisAdjust
                | FuncClass::FC_Far),
            b'Y' => Ok(FuncClass::FC_Global),
            b'Z' => Ok(FuncClass::FC_Global | FuncClass::FC_Far),
            b'$' => {
                let mut vflag = FuncClass::FC_VirtualThisAdjust;
                if mangled_name.try_consume_byte(b'R').is_some() {
                    vflag |= FuncClass::FC_VirtualThisAdjustEx;
                }
                let f = mangled_name
                    .try_consume()
                    .ok_or(Error::InvalidFunctionClass)?;
                match f {
                    b'0' => Ok(FuncClass::FC_Private | FuncClass::FC_Virtual | vflag),
                    b'1' => Ok(FuncClass::FC_Private
                        | FuncClass::FC_Virtual
                        | vflag
                        | FuncClass::FC_Far),
                    b'2' => Ok(FuncClass::FC_Protected | FuncClass::FC_Virtual | vflag),
                    b'3' => Ok(FuncClass::FC_Protected
                        | FuncClass::FC_Virtual
                        | vflag
                        | FuncClass::FC_Far),
                    b'4' => Ok(FuncClass::FC_Public | FuncClass::FC_Virtual | vflag),
                    b'5' => Ok(FuncClass::FC_Public
                        | FuncClass::FC_Virtual
                        | vflag
                        | FuncClass::FC_Far),
                    _ => Err(Error::InvalidFunctionClass),
                }
            }
            _ => Err(Error::InvalidFunctionClass),
        }
    }

    fn demangle_calling_convention(
        mangled_name: &mut &'string BStr,
    ) -> Result<Option<CallingConv>> {
        let f = mangled_name
            .try_consume()
            .ok_or(Error::InvalidCallingConvention)?;
        let cc = match f {
            b'A' | b'B' => Some(CallingConv::Cdecl),
            b'C' | b'D' => Some(CallingConv::Pascal),
            b'E' | b'F' => Some(CallingConv::Thiscall),
            b'G' | b'H' => Some(CallingConv::Stdcall),
            b'I' | b'J' => Some(CallingConv::Fastcall),
            b'M' | b'N' => Some(CallingConv::Clrcall),
            b'O' | b'P' => Some(CallingConv::Eabi),
            b'Q' => Some(CallingConv::Vectorcall),
            b'S' => Some(CallingConv::Swift),
            b'W' => Some(CallingConv::SwiftAsync),
            _ => None,
        };
        Ok(cc)
    }

    fn demangle_variable_storage_class(mangled_name: &mut &'string BStr) -> Result<StorageClass> {
        let f = mangled_name
            .try_consume()
            .ok_or(Error::InvalidVariableStorageClass)?;
        match f {
            b'0' => Ok(StorageClass::PrivateStatic),
            b'1' => Ok(StorageClass::ProtectedStatic),
            b'2' => Ok(StorageClass::PublicStatic),
            b'3' => Ok(StorageClass::Global),
            b'4' => Ok(StorageClass::FunctionLocalStatic),
            _ => Err(Error::InvalidVariableStorageClass),
        }
    }

    fn demangle_throw_specification(mangled_name: &mut &'string BStr) -> Result<bool> {
        if mangled_name.try_consume_str(b"_E").is_some() {
            Ok(true)
        } else if mangled_name.try_consume_byte(b'Z').is_some() {
            Ok(false)
        } else {
            Err(Error::InvalidThrowSpecification)
        }
    }

    fn demangle_wchar_literal(mangled_name: &mut &'string BStr) -> Result<u16> {
        let c1: u16 = Self::demangle_char_literal(mangled_name)?.into();
        let c2: u16 = Self::demangle_char_literal(mangled_name)?.into();
        Ok((c1 << 8) | c2)
    }

    fn demangle_char_literal(mangled_name: &mut &'string BStr) -> Result<u8> {
        let c = mangled_name
            .try_consume()
            .ok_or(Error::InvalidCharLiteral)?;
        match c {
            b'?' => {
                let c = mangled_name
                    .try_consume()
                    .ok_or(Error::InvalidCharLiteral)?;
                match c {
                    b'$' => {
                        let nibbles = mangled_name
                            .try_consume_n_fixed::<2>()
                            .ok_or(Error::InvalidCharLiteral)?;
                        let c1 = nibbles[0]
                            .try_convert_rebased_ascii_hexdigit_to_number()
                            .ok_or(Error::InvalidCharLiteral)?;
                        let c2 = nibbles[1]
                            .try_convert_rebased_ascii_hexdigit_to_number()
                            .ok_or(Error::InvalidCharLiteral)?;
                        Ok((c1 << 4) | c2)
                    }
                    c if c.is_ascii_digit() => {
                        const LOOKUP: [u8; 10] = [
                            b',', b'/', b'\\', b':', b'.', b' ', b'\n', b'\t', b'\'', b'-',
                        ];
                        let i = c - b'0';
                        // SAFETY: the range contains 10 numbers, and there are 10 ascii digits
                        let result = unsafe { LOOKUP.get_unchecked(i as usize) };
                        Ok(*result)
                    }
                    c if c.is_ascii_lowercase() => {
                        const LOOKUP: [u8; 26] = {
                            let mut result = [0xE1u8; 26];
                            let mut i = 0u8;
                            loop {
                                result[i as usize] += i;
                                i += 1;
                                if i as usize >= result.len() {
                                    break result;
                                }
                            }
                        };
                        let i = c - b'a';
                        // SAFETY: the range contains 26 numbers, and there are 26 ascii lowercase characters
                        let result = unsafe { LOOKUP.get_unchecked(i as usize) };
                        Ok(*result)
                    }
                    c if c.is_ascii_uppercase() => {
                        const LOOKUP: [u8; 26] = {
                            let mut result = [0xC1u8; 26];
                            let mut i = 0u8;
                            loop {
                                result[i as usize] += i;
                                i += 1;
                                if i as usize >= result.len() {
                                    break result;
                                }
                            }
                        };
                        let i = c - b'A';
                        // SAFETY: the range contains 26 numbers, and there are 26 ascii uppercase characters
                        let result = unsafe { LOOKUP.get_unchecked(i as usize) };
                        Ok(*result)
                    }
                    _ => Err(Error::InvalidCharLiteral),
                }
            }
            c => Ok(c),
        }
    }

    fn demangle_qualifiers(mangled_name: &mut &'string BStr) -> Result<(Qualifiers, bool)> {
        let f = mangled_name.try_consume().ok_or(Error::InvalidQualifiers)?;
        match f {
            // Member qualifiers
            b'Q' => Ok((Qualifiers::Q_None, true)),
            b'R' => Ok((Qualifiers::Q_Const, true)),
            b'S' => Ok((Qualifiers::Q_Volatile, true)),
            b'T' => Ok((Qualifiers::Q_Const | Qualifiers::Q_Volatile, true)),
            // Non-Member qualifiers
            b'A' => Ok((Qualifiers::Q_None, false)),
            b'B' => Ok((Qualifiers::Q_Const, false)),
            b'C' => Ok((Qualifiers::Q_Volatile, false)),
            b'D' => Ok((Qualifiers::Q_Const | Qualifiers::Q_Volatile, false)),
            _ => Err(Error::InvalidQualifiers),
        }
    }

    fn demangle_pointer_cv_qualifiers(
        mangled_name: &mut &'string BStr,
    ) -> Result<(Qualifiers, PointerAffinity)> {
        if mangled_name.try_consume_str(b"$$Q").is_some() {
            Ok((Qualifiers::Q_None, PointerAffinity::RValueReference))
        } else {
            let f = mangled_name
                .try_consume()
                .ok_or(Error::InvalidPointerCVQualifiers)?;
            match f {
                b'A' => Ok((Qualifiers::Q_None, PointerAffinity::Reference)),
                b'P' => Ok((Qualifiers::Q_None, PointerAffinity::Pointer)),
                b'Q' => Ok((Qualifiers::Q_Const, PointerAffinity::Pointer)),
                b'R' => Ok((Qualifiers::Q_Volatile, PointerAffinity::Pointer)),
                b'S' => Ok((
                    Qualifiers::Q_Const | Qualifiers::Q_Volatile,
                    PointerAffinity::Pointer,
                )),
                _ => Err(Error::InvalidPointerCVQualifiers),
            }
        }
    }

    #[must_use]
    fn demangle_function_ref_qualifier(
        mangled_name: &mut &'string BStr,
    ) -> Option<FunctionRefQualifier> {
        if mangled_name.try_consume_byte(b'G').is_some() {
            Some(FunctionRefQualifier::Reference)
        } else if mangled_name.try_consume_byte(b'H').is_some() {
            Some(FunctionRefQualifier::RValueReference)
        } else {
            None
        }
    }

    #[must_use]
    fn consume_special_intrinsic_kind(
        mangled_name: &mut &'string BStr,
    ) -> Option<SpecialIntrinsicKind> {
        if mangled_name.try_consume_str(b"?_7").is_some() {
            Some(SpecialIntrinsicKind::Vftable)
        } else if mangled_name.try_consume_str(b"?_8").is_some() {
            Some(SpecialIntrinsicKind::Vbtable)
        } else if mangled_name.try_consume_str(b"?_9").is_some() {
            Some(SpecialIntrinsicKind::VcallThunk)
        } else if mangled_name.try_consume_str(b"?_A").is_some() {
            Some(SpecialIntrinsicKind::Typeof)
        } else if mangled_name.try_consume_str(b"?_B").is_some() {
            Some(SpecialIntrinsicKind::LocalStaticGuard)
        } else if mangled_name.try_consume_str(b"?_C").is_some() {
            Some(SpecialIntrinsicKind::StringLiteralSymbol)
        } else if mangled_name.try_consume_str(b"?_P").is_some() {
            Some(SpecialIntrinsicKind::UdtReturning)
        } else if mangled_name.try_consume_str(b"?_R0").is_some() {
            Some(SpecialIntrinsicKind::RttiTypeDescriptor)
        } else if mangled_name.try_consume_str(b"?_R1").is_some() {
            Some(SpecialIntrinsicKind::RttiBaseClassDescriptor)
        } else if mangled_name.try_consume_str(b"?_R2").is_some() {
            Some(SpecialIntrinsicKind::RttiBaseClassArray)
        } else if mangled_name.try_consume_str(b"?_R3").is_some() {
            Some(SpecialIntrinsicKind::RttiClassHierarchyDescriptor)
        } else if mangled_name.try_consume_str(b"?_R4").is_some() {
            Some(SpecialIntrinsicKind::RttiCompleteObjLocator)
        } else if mangled_name.try_consume_str(b"?_S").is_some() {
            Some(SpecialIntrinsicKind::LocalVftable)
        } else if mangled_name.try_consume_str(b"?__E").is_some() {
            Some(SpecialIntrinsicKind::DynamicInitializer)
        } else if mangled_name.try_consume_str(b"?__F").is_some() {
            Some(SpecialIntrinsicKind::DynamicAtexitDestructor)
        } else if mangled_name.try_consume_str(b"?__J").is_some() {
            Some(SpecialIntrinsicKind::LocalStaticThreadGuard)
        } else {
            None
        }
    }

    fn output_escaped_char(ob: &mut OutputBuffer, c: u32) -> Result<()> {
        match c {
            0x00 => write!(ob, "\\0"),  // nul
            0x27 => write!(ob, "\\\'"), // single quote
            0x22 => write!(ob, "\\\""), // double quote
            0x5C => write!(ob, "\\\\"), // backslash
            0x07 => write!(ob, "\\a"),  // bell
            0x08 => write!(ob, "\\b"),  // backspace
            0x0C => write!(ob, "\\f"),  // form feed
            0x0A => write!(ob, "\\n"),  // new line
            0x0D => write!(ob, "\\r"),  // carriage return
            0x09 => write!(ob, "\\t"),  // tab
            0x0B => write!(ob, "\\v"),  // vertical tab
            _ if (0x20..=0x7E).contains(&c) => {
                // SAFETY: we just verified c is printable ascii
                let c = unsafe { char::from_u32_unchecked(c) };
                write!(ob, "{c}")
            }
            _ => write!(ob, "\\x{c:02X}"),
        }?;
        Ok(())
    }

    // A mangled (non-wide) string literal stores the total length of the string it
    // refers to (passed in num_bytes), and it contains up to 32 bytes of actual text
    // (passed in string_bytes).
    fn guess_char_byte_size(string_bytes: &[u8], num_bytes: u64) -> Option<usize> {
        if num_bytes == 0 {
            None
        } else if num_bytes % 2 == 1 {
            // If the number of bytes is odd, this is guaranteed to be a char string.
            Some(1)
        } else if num_bytes < 32 {
            // All strings can encode at most 32 bytes of data. If it's less than that,
            // then we encoded the entire string. In this case we check for a 1-byte,
            // 2-byte, or 4-byte null terminator.
            let trailing_nulls = string_bytes.iter().rev().take_while(|&&x| x == 0).count();
            if trailing_nulls >= 4 && num_bytes % 4 == 0 {
                Some(4)
            } else if trailing_nulls >= 2 {
                Some(2)
            } else {
                Some(1)
            }
        } else {
            // The whole string was not able to be encoded. Try to look at embedded null
            // terminators to guess. The heuristic is that we count all embedded null
            // terminators. If more than 2/3 are null, it's a char32. If more than 1/3
            // are null, it's a char16. Otherwise it's a char8. This obviously isn't
            // perfect and is biased towards languages that have ascii alphabets, but this
            // was always going to be best effort since the encoding is lossy.
            let embedded_nulls: usize = string_bytes.iter().map(|&x| usize::from(x == 0)).sum();
            if embedded_nulls >= 2 * string_bytes.len() / 3 && num_bytes % 4 == 0 {
                Some(4)
            } else if embedded_nulls >= string_bytes.len() / 3 {
                Some(2)
            } else {
                Some(1)
            }
        }
    }

    fn decode_multi_byte_char(
        string_bytes: &[u8],
        char_index: usize,
        char_bytes: usize,
    ) -> Option<u32> {
        if char_bytes == 1 || char_bytes == 2 || char_bytes == 4 {
            let offset = char_index * char_bytes;
            if offset > string_bytes.len() {
                return None;
            }

            let string_bytes = &string_bytes[offset..];
            if string_bytes.len() < char_bytes {
                return None;
            }

            let mut result = 0;
            for (i, &c) in string_bytes.iter().enumerate().take(char_bytes) {
                let c: u32 = c.into();
                result |= c << (8 * i);
            }

            Some(result)
        } else {
            None
        }
    }
}
