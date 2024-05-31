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
        NodeArray,
        NodeCache,
        NodeHandle,
        QualifiedName,
        VariableSymbol,
    },
    nodes::{
        CallingConv,
        CharKind,
        Downcast as _,
        FuncClass,
        FunctionRefQualifier,
        IIdentifierNode,
        INode,
        ISignatureNode,
        ISymbolNode,
        ITypeNode,
        IntrinsicFunctionKind,
        PointerAffinity,
        PrimitiveKind,
        Qualifiers,
        Result,
        StorageClass,
        TagKind,
        TypeNode,
        WriteableNode,
        WriteableTypeNode,
    },
    safe_write,
    Allocator,
    OutputFlags,
    Writer,
};
use arrayvec::ArrayVec;
use bstr::BStr;
use std::{
    mem::ManuallyDrop,
    ops::{
        Deref,
        DerefMut,
    },
};

#[derive(Clone, Copy)]
pub(crate) struct PrimitiveTypeNode {
    pub(crate) quals: Qualifiers,
    pub(crate) prim_kind: PrimitiveKind,
}

impl PrimitiveTypeNode {
    #[must_use]
    pub(crate) fn new(prim_kind: PrimitiveKind) -> Self {
        Self {
            quals: Qualifiers::Q_None,
            prim_kind,
        }
    }
}

impl WriteableNode for PrimitiveTypeNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        self.output_pair(cache, ob, flags)
    }
}

impl WriteableTypeNode for PrimitiveTypeNode {
    fn output_pre<W: Writer>(&self, _: &NodeCache, ob: &mut W, _: OutputFlags) -> Result<()> {
        let kind = match self.prim_kind {
            PrimitiveKind::Void => "void",
            PrimitiveKind::Bool => "bool",
            PrimitiveKind::Char => "char",
            PrimitiveKind::Schar => "signed char",
            PrimitiveKind::Uchar => "unsigned char",
            PrimitiveKind::Char8 => "char8_t",
            PrimitiveKind::Char16 => "char16_t",
            PrimitiveKind::Char32 => "char32_t",
            PrimitiveKind::Short => "short",
            PrimitiveKind::Ushort => "unsigned short",
            PrimitiveKind::Int => "int",
            PrimitiveKind::Uint => "unsigned int",
            PrimitiveKind::Long => "long",
            PrimitiveKind::Ulong => "unsigned long",
            PrimitiveKind::Int64 => "__int64",
            PrimitiveKind::Uint64 => "unsigned __int64",
            PrimitiveKind::Wchar => "wchar_t",
            PrimitiveKind::Float => "float",
            PrimitiveKind::Double => "double",
            PrimitiveKind::Ldouble => "long double",
            PrimitiveKind::Nullptr => "std::nullptr_t",
        };
        safe_write!(ob, "{kind}")?;
        self.quals.output(ob, true, false)
    }

    fn output_post<W: Writer>(&self, _: &NodeCache, _: &mut W, _: OutputFlags) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub(crate) struct FunctionSignatureNode {
    pub(crate) quals: Qualifiers,

    // Valid if this FunctionTypeNode is the Pointee of a PointerType or
    // MemberPointerType.
    #[allow(unused)]
    pub(crate) affinity: Option<PointerAffinity>,

    // The function's calling convention.
    pub(crate) call_convention: Option<CallingConv>,

    // Function flags (gloabl, public, etc)
    pub(crate) function_class: FuncClass,

    pub(crate) ref_qualifier: Option<FunctionRefQualifier>,

    // The return type of the function.
    pub(crate) return_type: Option<NodeHandle<ITypeNode>>,

    // True if this is a C-style ... varargs function.
    pub(crate) is_variadic: bool,

    // Function parameters
    pub(crate) params: Option<NodeHandle<NodeArray>>,

    // True if the function type is noexcept.
    pub(crate) is_noexcept: bool,
}

impl Default for FunctionSignatureNode {
    fn default() -> Self {
        Self {
            quals: Qualifiers::Q_None,
            affinity: None,
            call_convention: None,
            function_class: FuncClass::FC_Global,
            ref_qualifier: None,
            return_type: None,
            is_variadic: false,
            params: None,
            is_noexcept: false,
        }
    }
}

impl WriteableNode for FunctionSignatureNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        self.output_pair(cache, ob, flags)
    }
}

