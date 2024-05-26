mod derived;
mod enums;
mod intermediate;

// Node*
// ├── TypeNode*
// │   ├── PrimitiveTypeNode
// │   ├── FunctionSignatureNode
// │   │   └── ThunkSignatureNode
// │   ├── PointerTypeNode
// │   ├── TagTypeNode
// │   ├── ArrayTypeNode
// │   ├── IntrinsicNode*
// │   └── CustomTypeNode
// ├── IdentifierNode*
// │   ├── VcallThunkIdentifierNode
// │   ├── DynamicStructorIdentifierNode
// │   ├── NamedIdentifierNode
// │   ├── IntrinsicFunctionIdentifierNode
// │   ├── LiteralOperatorIdentifierNode
// │   ├── LocalStaticGuardIdentifierNode
// │   ├── ConversionOperatorIdentifierNode
// │   ├── StructorIdentifierNode
// │   └── RttiBaseClassDescriptorNode
// ├── NodeArrayNode
// ├── QualifiedNameNode
// ├── TemplateParameterReferenceNode
// ├── IntegerLiteralNode
// └── SymbolNode
//     ├── SpecialTableSymbolNode
//     ├── LocalStaticGuardVariableNode
//     ├── EncodedStringLiteralNode
//     ├── VariableSymbolNode
//     └── FunctionSymbolNode

use crate::cache::NodeCache;
pub(crate) use derived::{
    ArrayTypeNode,
    ConversionOperatorIdentifierNode,
    CustomTypeNode,
    DynamicStructorIdentifierNode,
    EncodedStringLiteralNode,
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
    RttiBaseClassDescriptorNode,
    SpecialTableSymbolNode,
    StructorIdentifierNode,
    TagTypeNode,
    TemplateParameterReferenceNode,
    ThunkSignatureNode,
    VariableSymbolNode,
    VcallThunkIdentifierNode,
};
pub(crate) use enums::{
    CallingConv,
    CharKind,
    FuncClass,
    FunctionRefQualifier,
    IntrinsicFunctionKind,
    PointerAffinity,
    PrimitiveKind,
    Qualifiers,
    SpecialIntrinsicKind,
    StorageClass,
    TagKind,
};
pub(crate) use intermediate::{
    Downcast,
    IIdentifierNode,
    INode,
    ISignatureNode,
    ISymbolNode,
    ITypeNode,
    IdentifierNode,
    IntermediateNode,
    Node,
    SignatureNode,
    SymbolNode,
    TypeNode,
};
use std::io::{
    self,
    Write as _,
};

type Result<T> = std::result::Result<T, io::Error>;

pub trait DerivedNode {}

macro_rules! is_derived_node {
    ($t:ty) => {
        impl DerivedNode for $t {}
    };
}

is_derived_node!(PrimitiveTypeNode);
is_derived_node!(FunctionSignatureNode);
is_derived_node!(ThunkSignatureNode);
is_derived_node!(PointerTypeNode);
is_derived_node!(TagTypeNode);
is_derived_node!(ArrayTypeNode);
is_derived_node!(CustomTypeNode);

is_derived_node!(VcallThunkIdentifierNode);
is_derived_node!(DynamicStructorIdentifierNode);
is_derived_node!(NamedIdentifierNode);
is_derived_node!(IntrinsicFunctionIdentifierNode);
is_derived_node!(LiteralOperatorIdentifierNode);
is_derived_node!(LocalStaticGuardIdentifierNode);
is_derived_node!(ConversionOperatorIdentifierNode);
is_derived_node!(StructorIdentifierNode);
is_derived_node!(RttiBaseClassDescriptorNode);

is_derived_node!(NodeArrayNode);
is_derived_node!(QualifiedNameNode);
is_derived_node!(TemplateParameterReferenceNode);
is_derived_node!(IntegerLiteralNode);

is_derived_node!(SpecialTableSymbolNode);
is_derived_node!(LocalStaticGuardVariableNode);
is_derived_node!(EncodedStringLiteralNode);
is_derived_node!(VariableSymbolNode);
is_derived_node!(FunctionSymbolNode);

fn output_space_if_necessary(ob: &mut Vec<u8>) -> Result<()> {
    if let Some(c) = ob.last() {
        if c.is_ascii_alphanumeric() || *c == b'>' {
            write!(ob, " ")?;
        }
    }
    Ok(())
}

pub(crate) type OutputBuffer = Vec<u8>;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    pub(crate) struct OutputFlags: u8 {
        const OF_Default = 0;
        const OF_NoCallingConvention = 1;
        const OF_NoTagSpecifier = 2;
        const OF_NoAccessSpecifier = 4;
        const OF_NoMemberType = 8;
        const OF_NoReturnType = 16;
        const OF_NoVariableType = 32;
    }
}

impl OutputFlags {
    #[must_use]
    fn no_calling_convention(self) -> bool {
        self.contains(Self::OF_NoCallingConvention)
    }

    #[must_use]
    fn no_tag_specifier(self) -> bool {
        self.contains(Self::OF_NoTagSpecifier)
    }

    #[must_use]
    fn no_access_specifier(self) -> bool {
        self.contains(Self::OF_NoAccessSpecifier)
    }

    #[must_use]
    fn no_member_type(self) -> bool {
        self.contains(Self::OF_NoMemberType)
    }

    #[must_use]
    fn no_return_type(self) -> bool {
        self.contains(Self::OF_NoReturnType)
    }

    #[must_use]
    fn no_variable_type(self) -> bool {
        self.contains(Self::OF_NoVariableType)
    }
}

pub(crate) trait WriteableNode {
    fn output(&self, cache: &NodeCache, ob: &mut OutputBuffer, flags: OutputFlags) -> Result<()>;
}

trait WriteableTypeNode {
    fn output_pair(
        &self,
        cache: &NodeCache,
        ob: &mut OutputBuffer,
        flags: OutputFlags,
    ) -> Result<()> {
        self.output_pre(cache, ob, flags)?;
        self.output_post(cache, ob, flags)?;
        Ok(())
    }

    fn output_pre(
        &self,
        cache: &NodeCache,
        ob: &mut OutputBuffer,
        flags: OutputFlags,
    ) -> Result<()>;

    fn output_post(
        &self,
        cache: &NodeCache,
        ob: &mut OutputBuffer,
        flags: OutputFlags,
    ) -> Result<()>;
}
