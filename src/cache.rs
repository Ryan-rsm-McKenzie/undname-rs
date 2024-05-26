use crate::nodes::{
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
};
use nonmax::NonMaxUsize;
use std::marker::PhantomData;

pub(crate) enum NodeStorage {
    PrimitiveType(Box<PrimitiveTypeNode>),
    FunctionSignature(Box<FunctionSignatureNode>),
    ThunkSignature(Box<ThunkSignatureNode>),
    PointerType(Box<PointerTypeNode>),
    TagType(Box<TagTypeNode>),
    ArrayType(Box<ArrayTypeNode>),
    CustomType(Box<CustomTypeNode>),

    VcallThunkIdentifier(Box<VcallThunkIdentifierNode>),
    DynamicStructorIdentifier(Box<DynamicStructorIdentifierNode>),
    NamedIdentifier(Box<NamedIdentifierNode>),
    IntrinsicFunctionIdentifier(Box<IntrinsicFunctionIdentifierNode>),
    LiteralOperatorIdentifier(Box<LiteralOperatorIdentifierNode>),
    LocalStaticGuardIdentifier(Box<LocalStaticGuardIdentifierNode>),
    ConversionOperatorIdentifier(Box<ConversionOperatorIdentifierNode>),
    StructorIdentifier(Box<StructorIdentifierNode>),
    RttiBaseClassDescriptor(Box<RttiBaseClassDescriptorNode>),

    NodeArray(Box<NodeArrayNode>),
    QualifiedName(Box<QualifiedNameNode>),
    TemplateParameterReference(Box<TemplateParameterReferenceNode>),
    IntegerLiteral(Box<IntegerLiteralNode>),

    Md5Symbol(Box<Md5SymbolNode>),
    SpecialTableSymbol(Box<SpecialTableSymbolNode>),
    LocalStaticGuardVariable(Box<LocalStaticGuardVariableNode>),
    EncodedStringLiteral(Box<EncodedStringLiteralNode>),
    VariableSymbol(Box<VariableSymbolNode>),
    FunctionSymbol(Box<FunctionSymbolNode>),
}