impl WriteableTypeNode for FunctionSignatureNode {
    fn output_pre<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
    ) -> Result<()> {
        if !flags.no_access_specifier() {
            if self.function_class.is_public() {
                safe_write!(ob, "public: ")?;
            }
            if self.function_class.is_protected() {
                safe_write!(ob, "protected: ")?;
            }
            if self.function_class.is_private() {
                safe_write!(ob, "private: ")?;
            }
        }

        if !flags.no_member_type() {
            if !self.function_class.is_global() && self.function_class.is_static() {
                safe_write!(ob, "static ")?;
            }
            if self.function_class.is_virtual() {
                safe_write!(ob, "virtual ")?;
            }
            if self.function_class.is_extern_c() {
                safe_write!(ob, "extern \"C\" ")?;
            }
        }

        if !flags.no_return_type() {
            if let Some(return_type) = self.return_type.map(|x| x.resolve(cache)) {
                return_type.output_pre(cache, ob, flags)?;
                safe_write!(ob, " ")?;
            }
        }

        if !flags.no_calling_convention() {
            if let Some(call_convention) = self.call_convention {
                call_convention.output(ob)?;
            }
        }

        Ok(())
    }

    fn output_post<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
    ) -> Result<()> {
        if !self.function_class.no_parameter_list() {
            safe_write!(ob, "(")?;
            if let Some(params) = self.params.map(|x| x.resolve(cache)) {
                params.output(cache, ob, flags)?;
            } else {
                safe_write!(ob, "void")?;
            }

            if self.is_variadic {
                if ob.last().is_some_and(|x| *x != b'(') {
                    safe_write!(ob, ", ")?;
                }
                safe_write!(ob, "...")?;
            }
            safe_write!(ob, ")")?;
        }

        if self.quals.is_const() {
            safe_write!(ob, " const")?;
        }
        if self.quals.is_volatile() {
            safe_write!(ob, " volatile")?;
        }
        if self.quals.is_restrict() {
            safe_write!(ob, " __restrict")?;
        }
        if self.quals.is_unaligned() {
            safe_write!(ob, " __unaligned")?;
        }

        if self.is_noexcept {
            safe_write!(ob, " noexcept")?;
        }

        match self.ref_qualifier {
            Some(FunctionRefQualifier::Reference) => safe_write!(ob, " &")?,
            Some(FunctionRefQualifier::RValueReference) => safe_write!(ob, " &&")?,
            _ => (),
        }

        if !flags.no_return_type() {
            if let Some(return_type) = self.return_type.map(|x| x.resolve(cache)) {
                return_type.output_post(cache, ob, flags)?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct ThisAdjustor {
    pub(crate) static_offset: u32,
    pub(crate) vbptr_offset: i32,
    pub(crate) vboffset_offset: i32,
    pub(crate) vtor_disp_offset: i32,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct ThunkSignatureNode {
    pub(crate) function_node: FunctionSignatureNode,
    pub(crate) this_adjust: ThisAdjustor,
}

impl Deref for ThunkSignatureNode {
    type Target = FunctionSignatureNode;

    fn deref(&self) -> &Self::Target {
        &self.function_node
    }
}

impl DerefMut for ThunkSignatureNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.function_node
    }
}

impl WriteableNode for ThunkSignatureNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        self.output_pair(cache, ob, flags)
    }
}

impl WriteableTypeNode for ThunkSignatureNode {
    fn output_pre<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
    ) -> Result<()> {
        safe_write!(ob, "[thunk]: ")?;
        self.function_node.output_pre(cache, ob, flags)
    }

    fn output_post<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
    ) -> Result<()> {
        let ThisAdjustor {
            static_offset,
            vbptr_offset,
            vboffset_offset,
            vtor_disp_offset,
        } = self.this_adjust;

        if self.function_class.has_static_this_adjust() {
            safe_write!(ob, "`adjustor{{{static_offset}}}'")?;
        } else if self.function_class.has_virtual_this_adjust() {
            if self.function_class.has_virtual_this_adjust_ex() {
                safe_write!(ob, "`vtordispex{{{vbptr_offset}, {vboffset_offset}, {vtor_disp_offset}, {static_offset}}}'")?;
            } else {
                safe_write!(ob, "`vtordisp{{{vtor_disp_offset}, {static_offset}}}'")?;
            }
        }

        self.function_node.output_post(cache, ob, flags)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct PointerTypeNode {
    pub(crate) quals: Qualifiers,

    // Is this a pointer, reference, or rvalue-reference?
    pub(crate) affinity: Option<PointerAffinity>,

    // If this is a member pointer, this is the class that the member is in.
    pub(crate) class_parent: Option<NodeHandle<QualifiedName>>,

    // Represents a type X in "a pointer to X", "a reference to X", or
    // "rvalue-reference to X"
    pub(crate) pointee: NodeHandle<ITypeNode>,
}

impl WriteableNode for PointerTypeNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        self.output_pair(cache, ob, flags)
    }
}

