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
    Buffer,
    OutputFlags,
    Result,
    Writer,
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
    VariableSymbolName,
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
use std::{
    io::Write as _,
    mem,
};

fn output_space_if_necessary<B: Buffer>(ob: &mut Writer<B>) -> Result<()> {
    if let Some(c) = ob.last() {
        if c.is_ascii_alphanumeric() || *c == b'>' {
            write!(ob, " ")?;
        }
    }
    Ok(())
}

pub(crate) trait WriteableNode {
    fn output<B: Buffer>(
        &self,
        cache: &NodeCache,
        ob: &mut Writer<B>,
        flags: OutputFlags,
    ) -> Result<()>;
}

trait WriteableTypeNode {
    fn output_pair<B: Buffer>(
        &self,
        cache: &NodeCache,
        ob: &mut Writer<B>,
        flags: OutputFlags,
    ) -> Result<()> {
        self.output_pre(cache, ob, flags)?;
        self.output_post(cache, ob, flags)?;
        Ok(())
    }

    fn output_pre<B: Buffer>(
        &self,
        cache: &NodeCache,
        ob: &mut Writer<B>,
        flags: OutputFlags,
    ) -> Result<()>;

    fn output_post<B: Buffer>(
        &self,
        cache: &NodeCache,
        ob: &mut Writer<B>,
        flags: OutputFlags,
    ) -> Result<()>;
}

macro_rules! assert_trivial_drop {
    ($t:ty) => {
        const _: () = assert!(!mem::needs_drop::<$t>());
    };
}

assert_trivial_drop!(PrimitiveTypeNode);
assert_trivial_drop!(FunctionSignatureNode);
assert_trivial_drop!(ThunkSignatureNode);
assert_trivial_drop!(PointerTypeNode);
assert_trivial_drop!(TagTypeNode);
assert_trivial_drop!(ArrayTypeNode);
assert_trivial_drop!(CustomTypeNode);

assert_trivial_drop!(VcallThunkIdentifierNode);
assert_trivial_drop!(DynamicStructorIdentifierNode);
assert_trivial_drop!(NamedIdentifierNode);
assert_trivial_drop!(IntrinsicFunctionIdentifierNode);
assert_trivial_drop!(LiteralOperatorIdentifierNode);
assert_trivial_drop!(LocalStaticGuardIdentifierNode);
assert_trivial_drop!(ConversionOperatorIdentifierNode);
assert_trivial_drop!(StructorIdentifierNode);
assert_trivial_drop!(RttiBaseClassDescriptorNode);

assert_trivial_drop!(NodeArrayNode);
assert_trivial_drop!(QualifiedNameNode);
assert_trivial_drop!(TemplateParameterReferenceNode);
assert_trivial_drop!(IntegerLiteralNode);

assert_trivial_drop!(SpecialTableSymbolNode);
assert_trivial_drop!(LocalStaticGuardVariableNode);
assert_trivial_drop!(EncodedStringLiteralNode);
assert_trivial_drop!(VariableSymbolNode);
assert_trivial_drop!(FunctionSymbolNode);
