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
    alloc,
    nodes::{
        ArrayTypeNode,
        ConversionOperatorIdentifierNode,
        CustomTypeNode,
        DynamicStructorIdentifierNode,
        EncodedStringLiteralNode,
        FunctionSignatureNode,
        FunctionSymbolNode,
        IIdentifierNode,
        INode,
        ISignatureNode,
        ISymbolNode,
        ITypeNode,
        IdentifierNode,
        IntegerLiteralNode,
        IntermediateNode,
        IntrinsicFunctionIdentifierNode,
        LiteralOperatorIdentifierNode,
        LocalStaticGuardIdentifierNode,
        LocalStaticGuardVariableNode,
        Md5SymbolNode,
        NamedIdentifierNode,
        Node,
        NodeArrayNode,
        PointerTypeNode,
        PrimitiveTypeNode,
        QualifiedNameNode,
        RttiBaseClassDescriptorNode,
        SignatureNode,
        SpecialTableSymbolNode,
        StructorIdentifierNode,
        SymbolNode,
        TagTypeNode,
        TemplateParameterReferenceNode,
        ThunkSignatureNode,
        TypeNode,
        VariableSymbolNode,
        VcallThunkIdentifierNode,
    },
    Error,
    Result,
};
use bumpalo::{
    collections::Vec as BumpVec,
    Bump,
};
use nonmax::NonMaxUsize;
use std::marker::PhantomData;