impl WriteableTypeNode for PointerTypeNode {
    fn output_pre<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
    ) -> Result<()> {
        let pointee = self.pointee.resolve(cache);
        if let TypeNode::Signature(sig) = pointee {
            // If this is a pointer to a function, don't output the calling convention.
            // It needs to go inside the parentheses.
            sig.output_pre(cache, ob, OutputFlags::NO_CALLING_CONVENTION)?;
        } else {
            pointee.output_pre(cache, ob, flags)?;
        }

        super::output_space_if_necessary(ob)?;

        if self.quals.is_unaligned() {
            safe_write!(ob, "__unaligned ")?;
        }

        match pointee {
            TypeNode::ArrayType(_) => safe_write!(ob, "(")?,
            TypeNode::Signature(sig) => {
                safe_write!(ob, "(")?;
                if let Some(call_convention) = sig.as_node().call_convention {
                    call_convention.output(ob)?;
                }
                safe_write!(ob, " ")?;
            }
            _ => (),
        }

        if let Some(class_parent) = self.class_parent.map(|x| x.resolve(cache)) {
            class_parent.output(cache, ob, flags)?;
            safe_write!(ob, "::")?;
        }

        let affinity = self
            .affinity
            .expect("pointer should have an affinity by this point");
        match affinity {
            PointerAffinity::Pointer => safe_write!(ob, "*")?,
            PointerAffinity::Reference => safe_write!(ob, "&")?,
            PointerAffinity::RValueReference => safe_write!(ob, "&&")?,
        }

        self.quals.output(ob, false, false)
    }

    fn output_post<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
    ) -> Result<()> {
        let pointee = self.pointee.resolve(cache);
        if matches!(pointee, TypeNode::ArrayType(_) | TypeNode::Signature(_)) {
            safe_write!(ob, ")")?;
        }
        pointee.output_post(cache, ob, flags)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct TagTypeNode {
    pub(crate) quals: Qualifiers,
    pub(crate) qualified_name: NodeHandle<QualifiedName>,
    pub(crate) tag: TagKind,
}

impl WriteableNode for TagTypeNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        self.output_pair(cache, ob, flags)
    }
}

impl WriteableTypeNode for TagTypeNode {
    fn output_pre<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
    ) -> Result<()> {
        if !flags.no_tag_specifier() {
            let tag = match self.tag {
                TagKind::Class => "class",
                TagKind::Struct => "struct",
                TagKind::Union => "union",
                TagKind::Enum => "enum",
            };
            safe_write!(ob, "{tag} ")?;
        }

        self.qualified_name
            .resolve(cache)
            .output(cache, ob, flags)?;

        self.quals.output(ob, true, false)
    }

    fn output_post<W: Writer>(&self, _: &NodeCache, _: &mut W, _: OutputFlags) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub(crate) struct ArrayTypeNode {
    pub(crate) quals: Qualifiers,

    // A list of array dimensions.  e.g. [3,4,5] in `int Foo[3][4][5]`
    pub(crate) dimensions: NodeHandle<NodeArray>,

    // The type of array element.
    pub(crate) element_type: NodeHandle<ITypeNode>,
}

impl ArrayTypeNode {
    fn output_one_dimension<W: Writer>(
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
        node: NodeHandle<INode>,
    ) -> Result<()> {
        let node = node.resolve(cache);
        let iln: &IntegerLiteralNode = node.downcast().expect(
            "the dimensions of an ArrayTypeNode should always be instances of IntegerLiteralNode",
        );

        if iln.value != 0 {
            iln.output(cache, ob, flags)
        } else {
            Ok(())
        }
    }