macro_rules! impl_into_storage {
    ($from:ident => $to:ident) => {
        impl From<$from> for NodeStorage {
            fn from(value: $from) -> Self {
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
impl_into_storage!(NamedIdentifierNode => NamedIdentifier);
impl_into_storage!(IntrinsicFunctionIdentifierNode => IntrinsicFunctionIdentifier);
impl_into_storage!(LiteralOperatorIdentifierNode => LiteralOperatorIdentifier);
impl_into_storage!(LocalStaticGuardIdentifierNode => LocalStaticGuardIdentifier);
impl_into_storage!(ConversionOperatorIdentifierNode => ConversionOperatorIdentifier);
impl_into_storage!(StructorIdentifierNode => StructorIdentifier);
impl_into_storage!(RttiBaseClassDescriptorNode => RttiBaseClassDescriptor);

impl_into_storage!(NodeArrayNode => NodeArray);
impl_into_storage!(QualifiedNameNode => QualifiedName);
impl_into_storage!(TemplateParameterReferenceNode => TemplateParameterReference);
impl_into_storage!(IntegerLiteralNode => IntegerLiteral);

impl_into_storage!(Md5SymbolNode => Md5Symbol);
impl_into_storage!(SpecialTableSymbolNode => SpecialTableSymbol);
impl_into_storage!(LocalStaticGuardVariableNode => LocalStaticGuardVariable);
impl_into_storage!(EncodedStringLiteralNode => EncodedStringLiteral);
impl_into_storage!(VariableSymbolNode => VariableSymbol);
impl_into_storage!(FunctionSymbolNode => FunctionSymbol);

pub(crate) trait UnwrapStorage<'node> {
    type Output;
    type OutputMut;

    #[must_use]
    fn try_unwrap(this: &'node NodeStorage) -> Option<Self::Output>;

    #[must_use]
    fn try_unwrap_mut(this: &'node mut NodeStorage) -> Option<Self::OutputMut>;
}

macro_rules! impl_from_storage {
    ($from:ident => $to:ident) => {
        impl<'node> UnwrapStorage<'node> for $to {
            type Output = &'node $to;
            type OutputMut = &'node mut $to;

            fn try_unwrap(this: &'node NodeStorage) -> Option<Self::Output> {
                if let NodeStorage::$from(x) = this {
                    Some(x)
                } else {
                    None
                }
            }

            fn try_unwrap_mut(this: &'node mut NodeStorage) -> Option<Self::OutputMut> {
                if let NodeStorage::$from(x) = this {
                    Some(x)
                } else {
                    None
                }
            }
        }
    };
}

impl_from_storage!(PrimitiveType => PrimitiveTypeNode);
impl_from_storage!(FunctionSignature => FunctionSignatureNode);
impl_from_storage!(ThunkSignature => ThunkSignatureNode);
impl_from_storage!(PointerType => PointerTypeNode);
impl_from_storage!(TagType => TagTypeNode);
impl_from_storage!(ArrayType => ArrayTypeNode);
impl_from_storage!(CustomType => CustomTypeNode);

impl_from_storage!(VcallThunkIdentifier => VcallThunkIdentifierNode);
impl_from_storage!(DynamicStructorIdentifier => DynamicStructorIdentifierNode);
impl_from_storage!(NamedIdentifier => NamedIdentifierNode);
impl_from_storage!(IntrinsicFunctionIdentifier => IntrinsicFunctionIdentifierNode);
impl_from_storage!(LiteralOperatorIdentifier => LiteralOperatorIdentifierNode);
impl_from_storage!(LocalStaticGuardIdentifier => LocalStaticGuardIdentifierNode);
impl_from_storage!(ConversionOperatorIdentifier => ConversionOperatorIdentifierNode);
impl_from_storage!(StructorIdentifier => StructorIdentifierNode);
impl_from_storage!(RttiBaseClassDescriptor => RttiBaseClassDescriptorNode);

impl_from_storage!(NodeArray => NodeArrayNode);
impl_from_storage!(QualifiedName => QualifiedNameNode);
impl_from_storage!(TemplateParameterReference => TemplateParameterReferenceNode);
impl_from_storage!(IntegerLiteral => IntegerLiteralNode);

impl_from_storage!(Md5Symbol => Md5SymbolNode);
impl_from_storage!(SpecialTableSymbol => SpecialTableSymbolNode);
impl_from_storage!(LocalStaticGuardVariable => LocalStaticGuardVariableNode);
impl_from_storage!(EncodedStringLiteral => EncodedStringLiteralNode);
impl_from_storage!(VariableSymbol => VariableSymbolNode);
impl_from_storage!(FunctionSymbol => FunctionSymbolNode);

macro_rules! impl_from_storage_interface {
	($interface:ident = [ $($variant:ident),+ $(,)? ]) => {
		impl<'node> UnwrapStorage<'node> for $interface {
			type Output = <$interface as IntermediateNode<'node>>::Const;
			type OutputMut = <$interface as IntermediateNode<'node>>::Mut;

			fn try_unwrap(this: &'node NodeStorage) -> Option<Self::Output> {
				match this {
					$(NodeStorage::$variant(x) => Some(x.as_ref().into()),)+
					#[allow(unreachable_patterns)]
					_ => None,
				}
			}

			fn try_unwrap_mut(this: &'node mut NodeStorage) -> Option<Self::OutputMut> {
				match this {
					$(NodeStorage::$variant(x) => Some(x.as_mut().into()),)+
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

#[derive(Default)]
pub(crate) struct NodeCache {
    storage: Vec<NodeStorage>,
}

impl NodeCache {
    pub(crate) fn intern<T>(&mut self, node: T) -> NodeHandle<T>
    where
        T: Into<NodeStorage>,
    {
        self.storage.push(node.into());
        let id = self.storage.len() - 1;
        // SAFETY: we would oom before allocating usize::MAX nodes
        let id = unsafe { NonMaxUsize::new_unchecked(id) };
        NodeHandle::new(id)
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
            marker: PhantomData::default(),
        }
    }

    pub(crate) fn resolve<'cache>(
        self,
        cache: &'cache NodeCache,
    ) -> <T as UnwrapStorage<'cache>>::Output
    where
        T: UnwrapStorage<'cache>,
    {
        let node = &cache.storage[self.id.get()];
        T::try_unwrap(node).expect("actual node type does not match encoded type")
    }

    pub(crate) fn resolve_mut<'cache>(
        self,
        cache: &'cache mut NodeCache,
    ) -> <T as UnwrapStorage<'cache>>::OutputMut
    where
        T: UnwrapStorage<'cache>,
    {
        let node = &mut cache.storage[self.id.get()];
        T::try_unwrap_mut(node).expect("actual node type does not match encoded type")
    }

    #[must_use]
    pub(crate) fn downcast<To>(self, cache: &NodeCache) -> Option<NodeHandle<To>>
    where
        Self: Downcast<To>,
    {
        <Self as Downcast<To>>::downcast(self, cache)
    }
}

impl<T> Clone for NodeHandle<T> {
    fn clone(&self) -> Self {
        Self::new(self.id)
    }
}

impl<T> Copy for NodeHandle<T> {}

macro_rules! impl_upcast {
    ($from:ty => $to:ty) => {
        impl From<NodeHandle<$from>> for NodeHandle<$to> {
            fn from(value: NodeHandle<$from>) -> Self {
                Self::new(value.id)
            }
        }
    };
}

impl_upcast!(ITypeNode => INode);
impl_upcast!(ISignatureNode => INode);
impl_upcast!(PrimitiveTypeNode => INode);
impl_upcast!(FunctionSignatureNode => INode);
impl_upcast!(ThunkSignatureNode => INode);
impl_upcast!(PointerTypeNode => INode);
impl_upcast!(TagTypeNode => INode);
impl_upcast!(ArrayTypeNode => INode);
impl_upcast!(CustomTypeNode => INode);

impl_upcast!(IIdentifierNode => INode);
impl_upcast!(VcallThunkIdentifierNode => INode);
impl_upcast!(DynamicStructorIdentifierNode => INode);
impl_upcast!(NamedIdentifierNode => INode);
impl_upcast!(IntrinsicFunctionIdentifierNode => INode);
impl_upcast!(LiteralOperatorIdentifierNode => INode);
impl_upcast!(LocalStaticGuardIdentifierNode => INode);
impl_upcast!(ConversionOperatorIdentifierNode => INode);
impl_upcast!(StructorIdentifierNode => INode);
impl_upcast!(RttiBaseClassDescriptorNode => INode);

impl_upcast!(NodeArrayNode => INode);
impl_upcast!(QualifiedNameNode => INode);
impl_upcast!(TemplateParameterReferenceNode => INode);
impl_upcast!(IntegerLiteralNode => INode);

impl_upcast!(ISymbolNode => INode);
impl_upcast!(Md5SymbolNode => INode);
impl_upcast!(SpecialTableSymbolNode => INode);
impl_upcast!(LocalStaticGuardVariableNode => INode);
impl_upcast!(EncodedStringLiteralNode => INode);
impl_upcast!(VariableSymbolNode => INode);
impl_upcast!(FunctionSymbolNode => INode);

impl_upcast!(PrimitiveTypeNode => ITypeNode);
impl_upcast!(ISignatureNode => ITypeNode);
impl_upcast!(FunctionSignatureNode => ITypeNode);
impl_upcast!(ThunkSignatureNode => ITypeNode);
impl_upcast!(PointerTypeNode => ITypeNode);
impl_upcast!(TagTypeNode => ITypeNode);
impl_upcast!(ArrayTypeNode => ITypeNode);
impl_upcast!(CustomTypeNode => ITypeNode);

impl_upcast!(FunctionSignatureNode => ISignatureNode);
impl_upcast!(ThunkSignatureNode => ISignatureNode);

impl_upcast!(VcallThunkIdentifierNode => IIdentifierNode);
impl_upcast!(DynamicStructorIdentifierNode => IIdentifierNode);
impl_upcast!(NamedIdentifierNode => IIdentifierNode);
impl_upcast!(IntrinsicFunctionIdentifierNode => IIdentifierNode);
impl_upcast!(LiteralOperatorIdentifierNode => IIdentifierNode);
impl_upcast!(LocalStaticGuardIdentifierNode => IIdentifierNode);
impl_upcast!(ConversionOperatorIdentifierNode => IIdentifierNode);
impl_upcast!(StructorIdentifierNode => IIdentifierNode);
impl_upcast!(RttiBaseClassDescriptorNode => IIdentifierNode);

impl_upcast!(Md5SymbolNode => ISymbolNode);
impl_upcast!(SpecialTableSymbolNode => ISymbolNode);
impl_upcast!(LocalStaticGuardVariableNode => ISymbolNode);
impl_upcast!(EncodedStringLiteralNode => ISymbolNode);
impl_upcast!(VariableSymbolNode => ISymbolNode);
impl_upcast!(FunctionSymbolNode => ISymbolNode);

pub(crate) trait Downcast<To> {
    #[must_use]
    fn downcast(self, cache: &NodeCache) -> Option<NodeHandle<To>>;
}

macro_rules! impl_downcast {
    ($for:ident, $from:ident::$variant:ident => $to:ident) => {
        impl Downcast<$to> for NodeHandle<$for> {
            fn downcast(self, cache: &NodeCache) -> Option<NodeHandle<$to>> {
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
impl_downcast!(INode, Node::NodeArray => NodeArrayNode);
impl_downcast!(INode, Node::QualifiedName => QualifiedNameNode);
impl_downcast!(INode, Node::TemplateParameterReference => TemplateParameterReferenceNode);
impl_downcast!(INode, Node::IntegerLiteral => IntegerLiteralNode);
impl_downcast!(INode, Node::Symbol => ISymbolNode);

impl_downcast!(ITypeNode, TypeNode::PrimitiveType => PrimitiveTypeNode);
impl_downcast!(ITypeNode, TypeNode::Signature => ISignatureNode);
impl_downcast!(ITypeNode, TypeNode::PointerType => PointerTypeNode);
impl_downcast!(ITypeNode, TypeNode::TagType => TagTypeNode);
impl_downcast!(ITypeNode, TypeNode::ArrayType => ArrayTypeNode);
impl_downcast!(ITypeNode, TypeNode::CustomType => CustomTypeNode);

impl_downcast!(ISignatureNode, SignatureNode::FunctionSignature => FunctionSignatureNode);
impl_downcast!(ISignatureNode, SignatureNode::ThunkSignature => ThunkSignatureNode);

impl_downcast!(IIdentifierNode, IdentifierNode::VcallThunkIdentifier => VcallThunkIdentifierNode);
impl_downcast!(IIdentifierNode, IdentifierNode::DynamicStructorIdentifier => DynamicStructorIdentifierNode);
impl_downcast!(IIdentifierNode, IdentifierNode::NamedIdentifier => NamedIdentifierNode);
impl_downcast!(IIdentifierNode, IdentifierNode::IntrinsicFunctionIdentifier => IntrinsicFunctionIdentifierNode);
impl_downcast!(IIdentifierNode, IdentifierNode::LiteralOperatorIdentifier => LiteralOperatorIdentifierNode);
impl_downcast!(IIdentifierNode, IdentifierNode::LocalStaticGuardIdentifier => LocalStaticGuardIdentifierNode);
impl_downcast!(IIdentifierNode, IdentifierNode::ConversionOperatorIdentifier => ConversionOperatorIdentifierNode);
impl_downcast!(IIdentifierNode, IdentifierNode::StructorIdentifier => StructorIdentifierNode);
impl_downcast!(IIdentifierNode, IdentifierNode::RttiBaseClassDescriptor => RttiBaseClassDescriptorNode);

impl_downcast!(ISymbolNode, SymbolNode::SpecialTableSymbol => SpecialTableSymbolNode);
impl_downcast!(ISymbolNode, SymbolNode::LocalStaticGuardVariable => LocalStaticGuardVariableNode);
impl_downcast!(ISymbolNode, SymbolNode::EncodedStringLiteral => EncodedStringLiteralNode);
impl_downcast!(ISymbolNode, SymbolNode::VariableSymbol => VariableSymbolNode);
impl_downcast!(ISymbolNode, SymbolNode::FunctionSymbol => FunctionSymbolNode);

impl Downcast<ISignatureNode> for NodeHandle<INode> {
    fn downcast(self, cache: &NodeCache) -> Option<NodeHandle<ISignatureNode>> {
        if let Node::Type(outer) = self.resolve(cache) {
            if let TypeNode::Signature(_) = outer {
                return Some(NodeHandle::new(self.id));
            }
        }
        None
    }
}

impl Downcast<FunctionSignatureNode> for NodeHandle<INode> {
    fn downcast(self, cache: &NodeCache) -> Option<NodeHandle<FunctionSignatureNode>> {
        if let Node::Type(outer) = self.resolve(cache) {
            if let TypeNode::Signature(inner) = outer {
                if let SignatureNode::FunctionSignature(_) = inner {
                    return Some(NodeHandle::new(self.id));
                }
            }
        }
        None
    }
}

impl Downcast<ThunkSignatureNode> for NodeHandle<INode> {
    fn downcast(self, cache: &NodeCache) -> Option<NodeHandle<ThunkSignatureNode>> {
        if let Node::Type(outer) = self.resolve(cache) {
            if let TypeNode::Signature(inner) = outer {
                if let SignatureNode::ThunkSignature(_) = inner {
                    return Some(NodeHandle::new(self.id));
                }
            }
        }
        None
    }
}
