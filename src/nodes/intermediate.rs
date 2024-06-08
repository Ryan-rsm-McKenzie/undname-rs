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
    },
    nodes::{
        ArrayTypeNode,
        ConversionOperatorIdentifierNode,
        CustomTypeNode,
        DynamicStructorIdentifierNode,
        EncodedStringLiteralNode,
        FuncClass,
        FunctionSignatureNode,
        FunctionSymbolNode,
        IntegerLiteralNode,
        IntrinsicFunctionIdentifierNode,
        LiteralOperatorIdentifierNode,
        LocalStaticGuardIdentifierNode,
        LocalStaticGuardVariableNode,
        Md5SymbolNode,
        NamedIdentifierNode,
        NodeArrayNode,
        PointerTypeNode,
        PrimitiveTypeNode,
        QualifiedNameNode,
        Qualifiers,
        Result,
        RttiBaseClassDescriptorNode,
        SpecialTableSymbolNode,
        StructorIdentifierNode,
        TagTypeNode,
        TemplateParameterReferenceNode,
        ThunkSignatureNode,
        VariableSymbolNode,
        VcallThunkIdentifierNode,
        WriteableNode,
        WriteableTypeNode,
    },
    OutputFlags,
    Writer,
};

pub(crate) trait Downcast<To> {
    #[must_use]
    fn downcast(self) -> Option<To>;
}

macro_rules! impl_downcast {
    ($from:ident::$variant:ident => $to:ty) => {
        impl<'storage, 'alloc: 'storage> Downcast<$to> for $from<'storage, 'alloc> {
            fn downcast(self) -> Option<$to> {
                if let Self::$variant(x) = self {
                    Some(x)
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! impl_upcast {
    ($from:ty => $to:ident::$variant:ident) => {
        impl<'storage, 'alloc: 'storage> From<$from> for $to<'storage, 'alloc> {
            fn from(value: $from) -> Self {
                Self::$variant(value.into())
            }
        }
    };
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy)]
pub(crate) enum Node<
    TypeT,
    IdentifierT,
    NodeArrayT,
    QualifiedNameT,
    TemplateParameterReferenceT,
    IntegerLiteralT,
    SymbolT,
> {
    Type(TypeT),
    Identifier(IdentifierT),
    NodeArray(NodeArrayT),
    QualifiedName(QualifiedNameT),
    TemplateParameterReference(TemplateParameterReferenceT),
    IntegerLiteral(IntegerLiteralT),
    Symbol(SymbolT),
}

pub(super) type NodeConst<'storage, 'alloc> = Node<
    TypeNodeConst<'storage, 'alloc>,
    IdentifierNodeConst<'storage, 'alloc>,
    &'storage NodeArrayNode<'alloc>,
    &'storage QualifiedNameNode,
    &'storage TemplateParameterReferenceNode,
    &'storage IntegerLiteralNode,
    SymbolNodeConst<'storage, 'alloc>,
>;

impl<'storage, 'alloc: 'storage> WriteableNode for NodeConst<'storage, 'alloc> {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        match self {
            Self::Type(x) => x.output(cache, ob, flags),
            Self::Identifier(x) => x.output(cache, ob, flags),
            Self::NodeArray(x) => x.output(cache, ob, flags),
            Self::QualifiedName(x) => x.output(cache, ob, flags),
            Self::TemplateParameterReference(x) => x.output(cache, ob, flags),
            Self::IntegerLiteral(x) => x.output(cache, ob, flags),
            Self::Symbol(x) => x.output(cache, ob, flags),
        }
    }
}