    fn output_dimensions_impl<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
    ) -> Result<()> {
        let dimensions = self.dimensions.resolve(cache);
        if let Some((&first, rest)) = dimensions.nodes.split_first() {
            Self::output_one_dimension(cache, ob, flags, first)?;
            for &handle in rest {
                safe_write!(ob, "][")?;
                Self::output_one_dimension(cache, ob, flags, handle)?;
            }
        }

        Ok(())
    }
}

impl WriteableNode for ArrayTypeNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        self.output_pair(cache, ob, flags)
    }
}

impl WriteableTypeNode for ArrayTypeNode {
    fn output_pre<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
    ) -> Result<()> {
        self.element_type
            .resolve(cache)
            .output_pre(cache, ob, flags)?;
        self.quals.output(ob, true, false)
    }

    fn output_post<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
    ) -> Result<()> {
        safe_write!(ob, "[")?;
        self.output_dimensions_impl(cache, ob, flags)?;
        safe_write!(ob, "]")?;
        self.element_type
            .resolve(cache)
            .output_post(cache, ob, flags)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct CustomTypeNode {
    pub(crate) quals: Qualifiers,
    pub(crate) identifier: NodeHandle<IIdentifierNode>,
}

impl WriteableNode for CustomTypeNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        self.output_pair(cache, ob, flags)
    }
}

impl WriteableTypeNode for CustomTypeNode {
    fn output_pre<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
    ) -> Result<()> {
        self.identifier.resolve(cache).output(cache, ob, flags)
    }

    fn output_post<W: Writer>(&self, _: &NodeCache, _: &mut W, _: OutputFlags) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct TemplateParameters(pub(crate) Option<NodeHandle<NodeArray>>);

impl TemplateParameters {
    fn output<W: Writer>(self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        if let Some(this) = self.map(|x| x.resolve(cache)) {
            safe_write!(ob, "<")?;
            this.output(cache, ob, flags)?;
            safe_write!(ob, ">")?;
        }
        Ok(())
    }
}

