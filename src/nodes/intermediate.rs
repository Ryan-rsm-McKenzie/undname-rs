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
        NodeCache,
        NodeHandle,
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
        OutputBuffer,
        OutputFlags,
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
};

pub(crate) trait Downcast<To> {
    #[must_use]
    fn downcast(self) -> Option<To>;
}

macro_rules! impl_downcast {
    ($from:ident::$variant:ident => $to:ty) => {
        impl<'node> Downcast<$to> for $from<'node> {
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
        impl<'node> From<$from> for $to<'node> {
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

pub(super) type NodeConst<'node> = Node<
    TypeNodeConst<'node>,
    IdentifierNodeConst<'node>,
    &'node NodeArrayNode,
    &'node QualifiedNameNode,
    &'node TemplateParameterReferenceNode,
    &'node IntegerLiteralNode,
    SymbolNodeConst<'node>,
>;

impl<'node> WriteableNode for NodeConst<'node> {
    fn output(&self, cache: &NodeCache, ob: &mut OutputBuffer, flags: OutputFlags) -> Result<()> {
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

impl_upcast!(TypeNodeConst<'node> => NodeConst::Type);
impl_upcast!(&'node PrimitiveTypeNode => NodeConst::Type);
impl_upcast!(SignatureNodeConst<'node> => NodeConst::Type);
impl_upcast!(&'node PointerTypeNode => NodeConst::Type);
impl_upcast!(&'node TagTypeNode => NodeConst::Type);
impl_upcast!(&'node ArrayTypeNode => NodeConst::Type);
impl_upcast!(&'node CustomTypeNode => NodeConst::Type);

impl_upcast!(IdentifierNodeConst<'node> => NodeConst::Identifier);
impl_upcast!(&'node VcallThunkIdentifierNode => NodeConst::Identifier);
impl_upcast!(&'node DynamicStructorIdentifierNode => NodeConst::Identifier);
impl_upcast!(&'node NamedIdentifierNode => NodeConst::Identifier);
impl_upcast!(&'node IntrinsicFunctionIdentifierNode => NodeConst::Identifier);
impl_upcast!(&'node LiteralOperatorIdentifierNode => NodeConst::Identifier);
impl_upcast!(&'node LocalStaticGuardIdentifierNode => NodeConst::Identifier);
impl_upcast!(&'node ConversionOperatorIdentifierNode => NodeConst::Identifier);
impl_upcast!(&'node StructorIdentifierNode => NodeConst::Identifier);
impl_upcast!(&'node RttiBaseClassDescriptorNode => NodeConst::Identifier);

impl_upcast!(&'node NodeArrayNode => NodeConst::NodeArray);
impl_upcast!(&'node QualifiedNameNode => NodeConst::QualifiedName);
impl_upcast!(&'node TemplateParameterReferenceNode => NodeConst::TemplateParameterReference);
impl_upcast!(&'node IntegerLiteralNode => NodeConst::IntegerLiteral);

impl_upcast!(SymbolNodeConst<'node> => NodeConst::Symbol);
impl_upcast!(&'node Md5SymbolNode => NodeConst::Symbol);
impl_upcast!(&'node SpecialTableSymbolNode => NodeConst::Symbol);
impl_upcast!(&'node LocalStaticGuardVariableNode => NodeConst::Symbol);
impl_upcast!(&'node EncodedStringLiteralNode => NodeConst::Symbol);
impl_upcast!(&'node VariableSymbolNode => NodeConst::Symbol);
impl_upcast!(&'node FunctionSymbolNode => NodeConst::Symbol);

impl<'node> From<&'node FunctionSignatureNode> for NodeConst<'node> {
    fn from(value: &'node FunctionSignatureNode) -> Self {
        let value: SignatureNodeConst = value.into();
        Self::Type(value.into())
    }
}

impl<'node> From<&'node ThunkSignatureNode> for NodeConst<'node> {
    fn from(value: &'node ThunkSignatureNode) -> Self {
        let value: SignatureNodeConst = value.into();
        Self::Type(value.into())
    }
}

impl_downcast!(NodeConst::Type => TypeNodeConst<'node>);
impl_downcast!(NodeConst::Identifier => IdentifierNodeConst<'node>);
impl_downcast!(NodeConst::NodeArray => &'node NodeArrayNode);
impl_downcast!(NodeConst::QualifiedName => &'node QualifiedNameNode);
impl_downcast!(NodeConst::TemplateParameterReference => &'node TemplateParameterReferenceNode);
impl_downcast!(NodeConst::IntegerLiteral => &'node IntegerLiteralNode);
impl_downcast!(NodeConst::Symbol => SymbolNodeConst<'node>);

impl<'node> Downcast<SignatureNodeConst<'node>> for NodeConst<'node> {
    fn downcast(self) -> Option<SignatureNodeConst<'node>> {
        if let Self::Type(TypeNode::Signature(inner)) = self {
            Some(inner)
        } else {
            None
        }
    }
}

impl<'node> Downcast<&'node FunctionSignatureNode> for NodeConst<'node> {
    fn downcast(self) -> Option<&'node FunctionSignatureNode> {
        if let Self::Type(TypeNode::Signature(SignatureNode::FunctionSignature(node))) = self {
            Some(node)
        } else {
            None
        }
    }
}

impl<'node> Downcast<&'node ThunkSignatureNode> for NodeConst<'node> {
    fn downcast(self) -> Option<&'node ThunkSignatureNode> {
        if let Self::Type(TypeNode::Signature(SignatureNode::ThunkSignature(node))) = self {
            Some(node)
        } else {
            None
        }
    }
}

pub(super) type NodeMut<'node> = Node<
    TypeNodeMut<'node>,
    IdentifierNodeMut<'node>,
    &'node mut NodeArrayNode,
    &'node mut QualifiedNameNode,
    &'node mut TemplateParameterReferenceNode,
    &'node mut IntegerLiteralNode,
    SymbolNodeMut<'node>,
>;

impl<'node> WriteableNode for NodeMut<'node> {
    fn output(&self, cache: &NodeCache, ob: &mut OutputBuffer, flags: OutputFlags) -> Result<()> {
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

impl_upcast!(TypeNodeMut<'node> => NodeMut::Type);
impl_upcast!(&'node mut PrimitiveTypeNode => NodeMut::Type);
impl_upcast!(SignatureNodeMut<'node> => NodeMut::Type);
impl_upcast!(&'node mut PointerTypeNode => NodeMut::Type);
impl_upcast!(&'node mut TagTypeNode => NodeMut::Type);
impl_upcast!(&'node mut ArrayTypeNode => NodeMut::Type);
impl_upcast!(&'node mut CustomTypeNode => NodeMut::Type);

impl_upcast!(IdentifierNodeMut<'node> => NodeMut::Identifier);
impl_upcast!(&'node mut VcallThunkIdentifierNode => NodeMut::Identifier);
impl_upcast!(&'node mut DynamicStructorIdentifierNode => NodeMut::Identifier);
impl_upcast!(&'node mut NamedIdentifierNode => NodeMut::Identifier);
impl_upcast!(&'node mut IntrinsicFunctionIdentifierNode => NodeMut::Identifier);
impl_upcast!(&'node mut LiteralOperatorIdentifierNode => NodeMut::Identifier);
impl_upcast!(&'node mut LocalStaticGuardIdentifierNode => NodeMut::Identifier);
impl_upcast!(&'node mut ConversionOperatorIdentifierNode => NodeMut::Identifier);
impl_upcast!(&'node mut StructorIdentifierNode => NodeMut::Identifier);
impl_upcast!(&'node mut RttiBaseClassDescriptorNode => NodeMut::Identifier);

impl_upcast!(&'node mut NodeArrayNode => NodeMut::NodeArray);
impl_upcast!(&'node mut QualifiedNameNode => NodeMut::QualifiedName);
impl_upcast!(&'node mut TemplateParameterReferenceNode => NodeMut::TemplateParameterReference);
impl_upcast!(&'node mut IntegerLiteralNode => NodeMut::IntegerLiteral);

impl_upcast!(SymbolNodeMut<'node> => NodeMut::Symbol);
impl_upcast!(&'node mut Md5SymbolNode => NodeMut::Symbol);
impl_upcast!(&'node mut SpecialTableSymbolNode => NodeMut::Symbol);
impl_upcast!(&'node mut LocalStaticGuardVariableNode => NodeMut::Symbol);
impl_upcast!(&'node mut EncodedStringLiteralNode => NodeMut::Symbol);
impl_upcast!(&'node mut VariableSymbolNode => NodeMut::Symbol);
impl_upcast!(&'node mut FunctionSymbolNode => NodeMut::Symbol);

impl<'node> From<&'node mut FunctionSignatureNode> for NodeMut<'node> {
    fn from(value: &'node mut FunctionSignatureNode) -> Self {
        let value: SignatureNodeMut = value.into();
        Self::Type(value.into())
    }
}

impl<'node> From<&'node mut ThunkSignatureNode> for NodeMut<'node> {
    fn from(value: &'node mut ThunkSignatureNode) -> Self {
        let value: SignatureNodeMut = value.into();
        Self::Type(value.into())
    }
}

impl_downcast!(NodeMut::Type => TypeNodeMut<'node>);
impl_downcast!(NodeMut::Identifier => IdentifierNodeMut<'node>);
impl_downcast!(NodeMut::NodeArray => &'node mut NodeArrayNode);
impl_downcast!(NodeMut::QualifiedName => &'node mut QualifiedNameNode);
impl_downcast!(NodeMut::TemplateParameterReference => &'node mut TemplateParameterReferenceNode);
impl_downcast!(NodeMut::IntegerLiteral => &'node mut IntegerLiteralNode);
impl_downcast!(NodeMut::Symbol => SymbolNodeMut<'node>);

impl<'node> Downcast<SignatureNodeMut<'node>> for NodeMut<'node> {
    fn downcast(self) -> Option<SignatureNodeMut<'node>> {
        if let Self::Type(TypeNode::Signature(inner)) = self {
            Some(inner)
        } else {
            None
        }
    }
}

impl<'node> Downcast<&'node mut FunctionSignatureNode> for NodeMut<'node> {
    fn downcast(self) -> Option<&'node mut FunctionSignatureNode> {
        if let Self::Type(TypeNode::Signature(SignatureNode::FunctionSignature(node))) = self {
            Some(node)
        } else {
            None
        }
    }
}

impl<'node> Downcast<&'node mut ThunkSignatureNode> for NodeMut<'node> {
    fn downcast(self) -> Option<&'node mut ThunkSignatureNode> {
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

pub(super) type TypeNodeConst<'node> = TypeNode<
    &'node PrimitiveTypeNode,
    SignatureNodeConst<'node>,
    &'node PointerTypeNode,
    &'node TagTypeNode,
    &'node ArrayTypeNode,
    &'node CustomTypeNode,
>;

impl<'node> WriteableNode for TypeNodeConst<'node> {
    fn output(&self, cache: &NodeCache, ob: &mut OutputBuffer, flags: OutputFlags) -> Result<()> {
        self.output_pair(cache, ob, flags)
    }
}

impl<'node> WriteableTypeNode for TypeNodeConst<'node> {
    fn output_pair(
        &self,
        cache: &NodeCache,
        ob: &mut OutputBuffer,
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

    fn output_pre(
        &self,
        cache: &NodeCache,
        ob: &mut OutputBuffer,
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

    fn output_post(
        &self,
        cache: &NodeCache,
        ob: &mut OutputBuffer,
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

impl_upcast!(&'node PrimitiveTypeNode => TypeNodeConst::PrimitiveType);
impl_upcast!(SignatureNodeConst<'node> => TypeNodeConst::Signature);
impl_upcast!(&'node FunctionSignatureNode => TypeNodeConst::Signature);
impl_upcast!(&'node ThunkSignatureNode => TypeNodeConst::Signature);
impl_upcast!(&'node PointerTypeNode => TypeNodeConst::PointerType);
impl_upcast!(&'node TagTypeNode => TypeNodeConst::TagType);
impl_upcast!(&'node ArrayTypeNode => TypeNodeConst::ArrayType);
impl_upcast!(&'node CustomTypeNode => TypeNodeConst::CustomType);

impl_downcast!(TypeNodeConst::PrimitiveType => &'node PrimitiveTypeNode);
impl_downcast!(TypeNodeConst::Signature => SignatureNodeConst<'node>);
impl_downcast!(TypeNodeConst::PointerType => &'node PointerTypeNode);
impl_downcast!(TypeNodeConst::TagType => &'node TagTypeNode);
impl_downcast!(TypeNodeConst::ArrayType => &'node ArrayTypeNode);
impl_downcast!(TypeNodeConst::CustomType => &'node CustomTypeNode);

impl<'node> Downcast<&'node FunctionSignatureNode> for TypeNodeConst<'node> {
    fn downcast(self) -> Option<&'node FunctionSignatureNode> {
        if let Self::Signature(SignatureNode::FunctionSignature(inner)) = self {
            Some(inner)
        } else {
            None
        }
    }
}

impl<'node> Downcast<&'node ThunkSignatureNode> for TypeNodeConst<'node> {
    fn downcast(self) -> Option<&'node ThunkSignatureNode> {
        if let Self::Signature(SignatureNode::ThunkSignature(inner)) = self {
            Some(inner)
        } else {
            None
        }
    }
}

pub(super) type TypeNodeMut<'node> = TypeNode<
    &'node mut PrimitiveTypeNode,
    SignatureNodeMut<'node>,
    &'node mut PointerTypeNode,
    &'node mut TagTypeNode,
    &'node mut ArrayTypeNode,
    &'node mut CustomTypeNode,
>;

impl<'node> TypeNodeMut<'node> {
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

impl<'node> WriteableNode for TypeNodeMut<'node> {
    fn output(&self, cache: &NodeCache, ob: &mut OutputBuffer, flags: OutputFlags) -> Result<()> {
        self.output_pair(cache, ob, flags)
    }
}

impl<'node> WriteableTypeNode for TypeNodeMut<'node> {
    fn output_pair(
        &self,
        cache: &NodeCache,
        ob: &mut OutputBuffer,
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

    fn output_pre(
        &self,
        cache: &NodeCache,
        ob: &mut OutputBuffer,
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

    fn output_post(
        &self,
        cache: &NodeCache,
        ob: &mut OutputBuffer,
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

impl_upcast!(&'node mut PrimitiveTypeNode => TypeNodeMut::PrimitiveType);
impl_upcast!(SignatureNodeMut<'node> => TypeNodeMut::Signature);
impl_upcast!(&'node mut FunctionSignatureNode => TypeNodeMut::Signature);
impl_upcast!(&'node mut ThunkSignatureNode => TypeNodeMut::Signature);
impl_upcast!(&'node mut PointerTypeNode => TypeNodeMut::PointerType);
impl_upcast!(&'node mut TagTypeNode => TypeNodeMut::TagType);
impl_upcast!(&'node mut ArrayTypeNode => TypeNodeMut::ArrayType);
impl_upcast!(&'node mut CustomTypeNode => TypeNodeMut::CustomType);

impl_downcast!(TypeNodeMut::PrimitiveType => &'node mut PrimitiveTypeNode);
impl_downcast!(TypeNodeMut::Signature => SignatureNodeMut<'node>);
impl_downcast!(TypeNodeMut::PointerType => &'node mut PointerTypeNode);
impl_downcast!(TypeNodeMut::TagType => &'node mut TagTypeNode);
impl_downcast!(TypeNodeMut::ArrayType => &'node mut ArrayTypeNode);
impl_downcast!(TypeNodeMut::CustomType => &'node mut CustomTypeNode);

impl<'node> Downcast<&'node mut FunctionSignatureNode> for TypeNodeMut<'node> {
    fn downcast(self) -> Option<&'node mut FunctionSignatureNode> {
        if let Self::Signature(SignatureNode::FunctionSignature(inner)) = self {
            Some(inner)
        } else {
            None
        }
    }
}

impl<'node> Downcast<&'node mut ThunkSignatureNode> for TypeNodeMut<'node> {
    fn downcast(self) -> Option<&'node mut ThunkSignatureNode> {
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

pub(super) type SignatureNodeConst<'node> =
    SignatureNode<&'node FunctionSignatureNode, &'node ThunkSignatureNode>;

impl<'node> SignatureNodeConst<'node> {
    pub(crate) fn as_node(&self) -> &FunctionSignatureNode {
        match self {
            Self::FunctionSignature(x) => x,
            Self::ThunkSignature(x) => &x.function_node,
        }
    }
}

impl<'node> WriteableNode for SignatureNodeConst<'node> {
    fn output(&self, cache: &NodeCache, ob: &mut OutputBuffer, flags: OutputFlags) -> Result<()> {
        self.output_pair(cache, ob, flags)
    }
}

impl<'node> WriteableTypeNode for SignatureNodeConst<'node> {
    fn output_pair(
        &self,
        cache: &NodeCache,
        ob: &mut OutputBuffer,
        flags: OutputFlags,
    ) -> Result<()> {
        match self {
            Self::FunctionSignature(x) => x.output_pair(cache, ob, flags),
            Self::ThunkSignature(x) => x.output_pair(cache, ob, flags),
        }
    }

    fn output_pre(
        &self,
        cache: &NodeCache,
        ob: &mut OutputBuffer,
        flags: OutputFlags,
    ) -> Result<()> {
        match self {
            Self::FunctionSignature(x) => x.output_pre(cache, ob, flags),
            Self::ThunkSignature(x) => x.output_pre(cache, ob, flags),
        }
    }

    fn output_post(
        &self,
        cache: &NodeCache,
        ob: &mut OutputBuffer,
        flags: OutputFlags,
    ) -> Result<()> {
        match self {
            Self::FunctionSignature(x) => x.output_post(cache, ob, flags),
            Self::ThunkSignature(x) => x.output_post(cache, ob, flags),
        }
    }
}

impl_upcast!(&'node FunctionSignatureNode => SignatureNodeConst::FunctionSignature);
impl_upcast!(&'node ThunkSignatureNode => SignatureNodeConst::ThunkSignature);

impl_downcast!(SignatureNodeConst::FunctionSignature => &'node FunctionSignatureNode);
impl_downcast!(SignatureNodeConst::ThunkSignature => &'node ThunkSignatureNode);

pub(super) type SignatureNodeMut<'node> =
    SignatureNode<&'node mut FunctionSignatureNode, &'node mut ThunkSignatureNode>;

impl<'node> SignatureNodeMut<'node> {
    pub(crate) fn set_function_class(&mut self, function_class: FuncClass) {
        match self {
            Self::FunctionSignature(x) => x.function_class = function_class,
            Self::ThunkSignature(x) => x.function_class = function_class,
        }
    }
}

impl<'node> WriteableNode for SignatureNodeMut<'node> {
    fn output(&self, cache: &NodeCache, ob: &mut OutputBuffer, flags: OutputFlags) -> Result<()> {
        self.output_pair(cache, ob, flags)
    }
}

impl<'node> WriteableTypeNode for SignatureNodeMut<'node> {
    fn output_pair(
        &self,
        cache: &NodeCache,
        ob: &mut OutputBuffer,
        flags: OutputFlags,
    ) -> Result<()> {
        match self {
            Self::FunctionSignature(x) => x.output_pair(cache, ob, flags),
            Self::ThunkSignature(x) => x.output_pair(cache, ob, flags),
        }
    }

    fn output_pre(
        &self,
        cache: &NodeCache,
        ob: &mut OutputBuffer,
        flags: OutputFlags,
    ) -> Result<()> {
        match self {
            Self::FunctionSignature(x) => x.output_pre(cache, ob, flags),
            Self::ThunkSignature(x) => x.output_pre(cache, ob, flags),
        }
    }

    fn output_post(
        &self,
        cache: &NodeCache,
        ob: &mut OutputBuffer,
        flags: OutputFlags,
    ) -> Result<()> {
        match self {
            Self::FunctionSignature(x) => x.output_post(cache, ob, flags),
            Self::ThunkSignature(x) => x.output_post(cache, ob, flags),
        }
    }
}

impl_upcast!(&'node mut FunctionSignatureNode => SignatureNodeMut::FunctionSignature);
impl_upcast!(&'node mut ThunkSignatureNode => SignatureNodeMut::ThunkSignature);

impl_downcast!(SignatureNodeMut::FunctionSignature => &'node mut FunctionSignatureNode);
impl_downcast!(SignatureNodeMut::ThunkSignature => &'node mut ThunkSignatureNode);

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

pub(super) type IdentifierNodeConst<'node> = IdentifierNode<
    &'node VcallThunkIdentifierNode,
    &'node DynamicStructorIdentifierNode,
    &'node NamedIdentifierNode,
    &'node IntrinsicFunctionIdentifierNode,
    &'node LiteralOperatorIdentifierNode,
    &'node LocalStaticGuardIdentifierNode,
    &'node ConversionOperatorIdentifierNode,
    &'node StructorIdentifierNode,
    &'node RttiBaseClassDescriptorNode,
>;

impl<'node> WriteableNode for IdentifierNodeConst<'node> {
    fn output(&self, cache: &NodeCache, ob: &mut OutputBuffer, flags: OutputFlags) -> Result<()> {
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

impl_upcast!(&'node VcallThunkIdentifierNode => IdentifierNodeConst::VcallThunkIdentifier);
impl_upcast!(&'node DynamicStructorIdentifierNode => IdentifierNodeConst::DynamicStructorIdentifier);
impl_upcast!(&'node NamedIdentifierNode => IdentifierNodeConst::NamedIdentifier);
impl_upcast!(&'node IntrinsicFunctionIdentifierNode => IdentifierNodeConst::IntrinsicFunctionIdentifier);
impl_upcast!(&'node LiteralOperatorIdentifierNode => IdentifierNodeConst::LiteralOperatorIdentifier);
impl_upcast!(&'node LocalStaticGuardIdentifierNode => IdentifierNodeConst::LocalStaticGuardIdentifier);
impl_upcast!(&'node ConversionOperatorIdentifierNode => IdentifierNodeConst::ConversionOperatorIdentifier);
impl_upcast!(&'node StructorIdentifierNode => IdentifierNodeConst::StructorIdentifier);
impl_upcast!(&'node RttiBaseClassDescriptorNode => IdentifierNodeConst::RttiBaseClassDescriptor);

impl_downcast!(IdentifierNodeConst::VcallThunkIdentifier => &'node VcallThunkIdentifierNode);
impl_downcast!(IdentifierNodeConst::DynamicStructorIdentifier => &'node DynamicStructorIdentifierNode);
impl_downcast!(IdentifierNodeConst::NamedIdentifier => &'node NamedIdentifierNode);
impl_downcast!(IdentifierNodeConst::IntrinsicFunctionIdentifier => &'node IntrinsicFunctionIdentifierNode);
impl_downcast!(IdentifierNodeConst::LiteralOperatorIdentifier => &'node LiteralOperatorIdentifierNode);
impl_downcast!(IdentifierNodeConst::LocalStaticGuardIdentifier => &'node LocalStaticGuardIdentifierNode);
impl_downcast!(IdentifierNodeConst::ConversionOperatorIdentifier => &'node ConversionOperatorIdentifierNode);
impl_downcast!(IdentifierNodeConst::StructorIdentifier => &'node StructorIdentifierNode);
impl_downcast!(IdentifierNodeConst::RttiBaseClassDescriptor => &'node RttiBaseClassDescriptorNode);

pub(super) type IdentifierNodeMut<'node> = IdentifierNode<
    &'node mut VcallThunkIdentifierNode,
    &'node mut DynamicStructorIdentifierNode,
    &'node mut NamedIdentifierNode,
    &'node mut IntrinsicFunctionIdentifierNode,
    &'node mut LiteralOperatorIdentifierNode,
    &'node mut LocalStaticGuardIdentifierNode,
    &'node mut ConversionOperatorIdentifierNode,
    &'node mut StructorIdentifierNode,
    &'node mut RttiBaseClassDescriptorNode,
>;

impl<'node> IdentifierNodeMut<'node> {
    pub(crate) fn set_template_params(&mut self, template_params: NodeHandle<NodeArrayNode>) {
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

impl<'node> WriteableNode for IdentifierNodeMut<'node> {
    fn output(&self, cache: &NodeCache, ob: &mut OutputBuffer, flags: OutputFlags) -> Result<()> {
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

impl_upcast!(&'node mut VcallThunkIdentifierNode => IdentifierNodeMut::VcallThunkIdentifier);
impl_upcast!(&'node mut DynamicStructorIdentifierNode => IdentifierNodeMut::DynamicStructorIdentifier);
impl_upcast!(&'node mut NamedIdentifierNode => IdentifierNodeMut::NamedIdentifier);
impl_upcast!(&'node mut IntrinsicFunctionIdentifierNode => IdentifierNodeMut::IntrinsicFunctionIdentifier);
impl_upcast!(&'node mut LiteralOperatorIdentifierNode => IdentifierNodeMut::LiteralOperatorIdentifier);
impl_upcast!(&'node mut LocalStaticGuardIdentifierNode => IdentifierNodeMut::LocalStaticGuardIdentifier);
impl_upcast!(&'node mut ConversionOperatorIdentifierNode => IdentifierNodeMut::ConversionOperatorIdentifier);
impl_upcast!(&'node mut StructorIdentifierNode => IdentifierNodeMut::StructorIdentifier);
impl_upcast!(&'node mut RttiBaseClassDescriptorNode => IdentifierNodeMut::RttiBaseClassDescriptor);

impl_downcast!(IdentifierNodeMut::VcallThunkIdentifier => &'node mut VcallThunkIdentifierNode);
impl_downcast!(IdentifierNodeMut::DynamicStructorIdentifier => &'node mut DynamicStructorIdentifierNode);
impl_downcast!(IdentifierNodeMut::NamedIdentifier => &'node mut NamedIdentifierNode);
impl_downcast!(IdentifierNodeMut::IntrinsicFunctionIdentifier => &'node mut IntrinsicFunctionIdentifierNode);
impl_downcast!(IdentifierNodeMut::LiteralOperatorIdentifier => &'node mut LiteralOperatorIdentifierNode);
impl_downcast!(IdentifierNodeMut::LocalStaticGuardIdentifier => &'node mut LocalStaticGuardIdentifierNode);
impl_downcast!(IdentifierNodeMut::ConversionOperatorIdentifier => &'node mut ConversionOperatorIdentifierNode);
impl_downcast!(IdentifierNodeMut::StructorIdentifier => &'node mut StructorIdentifierNode);
impl_downcast!(IdentifierNodeMut::RttiBaseClassDescriptor => &'node mut RttiBaseClassDescriptorNode);

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

pub(super) type SymbolNodeConst<'node> = SymbolNode<
    &'node Md5SymbolNode,
    &'node SpecialTableSymbolNode,
    &'node LocalStaticGuardVariableNode,
    &'node EncodedStringLiteralNode,
    &'node VariableSymbolNode,
    &'node FunctionSymbolNode,
>;

impl<'node> SymbolNodeConst<'node> {
    #[must_use]
    pub(crate) fn get_name(&self) -> Option<NodeHandle<QualifiedNameNode>> {
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

impl<'node> WriteableNode for SymbolNodeConst<'node> {
    fn output(&self, cache: &NodeCache, ob: &mut OutputBuffer, flags: OutputFlags) -> Result<()> {
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

impl_upcast!(&'node Md5SymbolNode => SymbolNodeConst::Md5Symbol);
impl_upcast!(&'node SpecialTableSymbolNode => SymbolNodeConst::SpecialTableSymbol);
impl_upcast!(&'node LocalStaticGuardVariableNode => SymbolNodeConst::LocalStaticGuardVariable);
impl_upcast!(&'node EncodedStringLiteralNode => SymbolNodeConst::EncodedStringLiteral);
impl_upcast!(&'node VariableSymbolNode => SymbolNodeConst::VariableSymbol);
impl_upcast!(&'node FunctionSymbolNode => SymbolNodeConst::FunctionSymbol);

impl_downcast!(SymbolNodeConst::Md5Symbol => &'node Md5SymbolNode);
impl_downcast!(SymbolNodeConst::SpecialTableSymbol => &'node SpecialTableSymbolNode);
impl_downcast!(SymbolNodeConst::LocalStaticGuardVariable => &'node LocalStaticGuardVariableNode);
impl_downcast!(SymbolNodeConst::EncodedStringLiteral => &'node EncodedStringLiteralNode);
impl_downcast!(SymbolNodeConst::VariableSymbol => &'node VariableSymbolNode);
impl_downcast!(SymbolNodeConst::FunctionSymbol => &'node FunctionSymbolNode);

pub(super) type SymbolNodeMut<'node> = SymbolNode<
    &'node mut Md5SymbolNode,
    &'node mut SpecialTableSymbolNode,
    &'node mut LocalStaticGuardVariableNode,
    &'node mut EncodedStringLiteralNode,
    &'node mut VariableSymbolNode,
    &'node mut FunctionSymbolNode,
>;

impl<'node> SymbolNodeMut<'node> {
    pub(crate) fn set_name(&mut self, name: NodeHandle<QualifiedNameNode>) {
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

impl<'node> WriteableNode for SymbolNodeMut<'node> {
    fn output(&self, cache: &NodeCache, ob: &mut OutputBuffer, flags: OutputFlags) -> Result<()> {
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

impl_upcast!(&'node mut Md5SymbolNode => SymbolNodeMut::Md5Symbol);
impl_upcast!(&'node mut SpecialTableSymbolNode => SymbolNodeMut::SpecialTableSymbol);
impl_upcast!(&'node mut LocalStaticGuardVariableNode => SymbolNodeMut::LocalStaticGuardVariable);
impl_upcast!(&'node mut EncodedStringLiteralNode => SymbolNodeMut::EncodedStringLiteral);
impl_upcast!(&'node mut VariableSymbolNode => SymbolNodeMut::VariableSymbol);
impl_upcast!(&'node mut FunctionSymbolNode => SymbolNodeMut::FunctionSymbol);

impl_downcast!(SymbolNodeMut::Md5Symbol => &'node mut Md5SymbolNode);
impl_downcast!(SymbolNodeMut::SpecialTableSymbol => &'node mut SpecialTableSymbolNode);
impl_downcast!(SymbolNodeMut::LocalStaticGuardVariable => &'node mut LocalStaticGuardVariableNode);
impl_downcast!(SymbolNodeMut::EncodedStringLiteral => &'node mut EncodedStringLiteralNode);
impl_downcast!(SymbolNodeMut::VariableSymbol => &'node mut VariableSymbolNode);
impl_downcast!(SymbolNodeMut::FunctionSymbol => &'node mut FunctionSymbolNode);

pub(crate) trait IntermediateNode<'node> {
    type Const;
    type Mut;
}

macro_rules! is_intermediate_node {
    ($interface:ident => ($const:ident, $mut:ident)) => {
        pub(crate) struct $interface;

        impl<'node> IntermediateNode<'node> for $interface {
            type Const = $const<'node>;
            type Mut = $mut<'node>;
        }
    };
}

is_intermediate_node!(INode => (NodeConst, NodeMut));
is_intermediate_node!(ITypeNode => (TypeNodeConst, TypeNodeMut));
is_intermediate_node!(ISignatureNode => (SignatureNodeConst, SignatureNodeMut));
is_intermediate_node!(IIdentifierNode => (IdentifierNodeConst, IdentifierNodeMut));
is_intermediate_node!(ISymbolNode => (SymbolNodeConst, SymbolNodeMut));