impl_upcast!(TypeNodeConst<'storage, 'alloc> => NodeConst::Type);
impl_upcast!(&'storage PrimitiveTypeNode => NodeConst::Type);
impl_upcast!(SignatureNodeConst<'storage, 'alloc> => NodeConst::Type);
impl_upcast!(&'storage PointerTypeNode => NodeConst::Type);
impl_upcast!(&'storage TagTypeNode => NodeConst::Type);
impl_upcast!(&'storage ArrayTypeNode => NodeConst::Type);
impl_upcast!(&'storage CustomTypeNode => NodeConst::Type);

impl_upcast!(IdentifierNodeConst<'storage, 'alloc> => NodeConst::Identifier);
impl_upcast!(&'storage VcallThunkIdentifierNode => NodeConst::Identifier);
impl_upcast!(&'storage DynamicStructorIdentifierNode => NodeConst::Identifier);
impl_upcast!(&'storage NamedIdentifierNode<'alloc> => NodeConst::Identifier);
impl_upcast!(&'storage IntrinsicFunctionIdentifierNode => NodeConst::Identifier);
impl_upcast!(&'storage LiteralOperatorIdentifierNode<'alloc> => NodeConst::Identifier);
impl_upcast!(&'storage LocalStaticGuardIdentifierNode => NodeConst::Identifier);
impl_upcast!(&'storage ConversionOperatorIdentifierNode => NodeConst::Identifier);
impl_upcast!(&'storage StructorIdentifierNode => NodeConst::Identifier);
impl_upcast!(&'storage RttiBaseClassDescriptorNode => NodeConst::Identifier);

impl_upcast!(&'storage NodeArrayNode<'alloc> => NodeConst::NodeArray);
impl_upcast!(&'storage QualifiedNameNode => NodeConst::QualifiedName);
impl_upcast!(&'storage TemplateParameterReferenceNode => NodeConst::TemplateParameterReference);
impl_upcast!(&'storage IntegerLiteralNode => NodeConst::IntegerLiteral);

impl_upcast!(SymbolNodeConst<'storage, 'alloc> => NodeConst::Symbol);
impl_upcast!(&'storage Md5SymbolNode => NodeConst::Symbol);
impl_upcast!(&'storage SpecialTableSymbolNode => NodeConst::Symbol);
impl_upcast!(&'storage LocalStaticGuardVariableNode => NodeConst::Symbol);
impl_upcast!(&'storage EncodedStringLiteralNode<'alloc> => NodeConst::Symbol);
impl_upcast!(&'storage VariableSymbolNode => NodeConst::Symbol);
impl_upcast!(&'storage FunctionSymbolNode => NodeConst::Symbol);

impl<'storage, 'alloc: 'storage> From<&'storage FunctionSignatureNode>
    for NodeConst<'storage, 'alloc>
{
    fn from(value: &'storage FunctionSignatureNode) -> Self {
        let value: SignatureNodeConst = value.into();
        Self::Type(value.into())
    }
}

impl<'storage, 'alloc: 'storage> From<&'storage ThunkSignatureNode>
    for NodeConst<'storage, 'alloc>
{
    fn from(value: &'storage ThunkSignatureNode) -> Self {
        let value: SignatureNodeConst = value.into();
        Self::Type(value.into())
    }
}

impl_downcast!(NodeConst::Type => TypeNodeConst<'storage, 'alloc>);
impl_downcast!(NodeConst::Identifier => IdentifierNodeConst<'storage, 'alloc>);
impl_downcast!(NodeConst::NodeArray => &'storage NodeArrayNode<'alloc>);
impl_downcast!(NodeConst::QualifiedName => &'storage QualifiedNameNode);
impl_downcast!(NodeConst::TemplateParameterReference => &'storage TemplateParameterReferenceNode);
impl_downcast!(NodeConst::IntegerLiteral => &'storage IntegerLiteralNode);
impl_downcast!(NodeConst::Symbol => SymbolNodeConst<'storage, 'alloc>);

impl<'storage, 'alloc: 'storage> Downcast<SignatureNodeConst<'storage, 'alloc>>
    for NodeConst<'storage, 'alloc>
{
    fn downcast(self) -> Option<SignatureNodeConst<'storage, 'alloc>> {
        if let Self::Type(TypeNode::Signature(inner)) = self {
            Some(inner)
        } else {
            None
        }
    }
}

impl<'storage, 'alloc: 'storage> Downcast<&'storage FunctionSignatureNode>
    for NodeConst<'storage, 'alloc>
{
    fn downcast(self) -> Option<&'storage FunctionSignatureNode> {
        if let Self::Type(TypeNode::Signature(SignatureNode::FunctionSignature(node))) = self {
            Some(node)
        } else {
            None
        }
    }
}

impl<'storage, 'alloc: 'storage> Downcast<&'storage ThunkSignatureNode>
    for NodeConst<'storage, 'alloc>
{
    fn downcast(self) -> Option<&'storage ThunkSignatureNode> {
        if let Self::Type(TypeNode::Signature(SignatureNode::ThunkSignature(node))) = self {
            Some(node)
        } else {
            None
        }
    }
}

pub(super) type NodeMut<'storage, 'alloc> = Node<
    TypeNodeMut<'storage, 'alloc>,
    IdentifierNodeMut<'storage, 'alloc>,
    &'storage mut NodeArrayNode<'alloc>,
    &'storage mut QualifiedNameNode,
    &'storage mut TemplateParameterReferenceNode,
    &'storage mut IntegerLiteralNode,
    SymbolNodeMut<'storage, 'alloc>,
>;

impl_upcast!(TypeNodeMut<'storage, 'alloc> => NodeMut::Type);
impl_upcast!(&'storage mut PrimitiveTypeNode => NodeMut::Type);
impl_upcast!(SignatureNodeMut<'storage, 'alloc> => NodeMut::Type);
impl_upcast!(&'storage mut PointerTypeNode => NodeMut::Type);
impl_upcast!(&'storage mut TagTypeNode => NodeMut::Type);
impl_upcast!(&'storage mut ArrayTypeNode => NodeMut::Type);
impl_upcast!(&'storage mut CustomTypeNode => NodeMut::Type);

impl_upcast!(IdentifierNodeMut<'storage, 'alloc> => NodeMut::Identifier);
impl_upcast!(&'storage mut VcallThunkIdentifierNode => NodeMut::Identifier);
impl_upcast!(&'storage mut DynamicStructorIdentifierNode => NodeMut::Identifier);
impl_upcast!(&'storage mut NamedIdentifierNode<'alloc> => NodeMut::Identifier);
impl_upcast!(&'storage mut IntrinsicFunctionIdentifierNode => NodeMut::Identifier);
impl_upcast!(&'storage mut LiteralOperatorIdentifierNode<'alloc> => NodeMut::Identifier);
impl_upcast!(&'storage mut LocalStaticGuardIdentifierNode => NodeMut::Identifier);
impl_upcast!(&'storage mut ConversionOperatorIdentifierNode => NodeMut::Identifier);
impl_upcast!(&'storage mut StructorIdentifierNode => NodeMut::Identifier);
impl_upcast!(&'storage mut RttiBaseClassDescriptorNode => NodeMut::Identifier);

impl_upcast!(&'storage mut NodeArrayNode<'alloc> => NodeMut::NodeArray);
impl_upcast!(&'storage mut QualifiedNameNode => NodeMut::QualifiedName);
impl_upcast!(&'storage mut TemplateParameterReferenceNode => NodeMut::TemplateParameterReference);
impl_upcast!(&'storage mut IntegerLiteralNode => NodeMut::IntegerLiteral);

impl_upcast!(SymbolNodeMut<'storage, 'alloc> => NodeMut::Symbol);
impl_upcast!(&'storage mut Md5SymbolNode => NodeMut::Symbol);
impl_upcast!(&'storage mut SpecialTableSymbolNode => NodeMut::Symbol);
impl_upcast!(&'storage mut LocalStaticGuardVariableNode => NodeMut::Symbol);
impl_upcast!(&'storage mut EncodedStringLiteralNode<'alloc> => NodeMut::Symbol);
impl_upcast!(&'storage mut VariableSymbolNode => NodeMut::Symbol);
impl_upcast!(&'storage mut FunctionSymbolNode => NodeMut::Symbol);

impl<'storage, 'alloc: 'storage> From<&'storage mut FunctionSignatureNode>
    for NodeMut<'storage, 'alloc>
{
    fn from(value: &'storage mut FunctionSignatureNode) -> Self {
        let value: SignatureNodeMut = value.into();
        Self::Type(value.into())
    }
}

impl<'storage, 'alloc: 'storage> From<&'storage mut ThunkSignatureNode>
    for NodeMut<'storage, 'alloc>
{
    fn from(value: &'storage mut ThunkSignatureNode) -> Self {
        let value: SignatureNodeMut = value.into();
        Self::Type(value.into())
    }
}

impl_downcast!(NodeMut::Type => TypeNodeMut<'storage, 'alloc>);
impl_downcast!(NodeMut::Identifier => IdentifierNodeMut<'storage, 'alloc>);
impl_downcast!(NodeMut::NodeArray => &'storage mut NodeArrayNode<'alloc>);
impl_downcast!(NodeMut::QualifiedName => &'storage mut QualifiedNameNode);
impl_downcast!(NodeMut::TemplateParameterReference => &'storage mut TemplateParameterReferenceNode);
impl_downcast!(NodeMut::IntegerLiteral => &'storage mut IntegerLiteralNode);
impl_downcast!(NodeMut::Symbol => SymbolNodeMut<'storage, 'alloc>);

impl<'storage, 'alloc: 'storage> Downcast<SignatureNodeMut<'storage, 'alloc>>
    for NodeMut<'storage, 'alloc>
{
    fn downcast(self) -> Option<SignatureNodeMut<'storage, 'alloc>> {
        if let Self::Type(TypeNode::Signature(inner)) = self {
            Some(inner)
        } else {
            None
        }
    }
}

impl<'storage, 'alloc: 'storage> Downcast<&'storage mut FunctionSignatureNode>
    for NodeMut<'storage, 'alloc>
{
    fn downcast(self) -> Option<&'storage mut FunctionSignatureNode> {
        if let Self::Type(TypeNode::Signature(SignatureNode::FunctionSignature(node))) = self {
            Some(node)
        } else {
            None
        }
    }
}

impl<'storage, 'alloc: 'storage> Downcast<&'storage mut ThunkSignatureNode>
    for NodeMut<'storage, 'alloc>
{
    fn downcast(self) -> Option<&'storage mut ThunkSignatureNode> {
        if let Self::Type(TypeNode::Signature(SignatureNode::ThunkSignature(node))) = self {
            Some(node)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum TypeNode<
    PrimitiveTypeT,
    SignatureT,
    PointerTypeT,
    TagTypeT,
    ArrayTypeT,
    CustomTypeT,
> {
    PrimitiveType(PrimitiveTypeT),
    Signature(SignatureT),
    PointerType(PointerTypeT),
    TagType(TagTypeT),
    ArrayType(ArrayTypeT),
    CustomType(CustomTypeT),
}

pub(super) type TypeNodeConst<'storage, 'alloc> = TypeNode<
    &'storage PrimitiveTypeNode,
    SignatureNodeConst<'storage, 'alloc>,
    &'storage PointerTypeNode,
    &'storage TagTypeNode,
    &'storage ArrayTypeNode,
    &'storage CustomTypeNode,
>;

impl<'storage, 'alloc: 'storage> WriteableNode for TypeNodeConst<'storage, 'alloc> {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        self.output_pair(cache, ob, flags)
    }
}

impl<'storage, 'alloc: 'storage> WriteableTypeNode for TypeNodeConst<'storage, 'alloc> {
    fn output_pair<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
    ) -> Result<()> {
        match self {
            Self::PrimitiveType(x) => x.output_pair(cache, ob, flags),
            Self::Signature(x) => x.output_pair(cache, ob, flags),
            Self::PointerType(x) => x.output_pair(cache, ob, flags),
            Self::TagType(x) => x.output_pair(cache, ob, flags),
            Self::ArrayType(x) => x.output_pair(cache, ob, flags),
            Self::CustomType(x) => x.output_pair(cache, ob, flags),
        }
    }

    fn output_pre<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
    ) -> Result<()> {
        match self {
            Self::PrimitiveType(x) => x.output_pre(cache, ob, flags),
            Self::Signature(x) => x.output_pre(cache, ob, flags),
            Self::PointerType(x) => x.output_pre(cache, ob, flags),
            Self::TagType(x) => x.output_pre(cache, ob, flags),
            Self::ArrayType(x) => x.output_pre(cache, ob, flags),
            Self::CustomType(x) => x.output_pre(cache, ob, flags),
        }
    }

    fn output_post<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
    ) -> Result<()> {
        match self {
            Self::PrimitiveType(x) => x.output_post(cache, ob, flags),
            Self::Signature(x) => x.output_post(cache, ob, flags),
            Self::PointerType(x) => x.output_post(cache, ob, flags),
            Self::TagType(x) => x.output_post(cache, ob, flags),
            Self::ArrayType(x) => x.output_post(cache, ob, flags),
            Self::CustomType(x) => x.output_post(cache, ob, flags),
        }
    }
}

impl_upcast!(&'storage PrimitiveTypeNode => TypeNodeConst::PrimitiveType);
impl_upcast!(SignatureNodeConst<'storage, 'alloc> => TypeNodeConst::Signature);
impl_upcast!(&'storage FunctionSignatureNode => TypeNodeConst::Signature);
impl_upcast!(&'storage ThunkSignatureNode => TypeNodeConst::Signature);
impl_upcast!(&'storage PointerTypeNode => TypeNodeConst::PointerType);
impl_upcast!(&'storage TagTypeNode => TypeNodeConst::TagType);
impl_upcast!(&'storage ArrayTypeNode => TypeNodeConst::ArrayType);
impl_upcast!(&'storage CustomTypeNode => TypeNodeConst::CustomType);

impl_downcast!(TypeNodeConst::PrimitiveType => &'storage PrimitiveTypeNode);
impl_downcast!(TypeNodeConst::Signature => SignatureNodeConst<'storage, 'alloc>);
impl_downcast!(TypeNodeConst::PointerType => &'storage PointerTypeNode);
impl_downcast!(TypeNodeConst::TagType => &'storage TagTypeNode);
impl_downcast!(TypeNodeConst::ArrayType => &'storage ArrayTypeNode);
impl_downcast!(TypeNodeConst::CustomType => &'storage CustomTypeNode);

impl<'storage, 'alloc: 'storage> Downcast<&'storage FunctionSignatureNode>
    for TypeNodeConst<'storage, 'alloc>
{
    fn downcast(self) -> Option<&'storage FunctionSignatureNode> {
        if let Self::Signature(SignatureNode::FunctionSignature(inner)) = self {
            Some(inner)
        } else {
            None
        }
    }
}

impl<'storage, 'alloc: 'storage> Downcast<&'storage ThunkSignatureNode>
    for TypeNodeConst<'storage, 'alloc>
{
    fn downcast(self) -> Option<&'storage ThunkSignatureNode> {
        if let Self::Signature(SignatureNode::ThunkSignature(inner)) = self {
            Some(inner)
        } else {
            None
        }
    }
}

pub(super) type TypeNodeMut<'storage, 'alloc> = TypeNode<
    &'storage mut PrimitiveTypeNode,
    SignatureNodeMut<'storage, 'alloc>,
    &'storage mut PointerTypeNode,
    &'storage mut TagTypeNode,
    &'storage mut ArrayTypeNode,
    &'storage mut CustomTypeNode,
>;

impl<'storage, 'alloc: 'storage> TypeNodeMut<'storage, 'alloc> {
    pub(crate) fn append_quals(&mut self, quals: Qualifiers) {
        match self {
            Self::PrimitiveType(x) => x.quals |= quals,
            Self::Signature(x) => match x {
                SignatureNode::FunctionSignature(x) => x.quals |= quals,
                SignatureNode::ThunkSignature(x) => x.quals |= quals,
            },
            Self::PointerType(x) => x.quals |= quals,
            Self::TagType(x) => x.quals |= quals,
            Self::ArrayType(x) => x.quals |= quals,
            Self::CustomType(x) => x.quals |= quals,
        }
    }

    pub(crate) fn set_quals(&mut self, quals: Qualifiers) {
        match self {
            Self::PrimitiveType(x) => x.quals = quals,
            Self::Signature(x) => match x {
                SignatureNode::FunctionSignature(x) => x.quals = quals,
                SignatureNode::ThunkSignature(x) => x.quals = quals,
            },
            Self::PointerType(x) => x.quals = quals,
            Self::TagType(x) => x.quals = quals,
            Self::ArrayType(x) => x.quals = quals,
            Self::CustomType(x) => x.quals = quals,
        }
    }
}

impl_upcast!(&'storage mut PrimitiveTypeNode => TypeNodeMut::PrimitiveType);
impl_upcast!(SignatureNodeMut<'storage, 'alloc> => TypeNodeMut::Signature);
impl_upcast!(&'storage mut FunctionSignatureNode => TypeNodeMut::Signature);
impl_upcast!(&'storage mut ThunkSignatureNode => TypeNodeMut::Signature);
impl_upcast!(&'storage mut PointerTypeNode => TypeNodeMut::PointerType);
impl_upcast!(&'storage mut TagTypeNode => TypeNodeMut::TagType);
impl_upcast!(&'storage mut ArrayTypeNode => TypeNodeMut::ArrayType);
impl_upcast!(&'storage mut CustomTypeNode => TypeNodeMut::CustomType);

impl_downcast!(TypeNodeMut::PrimitiveType => &'storage mut PrimitiveTypeNode);
impl_downcast!(TypeNodeMut::Signature => SignatureNodeMut<'storage, 'alloc>);
impl_downcast!(TypeNodeMut::PointerType => &'storage mut PointerTypeNode);
impl_downcast!(TypeNodeMut::TagType => &'storage mut TagTypeNode);
impl_downcast!(TypeNodeMut::ArrayType => &'storage mut ArrayTypeNode);
impl_downcast!(TypeNodeMut::CustomType => &'storage mut CustomTypeNode);

impl<'storage, 'alloc: 'storage> Downcast<&'storage mut FunctionSignatureNode>
    for TypeNodeMut<'storage, 'alloc>
{
    fn downcast(self) -> Option<&'storage mut FunctionSignatureNode> {
        if let Self::Signature(SignatureNode::FunctionSignature(inner)) = self {
            Some(inner)
        } else {
            None
        }
    }
}

impl<'storage, 'alloc: 'storage> Downcast<&'storage mut ThunkSignatureNode>
    for TypeNodeMut<'storage, 'alloc>
{
    fn downcast(self) -> Option<&'storage mut ThunkSignatureNode> {
        if let Self::Signature(SignatureNode::ThunkSignature(inner)) = self {
            Some(inner)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum SignatureNode<FunctionSignatureT, ThunkSignatureT> {
    FunctionSignature(FunctionSignatureT),
    ThunkSignature(ThunkSignatureT),
}

pub(super) type SignatureNodeConst<'storage, 'alloc> =
    SignatureNode<&'storage FunctionSignatureNode, &'storage ThunkSignatureNode>;

impl<'storage, 'alloc: 'storage> SignatureNodeConst<'storage, 'alloc> {
    pub(crate) fn as_node(&self) -> &FunctionSignatureNode {
        match self {
            Self::FunctionSignature(x) => x,
            Self::ThunkSignature(x) => &x.function_node,
        }
    }
}

impl<'storage, 'alloc: 'storage> WriteableNode for SignatureNodeConst<'storage, 'alloc> {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        self.output_pair(cache, ob, flags)
    }
}

impl<'storage, 'alloc: 'storage> WriteableTypeNode for SignatureNodeConst<'storage, 'alloc> {
    fn output_pair<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
    ) -> Result<()> {
        match self {
            Self::FunctionSignature(x) => x.output_pair(cache, ob, flags),
            Self::ThunkSignature(x) => x.output_pair(cache, ob, flags),
        }
    }

    fn output_pre<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
    ) -> Result<()> {
        match self {
            Self::FunctionSignature(x) => x.output_pre(cache, ob, flags),
            Self::ThunkSignature(x) => x.output_pre(cache, ob, flags),
        }
    }

    fn output_post<W: Writer>(
        &self,
        cache: &NodeCache,
        ob: &mut W,
        flags: OutputFlags,
    ) -> Result<()> {
        match self {
            Self::FunctionSignature(x) => x.output_post(cache, ob, flags),
            Self::ThunkSignature(x) => x.output_post(cache, ob, flags),
        }
    }
}

impl_upcast!(&'storage FunctionSignatureNode => SignatureNodeConst::FunctionSignature);
impl_upcast!(&'storage ThunkSignatureNode => SignatureNodeConst::ThunkSignature);

impl_downcast!(SignatureNodeConst::FunctionSignature => &'storage FunctionSignatureNode);
impl_downcast!(SignatureNodeConst::ThunkSignature => &'storage ThunkSignatureNode);

pub(super) type SignatureNodeMut<'storage, 'alloc> =
    SignatureNode<&'storage mut FunctionSignatureNode, &'storage mut ThunkSignatureNode>;

impl<'storage, 'alloc: 'storage> SignatureNodeMut<'storage, 'alloc> {
    pub(crate) fn set_function_class(&mut self, function_class: FuncClass) {
        match self {
            Self::FunctionSignature(x) => x.function_class = function_class,
            Self::ThunkSignature(x) => x.function_class = function_class,
        }
    }
}

impl_upcast!(&'storage mut FunctionSignatureNode => SignatureNodeMut::FunctionSignature);
impl_upcast!(&'storage mut ThunkSignatureNode => SignatureNodeMut::ThunkSignature);

impl_downcast!(SignatureNodeMut::FunctionSignature => &'storage mut FunctionSignatureNode);
impl_downcast!(SignatureNodeMut::ThunkSignature => &'storage mut ThunkSignatureNode);

#[derive(Clone, Copy)]
pub(crate) enum IdentifierNode<
    VcallThunkIdentifierT,
    DynamicStructorIdentifierT,
    NamedIdentifierT,
    IntrinsicFunctionIdentifierT,
    LiteralOperatorIdentifierT,
    LocalStaticGuardIdentifierT,
    ConversionOperatorIdentifierT,
    StructorIdentifierT,
    RttiBaseClassDescriptorT,
> {
    VcallThunkIdentifier(VcallThunkIdentifierT),
    DynamicStructorIdentifier(DynamicStructorIdentifierT),
    NamedIdentifier(NamedIdentifierT),
    IntrinsicFunctionIdentifier(IntrinsicFunctionIdentifierT),
    LiteralOperatorIdentifier(LiteralOperatorIdentifierT),
    LocalStaticGuardIdentifier(LocalStaticGuardIdentifierT),
    ConversionOperatorIdentifier(ConversionOperatorIdentifierT),
    StructorIdentifier(StructorIdentifierT),
    RttiBaseClassDescriptor(RttiBaseClassDescriptorT),
}

pub(super) type IdentifierNodeConst<'storage, 'alloc> = IdentifierNode<
    &'storage VcallThunkIdentifierNode,
    &'storage DynamicStructorIdentifierNode,
    &'storage NamedIdentifierNode<'alloc>,
    &'storage IntrinsicFunctionIdentifierNode,
    &'storage LiteralOperatorIdentifierNode<'alloc>,
    &'storage LocalStaticGuardIdentifierNode,
    &'storage ConversionOperatorIdentifierNode,
    &'storage StructorIdentifierNode,
    &'storage RttiBaseClassDescriptorNode,
>;

impl<'storage, 'alloc: 'storage> WriteableNode for IdentifierNodeConst<'storage, 'alloc> {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        match self {
            Self::VcallThunkIdentifier(x) => x.output(cache, ob, flags),
            Self::DynamicStructorIdentifier(x) => x.output(cache, ob, flags),
            Self::NamedIdentifier(x) => x.output(cache, ob, flags),
            Self::IntrinsicFunctionIdentifier(x) => x.output(cache, ob, flags),
            Self::LiteralOperatorIdentifier(x) => x.output(cache, ob, flags),
            Self::LocalStaticGuardIdentifier(x) => x.output(cache, ob, flags),
            Self::ConversionOperatorIdentifier(x) => x.output(cache, ob, flags),
            Self::StructorIdentifier(x) => x.output(cache, ob, flags),
            Self::RttiBaseClassDescriptor(x) => x.output(cache, ob, flags),
        }
    }
}

impl_upcast!(&'storage VcallThunkIdentifierNode => IdentifierNodeConst::VcallThunkIdentifier);
impl_upcast!(&'storage DynamicStructorIdentifierNode => IdentifierNodeConst::DynamicStructorIdentifier);
impl_upcast!(&'storage NamedIdentifierNode<'alloc> => IdentifierNodeConst::NamedIdentifier);
impl_upcast!(&'storage IntrinsicFunctionIdentifierNode => IdentifierNodeConst::IntrinsicFunctionIdentifier);
impl_upcast!(&'storage LiteralOperatorIdentifierNode<'alloc> => IdentifierNodeConst::LiteralOperatorIdentifier);
impl_upcast!(&'storage LocalStaticGuardIdentifierNode => IdentifierNodeConst::LocalStaticGuardIdentifier);
impl_upcast!(&'storage ConversionOperatorIdentifierNode => IdentifierNodeConst::ConversionOperatorIdentifier);
impl_upcast!(&'storage StructorIdentifierNode => IdentifierNodeConst::StructorIdentifier);
impl_upcast!(&'storage RttiBaseClassDescriptorNode => IdentifierNodeConst::RttiBaseClassDescriptor);

impl_downcast!(IdentifierNodeConst::VcallThunkIdentifier => &'storage VcallThunkIdentifierNode);
impl_downcast!(IdentifierNodeConst::DynamicStructorIdentifier => &'storage DynamicStructorIdentifierNode);
impl_downcast!(IdentifierNodeConst::NamedIdentifier => &'storage NamedIdentifierNode<'alloc>);
impl_downcast!(IdentifierNodeConst::IntrinsicFunctionIdentifier => &'storage IntrinsicFunctionIdentifierNode);
impl_downcast!(IdentifierNodeConst::LiteralOperatorIdentifier => &'storage LiteralOperatorIdentifierNode<'alloc>);
impl_downcast!(IdentifierNodeConst::LocalStaticGuardIdentifier => &'storage LocalStaticGuardIdentifierNode);
impl_downcast!(IdentifierNodeConst::ConversionOperatorIdentifier => &'storage ConversionOperatorIdentifierNode);
impl_downcast!(IdentifierNodeConst::StructorIdentifier => &'storage StructorIdentifierNode);
impl_downcast!(IdentifierNodeConst::RttiBaseClassDescriptor => &'storage RttiBaseClassDescriptorNode);

pub(super) type IdentifierNodeMut<'storage, 'alloc> = IdentifierNode<
    &'storage mut VcallThunkIdentifierNode,
    &'storage mut DynamicStructorIdentifierNode,
    &'storage mut NamedIdentifierNode<'alloc>,
    &'storage mut IntrinsicFunctionIdentifierNode,
    &'storage mut LiteralOperatorIdentifierNode<'alloc>,
    &'storage mut LocalStaticGuardIdentifierNode,
    &'storage mut ConversionOperatorIdentifierNode,
    &'storage mut StructorIdentifierNode,
    &'storage mut RttiBaseClassDescriptorNode,
>;

impl<'storage, 'alloc: 'storage> IdentifierNodeMut<'storage, 'alloc> {
    pub(crate) fn set_template_params(&mut self, template_params: NodeHandle<NodeArray>) {
        let params = match self {
            Self::VcallThunkIdentifier(x) => &mut x.template_params,
            Self::DynamicStructorIdentifier(x) => &mut x.template_params,
            Self::NamedIdentifier(x) => &mut x.template_params,
            Self::IntrinsicFunctionIdentifier(x) => &mut x.template_params,
            Self::LiteralOperatorIdentifier(x) => &mut x.template_params,
            Self::LocalStaticGuardIdentifier(x) => &mut x.template_params,
            Self::ConversionOperatorIdentifier(x) => &mut x.template_params,
            Self::StructorIdentifier(x) => &mut x.template_params,
            Self::RttiBaseClassDescriptor(x) => &mut x.template_params,
        };
        params.0 = Some(template_params);
    }
}

impl_upcast!(&'storage mut VcallThunkIdentifierNode => IdentifierNodeMut::VcallThunkIdentifier);
impl_upcast!(&'storage mut DynamicStructorIdentifierNode => IdentifierNodeMut::DynamicStructorIdentifier);
impl_upcast!(&'storage mut NamedIdentifierNode<'alloc> => IdentifierNodeMut::NamedIdentifier);
impl_upcast!(&'storage mut IntrinsicFunctionIdentifierNode => IdentifierNodeMut::IntrinsicFunctionIdentifier);
impl_upcast!(&'storage mut LiteralOperatorIdentifierNode<'alloc> => IdentifierNodeMut::LiteralOperatorIdentifier);
impl_upcast!(&'storage mut LocalStaticGuardIdentifierNode => IdentifierNodeMut::LocalStaticGuardIdentifier);
impl_upcast!(&'storage mut ConversionOperatorIdentifierNode => IdentifierNodeMut::ConversionOperatorIdentifier);
impl_upcast!(&'storage mut StructorIdentifierNode => IdentifierNodeMut::StructorIdentifier);
impl_upcast!(&'storage mut RttiBaseClassDescriptorNode => IdentifierNodeMut::RttiBaseClassDescriptor);

impl_downcast!(IdentifierNodeMut::VcallThunkIdentifier => &'storage mut VcallThunkIdentifierNode);
impl_downcast!(IdentifierNodeMut::DynamicStructorIdentifier => &'storage mut DynamicStructorIdentifierNode);
impl_downcast!(IdentifierNodeMut::NamedIdentifier => &'storage mut NamedIdentifierNode<'alloc>);
impl_downcast!(IdentifierNodeMut::IntrinsicFunctionIdentifier => &'storage mut IntrinsicFunctionIdentifierNode);
impl_downcast!(IdentifierNodeMut::LiteralOperatorIdentifier => &'storage mut LiteralOperatorIdentifierNode<'alloc>);
impl_downcast!(IdentifierNodeMut::LocalStaticGuardIdentifier => &'storage mut LocalStaticGuardIdentifierNode);
impl_downcast!(IdentifierNodeMut::ConversionOperatorIdentifier => &'storage mut ConversionOperatorIdentifierNode);
impl_downcast!(IdentifierNodeMut::StructorIdentifier => &'storage mut StructorIdentifierNode);
impl_downcast!(IdentifierNodeMut::RttiBaseClassDescriptor => &'storage mut RttiBaseClassDescriptorNode);

#[derive(Clone, Copy)]
pub(crate) enum SymbolNode<
    Md5SymbolT,
    SpecialTableSymbolT,
    LocalStaticGuardVariableT,
    EncodedStringLiteralT,
    VariableSymbolT,
    FunctionSymbolT,
> {
    Md5Symbol(Md5SymbolT),
    SpecialTableSymbol(SpecialTableSymbolT),
    LocalStaticGuardVariable(LocalStaticGuardVariableT),
    EncodedStringLiteral(EncodedStringLiteralT),
    VariableSymbol(VariableSymbolT),
    FunctionSymbol(FunctionSymbolT),
}

pub(super) type SymbolNodeConst<'storage, 'alloc> = SymbolNode<
    &'storage Md5SymbolNode,
    &'storage SpecialTableSymbolNode,
    &'storage LocalStaticGuardVariableNode,
    &'storage EncodedStringLiteralNode<'alloc>,
    &'storage VariableSymbolNode,
    &'storage FunctionSymbolNode,
>;

impl<'storage, 'alloc: 'storage> SymbolNodeConst<'storage, 'alloc> {
    #[must_use]
    pub(crate) fn get_name(&self) -> Option<NodeHandle<QualifiedName>> {
        match self {
            Self::Md5Symbol(x) => Some(x.name),
            Self::SpecialTableSymbol(x) => Some(x.name),
            Self::LocalStaticGuardVariable(x) => Some(x.name),
            Self::EncodedStringLiteral(x) => x.name,
            Self::VariableSymbol(x) => x.name,
            Self::FunctionSymbol(x) => x.name,
        }
    }
}

impl<'storage, 'alloc: 'storage> WriteableNode for SymbolNodeConst<'storage, 'alloc> {
    fn output<W: Writer>(&self, cache: &NodeCache, ob: &mut W, flags: OutputFlags) -> Result<()> {
        match self {
            Self::Md5Symbol(x) => x.output(cache, ob, flags),
            Self::SpecialTableSymbol(x) => x.output(cache, ob, flags),
            Self::LocalStaticGuardVariable(x) => x.output(cache, ob, flags),
            Self::EncodedStringLiteral(x) => x.output(cache, ob, flags),
            Self::VariableSymbol(x) => x.output(cache, ob, flags),
            Self::FunctionSymbol(x) => x.output(cache, ob, flags),
        }
    }
}

impl_upcast!(&'storage Md5SymbolNode => SymbolNodeConst::Md5Symbol);
impl_upcast!(&'storage SpecialTableSymbolNode => SymbolNodeConst::SpecialTableSymbol);
impl_upcast!(&'storage LocalStaticGuardVariableNode => SymbolNodeConst::LocalStaticGuardVariable);
impl_upcast!(&'storage EncodedStringLiteralNode<'alloc> => SymbolNodeConst::EncodedStringLiteral);
impl_upcast!(&'storage VariableSymbolNode => SymbolNodeConst::VariableSymbol);
impl_upcast!(&'storage FunctionSymbolNode => SymbolNodeConst::FunctionSymbol);

impl_downcast!(SymbolNodeConst::Md5Symbol => &'storage Md5SymbolNode);
impl_downcast!(SymbolNodeConst::SpecialTableSymbol => &'storage SpecialTableSymbolNode);
impl_downcast!(SymbolNodeConst::LocalStaticGuardVariable => &'storage LocalStaticGuardVariableNode);
impl_downcast!(SymbolNodeConst::EncodedStringLiteral => &'storage EncodedStringLiteralNode<'alloc>);
impl_downcast!(SymbolNodeConst::VariableSymbol => &'storage VariableSymbolNode);
impl_downcast!(SymbolNodeConst::FunctionSymbol => &'storage FunctionSymbolNode);

pub(super) type SymbolNodeMut<'storage, 'alloc> = SymbolNode<
    &'storage mut Md5SymbolNode,
    &'storage mut SpecialTableSymbolNode,
    &'storage mut LocalStaticGuardVariableNode,
    &'storage mut EncodedStringLiteralNode<'alloc>,
    &'storage mut VariableSymbolNode,
    &'storage mut FunctionSymbolNode,
>;

impl<'storage, 'alloc: 'storage> SymbolNodeMut<'storage, 'alloc> {
    pub(crate) fn set_name(&mut self, name: NodeHandle<QualifiedName>) {
        match self {
            Self::Md5Symbol(x) => x.name = name,
            Self::SpecialTableSymbol(x) => x.name = name,
            Self::LocalStaticGuardVariable(x) => x.name = name,
            Self::EncodedStringLiteral(x) => x.name = Some(name),
            Self::VariableSymbol(x) => x.name = Some(name),
            Self::FunctionSymbol(x) => x.name = Some(name),
        }
    }
}

impl_upcast!(&'storage mut Md5SymbolNode => SymbolNodeMut::Md5Symbol);
impl_upcast!(&'storage mut SpecialTableSymbolNode => SymbolNodeMut::SpecialTableSymbol);
impl_upcast!(&'storage mut LocalStaticGuardVariableNode => SymbolNodeMut::LocalStaticGuardVariable);
impl_upcast!(&'storage mut EncodedStringLiteralNode<'alloc> => SymbolNodeMut::EncodedStringLiteral);
impl_upcast!(&'storage mut VariableSymbolNode => SymbolNodeMut::VariableSymbol);
impl_upcast!(&'storage mut FunctionSymbolNode => SymbolNodeMut::FunctionSymbol);

impl_downcast!(SymbolNodeMut::Md5Symbol => &'storage mut Md5SymbolNode);
impl_downcast!(SymbolNodeMut::SpecialTableSymbol => &'storage mut SpecialTableSymbolNode);
impl_downcast!(SymbolNodeMut::LocalStaticGuardVariable => &'storage mut LocalStaticGuardVariableNode);
impl_downcast!(SymbolNodeMut::EncodedStringLiteral => &'storage mut EncodedStringLiteralNode<'alloc>);
impl_downcast!(SymbolNodeMut::VariableSymbol => &'storage mut VariableSymbolNode);
impl_downcast!(SymbolNodeMut::FunctionSymbol => &'storage mut FunctionSymbolNode);

pub(crate) trait IntermediateNode<'storage, 'alloc: 'storage> {
    type Const;
    type Mut;
}

macro_rules! is_intermediate_node {
    ($interface:ident => ($const:ident, $mut:ident)) => {
        pub(crate) struct $interface;

        impl<'storage, 'alloc: 'storage> IntermediateNode<'storage, 'alloc> for $interface {
            type Const = $const<'storage, 'alloc>;
            type Mut = $mut<'storage, 'alloc>;
        }
    };
}

is_intermediate_node!(INode => (NodeConst, NodeMut));
is_intermediate_node!(ITypeNode => (TypeNodeConst, TypeNodeMut));
is_intermediate_node!(ISignatureNode => (SignatureNodeConst, SignatureNodeMut));
is_intermediate_node!(IIdentifierNode => (IdentifierNodeConst, IdentifierNodeMut));
is_intermediate_node!(ISymbolNode => (SymbolNodeConst, SymbolNodeMut));