impl Deref for TemplateParameters {
    type Target = Option<NodeHandle<NodeArray>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TemplateParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct VcallThunkIdentifierNode {
    pub(crate) template_params: TemplateParameters,
    pub(crate) offset_in_vtable: u64,
}

impl WriteableNode for VcallThunkIdentifierNode {
    fn output<W: Writer>(&self, _: &NodeCache, ob: &mut W, _: OutputFlags) -> Result<()> {
        safe_write!(ob, "`vcall'{{{}, {{flat}}}}", self.offset_in_vtable)
    }
}

#[derive(Clone, Copy)]
pub(crate) enum DynamicStructorIdentifier {
    Variable(NodeHandle<VariableSymbol>),
    Name(NodeHandle<QualifiedName>),
}

impl From<NodeHandle<VariableSymbol>> for DynamicStructorIdentifier {
    fn from(value: NodeHandle<VariableSymbol>) -> Self {
        Self::Variable(value)
    }
}

impl From<NodeHandle<QualifiedName>> for DynamicStructorIdentifier {
    fn from(value: NodeHandle<QualifiedName>) -> Self {
        Self::Name(value)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct DynamicStructorIdentifierNode {
    pub(crate) template_params: TemplateParameters,
    pub(crate) identifier: DynamicStructorIdentifier,
    pub(crate) is_destructor: bool,
}

impl WriteableNode for DynamicStructorIdentifierNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        if self.is_destructor {
            safe_write!(ob, "`dynamic atexit destructor for ")?;
        } else {
            safe_write!(ob, "`dynamic initializer for ")?;
        }

        match self.identifier {
            DynamicStructorIdentifier::Variable(variable) => {
                safe_write!(ob, "`")?;
                variable.resolve(cache).output(cache, ob, flags)?;
                safe_write!(ob, "''")?;
            }
            DynamicStructorIdentifier::Name(name) => {
                safe_write!(ob, "'")?;
                name.resolve(cache).output(cache, ob, flags)?;
                safe_write!(ob, "''")?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Default)]
pub(crate) struct NamedIdentifierNode<'alloc> {
    pub(crate) template_params: TemplateParameters,
    pub(crate) name: &'alloc BStr,
}

impl<'alloc> WriteableNode for NamedIdentifierNode<'alloc> {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        safe_write!(ob, "{}", self.name)?;
        self.template_params.output(cache, ob, flags)
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct IntrinsicFunctionIdentifierNode {
    pub(crate) template_params: TemplateParameters,
    pub(crate) operator: Option<IntrinsicFunctionKind>,
}

impl IntrinsicFunctionIdentifierNode {
    #[must_use]
    pub(crate) fn new(operator: Option<IntrinsicFunctionKind>) -> Self {
        Self {
            operator,
            ..Default::default()
        }
    }
}

impl WriteableNode for IntrinsicFunctionIdentifierNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        if let Some(operator) = self.operator {
            let op = match operator {
                IntrinsicFunctionKind::New => "operator new",
                IntrinsicFunctionKind::Delete => "operator delete",
                IntrinsicFunctionKind::Assign => "operator=",
                IntrinsicFunctionKind::RightShift => "operator>>",
                IntrinsicFunctionKind::LeftShift => "operator<<",
                IntrinsicFunctionKind::LogicalNot => "operator!",
                IntrinsicFunctionKind::Equals => "operator==",
                IntrinsicFunctionKind::NotEquals => "operator!=",
                IntrinsicFunctionKind::ArraySubscript => "operator[]",
                IntrinsicFunctionKind::Pointer => "operator->",
                IntrinsicFunctionKind::Increment => "operator++",
                IntrinsicFunctionKind::Decrement => "operator--",
                IntrinsicFunctionKind::Minus => "operator-",
                IntrinsicFunctionKind::Plus => "operator+",
                IntrinsicFunctionKind::Dereference => "operator*",
                IntrinsicFunctionKind::BitwiseAnd => "operator&",
                IntrinsicFunctionKind::MemberPointer => "operator->*",
                IntrinsicFunctionKind::Divide => "operator/",
                IntrinsicFunctionKind::Modulus => "operator%",
                IntrinsicFunctionKind::LessThan => "operator<",
                IntrinsicFunctionKind::LessThanEqual => "operator<=",
                IntrinsicFunctionKind::GreaterThan => "operator>",
                IntrinsicFunctionKind::GreaterThanEqual => "operator>=",
                IntrinsicFunctionKind::Comma => "operator,",
                IntrinsicFunctionKind::Parens => "operator()",
                IntrinsicFunctionKind::BitwiseNot => "operator~",
                IntrinsicFunctionKind::BitwiseXor => "operator^",
                IntrinsicFunctionKind::BitwiseOr => "operator|",
                IntrinsicFunctionKind::LogicalAnd => "operator&&",
                IntrinsicFunctionKind::LogicalOr => "operator||",
                IntrinsicFunctionKind::TimesEqual => "operator*=",
                IntrinsicFunctionKind::PlusEqual => "operator+=",
                IntrinsicFunctionKind::MinusEqual => "operator-=",
                IntrinsicFunctionKind::DivEqual => "operator/=",
                IntrinsicFunctionKind::ModEqual => "operator%=",
                IntrinsicFunctionKind::RshEqual => "operator>>=",
                IntrinsicFunctionKind::LshEqual => "operator<<=",
                IntrinsicFunctionKind::BitwiseAndEqual => "operator&=",
                IntrinsicFunctionKind::BitwiseOrEqual => "operator|=",
                IntrinsicFunctionKind::BitwiseXorEqual => "operator^=",
                IntrinsicFunctionKind::VbaseDtor => "`vbase dtor'",
                IntrinsicFunctionKind::VecDelDtor => "`vector deleting dtor'",
                IntrinsicFunctionKind::DefaultCtorClosure => "`default ctor closure'",
                IntrinsicFunctionKind::ScalarDelDtor => "`scalar deleting dtor'",
                IntrinsicFunctionKind::VecCtorIter => "`vector ctor iterator'",
                IntrinsicFunctionKind::VecDtorIter => "`vector dtor iterator'",
                IntrinsicFunctionKind::VecVbaseCtorIter => "`vector vbase ctor iterator'",
                IntrinsicFunctionKind::VdispMap => "`virtual displacement map'",
                IntrinsicFunctionKind::EHVecCtorIter => "`eh vector ctor iterator'",
                IntrinsicFunctionKind::EHVecDtorIter => "`eh vector dtor iterator'",
                IntrinsicFunctionKind::EHVecVbaseCtorIter => "`eh vector vbase ctor iterator'",
                IntrinsicFunctionKind::CopyCtorClosure => "`copy ctor closure'",
                IntrinsicFunctionKind::LocalVftableCtorClosure => "`local vftable ctor closure'",
                IntrinsicFunctionKind::ArrayNew => "operator new[]",
                IntrinsicFunctionKind::ArrayDelete => "operator delete[]",
                IntrinsicFunctionKind::ManVectorCtorIter => "`managed vector ctor iterator'",
                IntrinsicFunctionKind::ManVectorDtorIter => "`managed vector dtor iterator'",
                IntrinsicFunctionKind::EHVectorCopyCtorIter => "`EH vector copy ctor iterator'",
                IntrinsicFunctionKind::EHVectorVbaseCopyCtorIter => {
                    "`EH vector vbase copy ctor iterator'"
                }
                IntrinsicFunctionKind::VectorCopyCtorIter => "`vector copy ctor iterator'",
                IntrinsicFunctionKind::VectorVbaseCopyCtorIter => {
                    "`vector vbase copy constructor iterator'"
                }
                IntrinsicFunctionKind::ManVectorVbaseCopyCtorIter => {
                    "`managed vector vbase copy constructor iterator'"
                }
                IntrinsicFunctionKind::CoAwait => "operator co_await",
                IntrinsicFunctionKind::Spaceship => "operator<=>",
            };
            safe_write!(ob, "{op}")?;
        }
        self.template_params.output(cache, ob, flags)
    }
}

#[derive(Clone, Default)]
pub(crate) struct LiteralOperatorIdentifierNode<'alloc> {
    pub(crate) template_params: TemplateParameters,
    pub(crate) name: &'alloc BStr,
}

