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

use crate::{
    cache::NodeCache,
    OutputBuffer,
    OutputFlags,
};
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
    TemplateParameters,
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
