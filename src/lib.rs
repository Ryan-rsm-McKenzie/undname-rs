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

mod cache;
mod demangler;
mod extensions;
mod nodes;

use crate::{
    demangler::Demangler,
    nodes::OutputFlags,
};
pub use bstr::{
    BStr,
    BString,
    ByteSlice,
    ByteVec,
};
use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to demangle anonymous namespace name")]
    InvalidAnonymousNamespaceName,

    #[error("failed to demangle array type")]
    InvalidArrayType,

    #[error("tried to access a backref that does not exist")]
    InvalidBackRef,

    #[error("failed to demangle calling convention")]
    InvalidCallingConvention,

    #[error("failed to demangle char literal")]
    InvalidCharLiteral,

    #[error("failed to demangle class type")]
    InvalidClassType,

    #[error("failed to demangle custom type")]
    InvalidCustomType,

    #[error("failed to demangle declarator")]
    InvalidDeclarator,

    #[error("failed to demangle encoded symbol")]
    InvalidEncodedSymbol,

    #[error("failed to demangle fully qualified symbol name")]
    InvalidFullyQualifiedSymbolName,

    #[error("failed to demangle function class")]
    InvalidFunctionClass,

    #[error("failed to demangle function encoding")]
    InvalidFunctionEncoding,

    #[error("failed to demangle function identifier code")]
    InvalidFunctionIdentifierCode,

    #[error("failed to demangle function parameter list")]
    InvalidFunctionParameterList,

    #[error("failed to demangle function type")]
    InvalidFunctionType,

    #[error("failed to demangle init fini stub")]
    InvalidInitFiniStub,

    #[error("failed to demangle intrinsic function code")]
    InvalidIntrinsicFunctionCode,

    #[error("failed to demangle locally scoped name piece")]
    InvalidLocallyScopedNamePiece,

    #[error("failed to demangle local static guard")]
    InvalidLocalStaticGuard,

    #[error("failed to demangle md5 name")]
    InvalidMd5Name,

    #[error("failed to demangle member pointer type")]
    InvalidMemberPointerType,

    #[error("failed to demangle name scope chain")]
    InvalidNameScopeChain,

    #[error("failed to demangle number")]
    InvalidNumber,

    #[error("failed to demangle pointer cv qualifiers")]
    InvalidPointerCVQualifiers,

    #[error("failed to demangle pointer type")]
    InvalidPointerType,

    #[error("failed to demangle primitive type")]
    InvalidPrimitiveType,

    #[error("failed to demangle qualifiers")]
    InvalidQualifiers,

    #[error("failed to demangle rtti base class descriptor node")]
    InvalidRttiBaseClassDescriptorNode,

    #[error("failed to demangle signed number")]
    InvalidSigned,

    #[error("failed to demangle simple string")]
    InvalidSimpleString,

    #[error("failed to demangle special intrinsic")]
    InvalidSpecialIntrinsic,

    #[error("failed to demangle special table symbol node")]
    InvalidSpecialTableSymbolNode,

    #[error("failed to demangle string literal")]
    InvalidStringLiteral,

    #[error("failed to demangle tag unique name")]
    InvalidTagUniqueName,

    #[error("failed to demangle template instantiation name")]
    InvalidTemplateInstantiationName,

    #[error("failed to demangle template parameter list")]
    InvalidTemplateParameterList,

    #[error("failed to demangle throw specification")]
    InvalidThrowSpecification,

    #[error("failed to demangle type")]
    InvalidType,

    #[error("failed to demangle typinfo name")]
    InvalidTypeinfoName,

    #[error("failed to demangle unsigned number")]
    InvalidUnsigned,

    #[error("failed to demangle untyped variable")]
    InvalidUntypedVariable,

    #[error("failed to demangle variable storage class")]
    InvalidVariableStorageClass,

    #[error("failed to demangle vcall thunk node")]
    InvalidVcallThunkNode,

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("tried to save too many backrefs")]
    TooManyBackRefs,
}

pub type Result<T> = std::result::Result<T, Error>;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Flags(OutputFlags);

impl Flags {
    #[must_use]
    pub const fn no_access_specifier(self, value: bool) -> Self {
        if value {
            Self(self.0.intersection(OutputFlags::OF_NoAccessSpecifier))
        } else {
            Self(self.0.difference(OutputFlags::OF_NoAccessSpecifier))
        }
    }

    #[must_use]
    pub const fn no_calling_convention(self, value: bool) -> Self {
        if value {
            Self(self.0.intersection(OutputFlags::OF_NoCallingConvention))
        } else {
            Self(self.0.difference(OutputFlags::OF_NoCallingConvention))
        }
    }

    #[must_use]
    pub const fn no_member_type(self, value: bool) -> Self {
        if value {
            Self(self.0.intersection(OutputFlags::OF_NoMemberType))
        } else {
            Self(self.0.difference(OutputFlags::OF_NoMemberType))
        }
    }

    #[must_use]
    pub const fn no_return_type(self, value: bool) -> Self {
        if value {
            Self(self.0.intersection(OutputFlags::OF_NoReturnType))
        } else {
            Self(self.0.difference(OutputFlags::OF_NoReturnType))
        }
    }

    #[must_use]
    pub const fn no_tag_specifier(self, value: bool) -> Self {
        if value {
            Self(self.0.intersection(OutputFlags::OF_NoTagSpecifier))
        } else {
            Self(self.0.difference(OutputFlags::OF_NoTagSpecifier))
        }
    }

    #[must_use]
    pub const fn no_variable_type(self, value: bool) -> Self {
        if value {
            Self(self.0.intersection(OutputFlags::OF_NoVariableType))
        } else {
            Self(self.0.difference(OutputFlags::OF_NoVariableType))
        }
    }
}

pub fn demangle(mangled_name: &BStr, flags: Flags) -> Result<BString> {
    let mut d = Demangler::default();
    d.parse(mangled_name, flags.0)
}