impl<'alloc> WriteableNode for LiteralOperatorIdentifierNode<'alloc> {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        safe_write!(ob, "operator \"\"{}", self.name)?;
        self.template_params.output(cache, ob, flags)
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct LocalStaticGuardIdentifierNode {
    pub(crate) template_params: TemplateParameters,
    pub(crate) is_thread: bool,
    pub(crate) scope_index: u32,
}

impl WriteableNode for LocalStaticGuardIdentifierNode {
    fn output<W: Writer>(&self, _: &NodeCache, ob: &mut W, _: OutputFlags) -> Result<()> {
        if self.is_thread {
            safe_write!(ob, "`local static thread guard'")?;
        } else {
            safe_write!(ob, "`local static guard'")?;
        }

        if self.scope_index > 0 {
            safe_write!(ob, "{{{}}}", self.scope_index)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct ConversionOperatorIdentifierNode {
    pub(crate) template_params: TemplateParameters,
    pub(crate) target_type: Option<NodeHandle<ITypeNode>>,
}

impl WriteableNode for ConversionOperatorIdentifierNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        safe_write!(ob, "operator")?;
        self.template_params.output(cache, ob, flags)?;
        safe_write!(ob, " ")?;

        if let Some(target_type) = self.target_type.map(|x| x.resolve(cache)) {
            target_type.output(cache, ob, flags)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct StructorIdentifierNode {
    pub(crate) template_params: TemplateParameters,
    pub(crate) class: Option<NodeHandle<IIdentifierNode>>,
    pub(crate) is_destructor: bool,
}

impl WriteableNode for StructorIdentifierNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        if self.is_destructor {
            safe_write!(ob, "~")?;
        }
        if let Some(class) = self.class {
            class.resolve(cache).output(cache, ob, flags)?;
        }
        self.template_params.output(cache, ob, flags)
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct RttiBaseClassDescriptorNode {
    pub(crate) template_params: TemplateParameters,
    pub(crate) nv_offset: u32,
    pub(crate) vbptr_offset: i32,
    pub(crate) vbtable_offset: u32,
    pub(crate) flags: u32,
}

impl WriteableNode for RttiBaseClassDescriptorNode {
    fn output<W: Writer>(&self, _: &NodeCache, ob: &mut W, _: OutputFlags) -> Result<()> {
        safe_write!(
            ob,
            "`RTTI Base Class Descriptor at ({}, {}, {}, {})'",
            self.nv_offset,
            self.vbptr_offset,
            self.vbtable_offset,
            self.flags
        )
    }
}

#[derive(Clone, Default)]
pub(crate) struct NodeArrayNode<'alloc> {
    pub(crate) nodes: &'alloc [NodeHandle<INode>],
}

impl<'alloc> NodeArrayNode<'alloc> {
    pub(crate) fn do_output<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
        separator: &str,
    ) -> Result<()> {
        if let Some((&first, rest)) = self.nodes.split_first() {
            first.resolve(cache).output(cache, ob, flags)?;
            for &node in rest {
                safe_write!(ob, "{separator}")?;
                node.resolve(cache).output(cache, ob, flags)?;
            }
        }
        Ok(())
    }
}

impl<'alloc> WriteableNode for NodeArrayNode<'alloc> {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        self.do_output(cache, ob, flags, ", ")
    }
}

#[derive(Clone, Copy)]
pub(crate) struct QualifiedNameNode {
    pub(crate) components: NodeHandle<NodeArray>,
}

impl QualifiedNameNode {
    #[must_use]
    pub(crate) fn get_unqualified_identifier(
        self,
        cache: &NodeCache<'_>,
    ) -> Option<NodeHandle<IIdentifierNode>> {
        let components = self.components.resolve(cache);
        if let Some(&node) = components.nodes.last() {
            node.downcast(cache)
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) fn synthesize_from_id<'alloc>(
        allocator: &'alloc Allocator,
        cache: &mut NodeCache<'alloc>,
        identifier: NodeHandle<IIdentifierNode>,
    ) -> Result<Self> {
        let components = cache.intern(
            allocator,
            NodeArrayNode {
                nodes: allocator.allocate_slice(&[identifier.into()]),
            },
        )?;
        Ok(Self { components })
    }