pub(crate) enum NodeStorage<'alloc> {
    PrimitiveType(&'alloc mut PrimitiveTypeNode),
    FunctionSignature(&'alloc mut FunctionSignatureNode),
    ThunkSignature(&'alloc mut ThunkSignatureNode),
    PointerType(&'alloc mut PointerTypeNode),
    TagType(&'alloc mut TagTypeNode),
    ArrayType(&'alloc mut ArrayTypeNode),
    CustomType(&'alloc mut CustomTypeNode),

    VcallThunkIdentifier(&'alloc mut VcallThunkIdentifierNode),
    DynamicStructorIdentifier(&'alloc mut DynamicStructorIdentifierNode),
    NamedIdentifier(&'alloc mut NamedIdentifierNode<'alloc>),
    IntrinsicFunctionIdentifier(&'alloc mut IntrinsicFunctionIdentifierNode),
    LiteralOperatorIdentifier(&'alloc mut LiteralOperatorIdentifierNode<'alloc>),
    LocalStaticGuardIdentifier(&'alloc mut LocalStaticGuardIdentifierNode),
    ConversionOperatorIdentifier(&'alloc mut ConversionOperatorIdentifierNode),
    StructorIdentifier(&'alloc mut StructorIdentifierNode),
    RttiBaseClassDescriptor(&'alloc mut RttiBaseClassDescriptorNode),

    NodeArray(&'alloc mut NodeArrayNode<'alloc>),
    QualifiedName(&'alloc mut QualifiedNameNode),
    TemplateParameterReference(&'alloc mut TemplateParameterReferenceNode),
    IntegerLiteral(&'alloc mut IntegerLiteralNode),

    Md5Symbol(&'alloc mut Md5SymbolNode),
    SpecialTableSymbol(&'alloc mut SpecialTableSymbolNode),
    LocalStaticGuardVariable(&'alloc mut LocalStaticGuardVariableNode),
    EncodedStringLiteral(&'alloc mut EncodedStringLiteralNode<'alloc>),
    VariableSymbol(&'alloc mut VariableSymbolNode),
    FunctionSymbol(&'alloc mut FunctionSymbolNode),
}

macro_rules! impl_into_storage {
    ($from:ty => $to:ident) => {
        impl<'alloc> From<&'alloc mut $from> for NodeStorage<'alloc> {
            fn from(value: &'alloc mut $from) -> Self {
                Self::$to(value.into())
            }
        }
    };
}

impl_into_storage!(PrimitiveTypeNode => PrimitiveType);
impl_into_storage!(FunctionSignatureNode => FunctionSignature);
impl_into_storage!(ThunkSignatureNode => ThunkSignature);
impl_into_storage!(PointerTypeNode => PointerType);
impl_into_storage!(TagTypeNode => TagType);
impl_into_storage!(ArrayTypeNode => ArrayType);
impl_into_storage!(CustomTypeNode => CustomType);

impl_into_storage!(VcallThunkIdentifierNode => VcallThunkIdentifier);
impl_into_storage!(DynamicStructorIdentifierNode => DynamicStructorIdentifier);
impl_into_storage!(NamedIdentifierNode<'alloc> => NamedIdentifier);
impl_into_storage!(IntrinsicFunctionIdentifierNode => IntrinsicFunctionIdentifier);
impl_into_storage!(LiteralOperatorIdentifierNode<'alloc> => LiteralOperatorIdentifier);
impl_into_storage!(LocalStaticGuardIdentifierNode => LocalStaticGuardIdentifier);
impl_into_storage!(ConversionOperatorIdentifierNode => ConversionOperatorIdentifier);
impl_into_storage!(StructorIdentifierNode => StructorIdentifier);
impl_into_storage!(RttiBaseClassDescriptorNode => RttiBaseClassDescriptor);

impl_into_storage!(NodeArrayNode<'alloc> => NodeArray);
impl_into_storage!(QualifiedNameNode => QualifiedName);
impl_into_storage!(TemplateParameterReferenceNode => TemplateParameterReference);
impl_into_storage!(IntegerLiteralNode => IntegerLiteral);

impl_into_storage!(Md5SymbolNode => Md5Symbol);
impl_into_storage!(SpecialTableSymbolNode => SpecialTableSymbol);
impl_into_storage!(LocalStaticGuardVariableNode => LocalStaticGuardVariable);
impl_into_storage!(EncodedStringLiteralNode<'alloc> => EncodedStringLiteral);
impl_into_storage!(VariableSymbolNode => VariableSymbol);
impl_into_storage!(FunctionSymbolNode => FunctionSymbol);

pub(crate) trait UnwrapStorage<'storage, 'alloc: 'storage> {
    type Output;
    type OutputMut;

    #[must_use]
    fn try_unwrap(this: &'storage NodeStorage<'alloc>) -> Option<Self::Output>;

    #[must_use]
    fn try_unwrap_mut(this: &'storage mut NodeStorage<'alloc>) -> Option<Self::OutputMut>;
}

macro_rules! impl_from_storage {
    ($t:ident) => {
        impl<'storage, 'alloc: 'storage> UnwrapStorage<'storage, 'alloc> for $t {
            type Output = &'storage <$t as ResolverToNode<'alloc>>::Node;
            type OutputMut = &'storage mut <$t as ResolverToNode<'alloc>>::Node;

            fn try_unwrap(this: &'storage NodeStorage<'alloc>) -> Option<Self::Output> {
                if let NodeStorage::$t(x) = this {
                    Some(x)
                } else {
                    None
                }
            }

            fn try_unwrap_mut(this: &'storage mut NodeStorage<'alloc>) -> Option<Self::OutputMut> {
                if let NodeStorage::$t(x) = this {
                    Some(x)
                } else {
                    None
                }
            }
        }
    };
}

impl_from_storage!(PrimitiveType);
impl_from_storage!(FunctionSignature);
impl_from_storage!(ThunkSignature);
impl_from_storage!(PointerType);
impl_from_storage!(TagType);
impl_from_storage!(ArrayType);
impl_from_storage!(CustomType);

impl_from_storage!(VcallThunkIdentifier);
impl_from_storage!(DynamicStructorIdentifier);
impl_from_storage!(NamedIdentifier);
impl_from_storage!(IntrinsicFunctionIdentifier);
impl_from_storage!(LiteralOperatorIdentifier);
impl_from_storage!(LocalStaticGuardIdentifier);
impl_from_storage!(ConversionOperatorIdentifier);
impl_from_storage!(StructorIdentifier);
impl_from_storage!(RttiBaseClassDescriptor);

impl_from_storage!(NodeArray);
impl_from_storage!(QualifiedName);
impl_from_storage!(TemplateParameterReference);
impl_from_storage!(IntegerLiteral);

impl_from_storage!(Md5Symbol);
impl_from_storage!(SpecialTableSymbol);
impl_from_storage!(LocalStaticGuardVariable);
impl_from_storage!(EncodedStringLiteral);
impl_from_storage!(VariableSymbol);
impl_from_storage!(FunctionSymbol);

macro_rules! impl_from_storage_interface {
	($interface:ident = [ $($variant:ident),+ $(,)? ]) => {
		impl<'storage, 'alloc: 'storage> UnwrapStorage<'storage, 'alloc> for $interface {
			type Output = <$interface as IntermediateNode<'storage, 'alloc>>::Const;
			type OutputMut = <$interface as IntermediateNode<'storage, 'alloc>>::Mut;

			fn try_unwrap(this: &'storage NodeStorage<'alloc>) -> Option<Self::Output> {
				match this {
					$(NodeStorage::$variant(x) => {
						let x: &_ = *x;
						Some(x.into())
					})+
					#[allow(unreachable_patterns)]
					_ => None,
				}
			}

			fn try_unwrap_mut(this: &'storage mut NodeStorage<'alloc>) -> Option<Self::OutputMut> {
				match this {
					$(NodeStorage::$variant(x) => Some((*x).into()),)+
					#[allow(unreachable_patterns)]
					_ => None,
				}
			}
		}
	};
}

impl_from_storage_interface!(
    INode = [
        PrimitiveType,
        FunctionSignature,
        ThunkSignature,
        PointerType,
        TagType,
        ArrayType,
        CustomType,
        VcallThunkIdentifier,
        DynamicStructorIdentifier,
        NamedIdentifier,
        IntrinsicFunctionIdentifier,
        LiteralOperatorIdentifier,
        LocalStaticGuardIdentifier,
        ConversionOperatorIdentifier,
        StructorIdentifier,
        RttiBaseClassDescriptor,
        NodeArray,
        QualifiedName,
        TemplateParameterReference,
        IntegerLiteral,
        Md5Symbol,
        SpecialTableSymbol,
        LocalStaticGuardVariable,
        EncodedStringLiteral,
        VariableSymbol,
        FunctionSymbol,
    ]
);

impl_from_storage_interface!(
    ITypeNode = [
        PrimitiveType,
        FunctionSignature,
        ThunkSignature,
        PointerType,
        TagType,
        ArrayType,
        CustomType,
    ]
);

impl_from_storage_interface!(ISignatureNode = [FunctionSignature, ThunkSignature]);

impl_from_storage_interface!(
    IIdentifierNode = [
        VcallThunkIdentifier,
        DynamicStructorIdentifier,
        NamedIdentifier,
        IntrinsicFunctionIdentifier,
        LiteralOperatorIdentifier,
        LocalStaticGuardIdentifier,
        ConversionOperatorIdentifier,
        StructorIdentifier,
        RttiBaseClassDescriptor,
    ]
);

impl_from_storage_interface!(
    ISymbolNode = [
        Md5Symbol,
        SpecialTableSymbol,
        LocalStaticGuardVariable,
        EncodedStringLiteral,
        VariableSymbol,
        FunctionSymbol,
    ]
);

pub(crate) struct NodeCache<'alloc> {
    storage: BumpVec<'alloc, NodeStorage<'alloc>>,
}

impl<'alloc> NodeCache<'alloc> {
    pub(crate) fn new(allocator: &'alloc Bump) -> Self {
        Self {
            storage: alloc::new_vec(allocator),
        }
    }

    pub(crate) fn intern<T>(
        &mut self,
        node: T,
    ) -> Result<NodeHandle<<T as NodeToResolver>::Resolver>>
    where
        T: NodeToResolver + 'alloc,
        &'alloc mut T: Into<NodeStorage<'alloc>>,
    {
        if self.storage.len() + 1 > (1 << 11) {
            // a mangled string with this many nodes is probably malformed... bail
            return Err(Error::MaliciousInput);
        }

        let allocator = self.storage.bump();
        let node = alloc::allocate(allocator, node);
        self.storage.push(node.into());
        let id = self.storage.len() - 1;
        // SAFETY: we would oom before allocating usize::MAX nodes
        let id = unsafe { NonMaxUsize::new_unchecked(id) };
        Ok(NodeHandle::new(id))
    }
}

#[repr(transparent)]
pub(crate) struct NodeHandle<T> {
    id: NonMaxUsize, // enables niche optimization
    marker: PhantomData<T>,
}

impl<T> NodeHandle<T> {
    fn new(id: NonMaxUsize) -> Self {
        Self {
            id,
            marker: PhantomData,
        }
    }

    pub(crate) fn resolve<'storage, 'alloc: 'storage>(
        self,
        cache: &'storage NodeCache<'alloc>,
    ) -> <T as UnwrapStorage<'storage, 'alloc>>::Output
    where
        T: UnwrapStorage<'storage, 'alloc>,
    {
        let node = &cache.storage[self.id.get()];
        T::try_unwrap(node).expect("actual node type does not match encoded type")
    }

    pub(crate) fn resolve_mut<'storage, 'alloc: 'storage>(
        self,
        cache: &'storage mut NodeCache<'alloc>,
    ) -> <T as UnwrapStorage<'storage, 'alloc>>::OutputMut
    where
        T: UnwrapStorage<'storage, 'alloc>,
    {
        let node = &mut cache.storage[self.id.get()];
        T::try_unwrap_mut(node).expect("actual node type does not match encoded type")
    }

    #[must_use]
    pub(crate) fn downcast<'storage, 'alloc: 'storage, To>(
        self,
        cache: &'storage NodeCache<'alloc>,
    ) -> Option<NodeHandle<To>>
    where
        Self: Downcast<To>,
    {
        <Self as Downcast<To>>::downcast(self, cache)
    }
}

impl<T> Clone for NodeHandle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for NodeHandle<T> {}

macro_rules! impl_upcast {
    ($from:ident => $to:ty) => {
        impl From<NodeHandle<$from>> for NodeHandle<$to> {
            fn from(value: NodeHandle<$from>) -> Self {
                Self::new(value.id)
            }
        }
    };
}

impl_upcast!(ITypeNode => INode);
impl_upcast!(ISignatureNode => INode);
impl_upcast!(PrimitiveType => INode);
impl_upcast!(FunctionSignature => INode);
impl_upcast!(ThunkSignature => INode);
impl_upcast!(PointerType => INode);
impl_upcast!(TagType => INode);
impl_upcast!(ArrayType => INode);
impl_upcast!(CustomType => INode);

impl_upcast!(IIdentifierNode => INode);
impl_upcast!(VcallThunkIdentifier => INode);
impl_upcast!(DynamicStructorIdentifier => INode);
impl_upcast!(NamedIdentifier => INode);
impl_upcast!(IntrinsicFunctionIdentifier => INode);
impl_upcast!(LiteralOperatorIdentifier => INode);
impl_upcast!(LocalStaticGuardIdentifier => INode);
impl_upcast!(ConversionOperatorIdentifier => INode);
impl_upcast!(StructorIdentifier => INode);
impl_upcast!(RttiBaseClassDescriptor => INode);

impl_upcast!(NodeArray => INode);
impl_upcast!(QualifiedName => INode);
impl_upcast!(TemplateParameterReference => INode);
impl_upcast!(IntegerLiteral => INode);

impl_upcast!(ISymbolNode => INode);
impl_upcast!(Md5Symbol => INode);
impl_upcast!(SpecialTableSymbol => INode);
impl_upcast!(LocalStaticGuardVariable => INode);
impl_upcast!(EncodedStringLiteral => INode);
impl_upcast!(VariableSymbol => INode);
impl_upcast!(FunctionSymbol => INode);

impl_upcast!(PrimitiveType => ITypeNode);
impl_upcast!(ISignatureNode => ITypeNode);
impl_upcast!(FunctionSignature => ITypeNode);
impl_upcast!(ThunkSignature => ITypeNode);
impl_upcast!(PointerType => ITypeNode);
impl_upcast!(TagType => ITypeNode);
impl_upcast!(ArrayType => ITypeNode);
impl_upcast!(CustomType => ITypeNode);

impl_upcast!(FunctionSignature => ISignatureNode);
impl_upcast!(ThunkSignature => ISignatureNode);

impl_upcast!(VcallThunkIdentifier => IIdentifierNode);
impl_upcast!(DynamicStructorIdentifier => IIdentifierNode);
impl_upcast!(NamedIdentifier => IIdentifierNode);
impl_upcast!(IntrinsicFunctionIdentifier => IIdentifierNode);
impl_upcast!(LiteralOperatorIdentifier => IIdentifierNode);
impl_upcast!(LocalStaticGuardIdentifier => IIdentifierNode);
impl_upcast!(ConversionOperatorIdentifier => IIdentifierNode);
impl_upcast!(StructorIdentifier => IIdentifierNode);
impl_upcast!(RttiBaseClassDescriptor => IIdentifierNode);

impl_upcast!(Md5Symbol => ISymbolNode);
impl_upcast!(SpecialTableSymbol => ISymbolNode);
impl_upcast!(LocalStaticGuardVariable => ISymbolNode);
impl_upcast!(EncodedStringLiteral => ISymbolNode);
impl_upcast!(VariableSymbol => ISymbolNode);
impl_upcast!(FunctionSymbol => ISymbolNode);

pub(crate) trait Downcast<To> {
    #[must_use]
    fn downcast<'storage, 'alloc: 'storage>(
        self,
        cache: &'storage NodeCache<'alloc>,
    ) -> Option<NodeHandle<To>>;
}

macro_rules! impl_downcast {
    ($for:ident, $from:ident::$variant:ident => $to:ty) => {
        impl<'node> Downcast<$to> for NodeHandle<$for> {
            fn downcast<'storage, 'alloc: 'storage>(
                self,
                cache: &'storage NodeCache<'alloc>,
            ) -> Option<NodeHandle<$to>> {
                if let $from::$variant(_) = self.resolve(cache) {
                    Some(NodeHandle::new(self.id))
                } else {
                    None
                }
            }
        }
    };
}

impl_downcast!(INode, Node::Type => ITypeNode);
impl_downcast!(INode, Node::Identifier => IIdentifierNode);
impl_downcast!(INode, Node::NodeArray => NodeArray);
impl_downcast!(INode, Node::QualifiedName => QualifiedName);
impl_downcast!(INode, Node::TemplateParameterReference => TemplateParameterReference);
impl_downcast!(INode, Node::IntegerLiteral => IntegerLiteral);
impl_downcast!(INode, Node::Symbol => ISymbolNode);

impl_downcast!(ITypeNode, TypeNode::PrimitiveType => PrimitiveType);
impl_downcast!(ITypeNode, TypeNode::Signature => ISignatureNode);
impl_downcast!(ITypeNode, TypeNode::PointerType => PointerType);
impl_downcast!(ITypeNode, TypeNode::TagType => TagType);
impl_downcast!(ITypeNode, TypeNode::ArrayType => ArrayType);
impl_downcast!(ITypeNode, TypeNode::CustomType => CustomType);

impl_downcast!(ISignatureNode, SignatureNode::FunctionSignature => FunctionSignature);
impl_downcast!(ISignatureNode, SignatureNode::ThunkSignature => ThunkSignature);

impl_downcast!(IIdentifierNode, IdentifierNode::VcallThunkIdentifier => VcallThunkIdentifier);
impl_downcast!(IIdentifierNode, IdentifierNode::DynamicStructorIdentifier => DynamicStructorIdentifier);
impl_downcast!(IIdentifierNode, IdentifierNode::NamedIdentifier => NamedIdentifier);
impl_downcast!(IIdentifierNode, IdentifierNode::IntrinsicFunctionIdentifier => IntrinsicFunctionIdentifier);
impl_downcast!(IIdentifierNode, IdentifierNode::LiteralOperatorIdentifier => LiteralOperatorIdentifier);
impl_downcast!(IIdentifierNode, IdentifierNode::LocalStaticGuardIdentifier => LocalStaticGuardIdentifier);
impl_downcast!(IIdentifierNode, IdentifierNode::ConversionOperatorIdentifier => ConversionOperatorIdentifier);
impl_downcast!(IIdentifierNode, IdentifierNode::StructorIdentifier => StructorIdentifier);
impl_downcast!(IIdentifierNode, IdentifierNode::RttiBaseClassDescriptor => RttiBaseClassDescriptor);

impl_downcast!(ISymbolNode, SymbolNode::SpecialTableSymbol => SpecialTableSymbol);
impl_downcast!(ISymbolNode, SymbolNode::LocalStaticGuardVariable => LocalStaticGuardVariable);
impl_downcast!(ISymbolNode, SymbolNode::EncodedStringLiteral => EncodedStringLiteral);
impl_downcast!(ISymbolNode, SymbolNode::VariableSymbol => VariableSymbol);
impl_downcast!(ISymbolNode, SymbolNode::FunctionSymbol => FunctionSymbol);

impl Downcast<ISignatureNode> for NodeHandle<INode> {
    fn downcast<'storage, 'alloc: 'storage>(
        self,
        cache: &'storage NodeCache<'alloc>,
    ) -> Option<NodeHandle<ISignatureNode>> {
        if let Node::Type(TypeNode::Signature(_)) = self.resolve(cache) {
            Some(NodeHandle::new(self.id))
        } else {
            None
        }
    }
}

impl Downcast<FunctionSignature> for NodeHandle<INode> {
    fn downcast<'storage, 'alloc: 'storage>(
        self,
        cache: &'storage NodeCache<'alloc>,
    ) -> Option<NodeHandle<FunctionSignature>> {
        if let Node::Type(TypeNode::Signature(SignatureNode::FunctionSignature(_))) =
            self.resolve(cache)
        {
            Some(NodeHandle::new(self.id))
        } else {
            None
        }
    }
}

impl Downcast<ThunkSignature> for NodeHandle<INode> {
    fn downcast<'storage, 'alloc: 'storage>(
        self,
        cache: &'storage NodeCache<'alloc>,
    ) -> Option<NodeHandle<ThunkSignature>> {
        if let Node::Type(TypeNode::Signature(SignatureNode::ThunkSignature(_))) =
            self.resolve(cache)
        {
            Some(NodeHandle::new(self.id))
        } else {
            None
        }
    }
}

pub(crate) trait ResolverToNode<'alloc> {
    type Node;
}

pub(crate) trait NodeToResolver {
    type Resolver;
}

macro_rules! impl_node_handle {
    ($resolver:ident => $node:ty) => {
        pub(crate) struct $resolver;

        impl<'alloc> ResolverToNode<'alloc> for $resolver {
            type Node = $node;
        }

        impl<'alloc> NodeToResolver for $node {
            type Resolver = $resolver;
        }
    };
}

impl_node_handle!(PrimitiveType => PrimitiveTypeNode);
impl_node_handle!(FunctionSignature => FunctionSignatureNode);
impl_node_handle!(ThunkSignature => ThunkSignatureNode);
impl_node_handle!(PointerType => PointerTypeNode);
impl_node_handle!(TagType => TagTypeNode);
impl_node_handle!(ArrayType => ArrayTypeNode);
impl_node_handle!(CustomType => CustomTypeNode);

impl_node_handle!(VcallThunkIdentifier => VcallThunkIdentifierNode);
impl_node_handle!(DynamicStructorIdentifier => DynamicStructorIdentifierNode);
impl_node_handle!(NamedIdentifier => NamedIdentifierNode<'alloc>);
impl_node_handle!(IntrinsicFunctionIdentifier => IntrinsicFunctionIdentifierNode);
impl_node_handle!(LiteralOperatorIdentifier => LiteralOperatorIdentifierNode<'alloc>);
impl_node_handle!(LocalStaticGuardIdentifier => LocalStaticGuardIdentifierNode);
impl_node_handle!(ConversionOperatorIdentifier => ConversionOperatorIdentifierNode);
impl_node_handle!(StructorIdentifier => StructorIdentifierNode);
impl_node_handle!(RttiBaseClassDescriptor => RttiBaseClassDescriptorNode);

impl_node_handle!(NodeArray => NodeArrayNode<'alloc>);
impl_node_handle!(QualifiedName => QualifiedNameNode);
impl_node_handle!(TemplateParameterReference => TemplateParameterReferenceNode);
impl_node_handle!(IntegerLiteral => IntegerLiteralNode);

impl_node_handle!(Md5Symbol => Md5SymbolNode);
impl_node_handle!(SpecialTableSymbol => SpecialTableSymbolNode);
impl_node_handle!(LocalStaticGuardVariable => LocalStaticGuardVariableNode);
impl_node_handle!(EncodedStringLiteral => EncodedStringLiteralNode<'alloc>);
impl_node_handle!(VariableSymbol => VariableSymbolNode);
impl_node_handle!(FunctionSymbol => FunctionSymbolNode);