    #[must_use]
    pub(crate) fn synthesize_from_name<'alloc, 'string: 'alloc>(
        allocator: &'alloc Allocator,
        cache: &mut NodeCache<'alloc>,
        name: &'string BStr,
    ) -> Result<Self> {
        let id = cache.intern(
            allocator,
            NamedIdentifierNode {
                name,
                ..Default::default()
            },
        )?;
        Self::synthesize_from_id(allocator, cache, id.into())
    }
}

impl WriteableNode for QualifiedNameNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        self.components
            .resolve(cache)
            .do_output(cache, ob, flags, "::")
    }
}

#[derive(Clone, Default)]
pub(crate) struct TemplateParameterReferenceNode {
    pub(crate) symbol: Option<NodeHandle<ISymbolNode>>,
    pub(crate) thunk_offsets: ManuallyDrop<ArrayVec<i64, 3>>, // it's safe to ignore dropping here: i64 is trivial
    pub(crate) affinity: Option<PointerAffinity>,
    #[allow(unused)]
    pub(crate) is_member_pointer: bool,
}

impl WriteableNode for TemplateParameterReferenceNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        if !self.thunk_offsets.is_empty() {
            safe_write!(ob, "{{")?;
        } else if self.affinity.is_some_and(|x| x == PointerAffinity::Pointer) {
            safe_write!(ob, "&")?;
        }

        if let Some(symbol) = self.symbol.map(|x| x.resolve(cache)) {
            symbol.output(cache, ob, flags)?;
            if !self.thunk_offsets.is_empty() {
                safe_write!(ob, ", ")?;
            }
        }

        if let Some((&first, rest)) = self.thunk_offsets.split_first() {
            safe_write!(ob, "{first}")?;
            for offset in rest {
                safe_write!(ob, ", {offset}")?;
            }
            safe_write!(ob, "}}")?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct IntegerLiteralNode {
    pub(crate) value: u64,
    pub(crate) is_negative: bool,
}

impl WriteableNode for IntegerLiteralNode {
    fn output<W: Writer>(&self, _: &NodeCache, ob: &mut W, _: OutputFlags) -> Result<()> {
        let sign = if self.is_negative { "-" } else { "" };
        safe_write!(ob, "{sign}{}", self.value)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Md5SymbolNode {
    pub(crate) name: NodeHandle<QualifiedName>,
}

impl WriteableNode for Md5SymbolNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        self.name.resolve(cache).output(cache, ob, flags)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct SpecialTableSymbolNode {
    pub(crate) name: NodeHandle<QualifiedName>,
    pub(crate) target_name: Option<NodeHandle<QualifiedName>>,
    pub(crate) quals: Qualifiers,
}

impl WriteableNode for SpecialTableSymbolNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        self.quals.output(ob, false, true)?;
        self.name.resolve(cache).output(cache, ob, flags)?;
        if let Some(target_name) = self.target_name.map(|x| x.resolve(cache)) {
            safe_write!(ob, "{{for `")?;
            target_name.output(cache, ob, flags)?;
            safe_write!(ob, "'}}")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub(crate) struct LocalStaticGuardVariableNode {
    pub(crate) name: NodeHandle<QualifiedName>,
    #[allow(unused)]
    pub(crate) is_visible: bool,
}

impl WriteableNode for LocalStaticGuardVariableNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        self.name.resolve(cache).output(cache, ob, flags)
    }
}

#[derive(Clone)]
pub(crate) struct EncodedStringLiteralNode<'alloc> {
    pub(crate) name: Option<NodeHandle<QualifiedName>>,
    pub(crate) decoded_string: &'alloc BStr,
    pub(crate) is_truncated: bool,
    pub(crate) char: CharKind,
}

impl<'alloc> WriteableNode for EncodedStringLiteralNode<'alloc> {
    fn output<W: Writer>(&self, _: &NodeCache, ob: &mut W, _: OutputFlags) -> Result<()> {
        let prefix = match self.char {
            CharKind::Wchar => "L\"",
            CharKind::Char => "\"",
            CharKind::Char16 => "u\"",
            CharKind::Char32 => "U\"",
        };
        let truncation = if self.is_truncated { "..." } else { "" };
        safe_write!(ob, "{prefix}{}\"{truncation}", self.decoded_string)
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct VariableSymbolNode {
    pub(crate) name: Option<NodeHandle<QualifiedName>>,
    pub(crate) sc: Option<StorageClass>,
    pub(crate) r#type: Option<NodeHandle<ITypeNode>>,
}

impl VariableSymbolNode {
    #[must_use]
    pub(crate) fn synthesize<'alloc, 'string: 'alloc>(
        allocator: &'alloc Allocator,
        cache: &mut NodeCache<'alloc>,
        r#type: NodeHandle<ITypeNode>,
        variable_name: &'string BStr,
    ) -> Result<Self> {
        let name = {
            let x = QualifiedNameNode::synthesize_from_name(allocator, cache, variable_name)?;
            cache.intern(allocator, x)?
        };

        Ok(Self {
            name: Some(name),
            sc: None,
            r#type: Some(r#type),
        })
    }
}

impl WriteableNode for VariableSymbolNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        let (access_spec, is_static) = match self.sc {
            Some(StorageClass::PrivateStatic) => (Some("private"), true),
            Some(StorageClass::PublicStatic) => (Some("public"), true),
            Some(StorageClass::ProtectedStatic) => (Some("protected"), true),
            _ => (None, false),
        };

        if !flags.no_access_specifier() {
            if let Some(access_spec) = access_spec {
                safe_write!(ob, "{access_spec}: ")?;
            }
        }
        if !flags.no_member_type() && is_static {
            safe_write!(ob, "static ")?;
        }

        let r#type = (!flags.no_variable_type())
            .then(|| self.r#type.map(|x| x.resolve(cache)))
            .flatten();

        if let Some(r#type) = r#type {
            r#type.output_pre(cache, ob, flags)?;
            super::output_space_if_necessary(ob)?;
        }
        if let Some(name) = self.name {
            name.resolve(cache).output(cache, ob, flags)?;
        }
        if let Some(r#type) = r#type {
            r#type.output_post(cache, ob, flags)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub(crate) struct FunctionSymbolNode {
    pub(crate) name: Option<NodeHandle<QualifiedName>>,
    pub(crate) signature: NodeHandle<ISignatureNode>,
}

impl WriteableNode for FunctionSymbolNode {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        self.signature.resolve(cache).output_pre(cache, ob, flags)?;
        super::output_space_if_necessary(ob)?;
        if let Some(name) = self.name {
            name.resolve(cache).output(cache, ob, flags)?;
        }
        self.signature.resolve(cache).output_post(cache, ob, flags)
    }
}
