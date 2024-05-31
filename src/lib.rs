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

#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::similar_names,
    clippy::struct_field_names,
    clippy::too_many_lines
)]

mod cache;
mod demangler;
mod extensions;
mod nodes;

#[cfg(test)]
mod tests;

use crate::demangler::Demangler;
pub use bstr::{
    BStr,
    BString,
    ByteSlice,
    ByteVec,
};
use bumpalo::{
    collections::Vec as BumpVec,
    Bump,
};
use std::{
    io::{
        self,
        Write,
    },
    mem,
};

type OutputFlags = Flags;

trait Writer: Write {
    fn last(&self) -> Option<&u8>;
    fn len(&self) -> usize;
}

impl Writer for Vec<u8> {
    fn last(&self) -> Option<&u8> {
        self.as_slice().last()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<'bump> Writer for BumpVec<'bump, u8> {
    fn last(&self) -> Option<&u8> {
        self.as_slice().last()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

#[derive(Default)]
struct Allocator {
    alloc: Bump,
}

impl Allocator {
    pub(crate) fn allocate<T>(&self, val: T) -> &mut T {
        debug_assert!(!mem::needs_drop::<T>());
        self.alloc.alloc(val)
    }

    pub(crate) fn allocate_slice<'this, T>(&'this self, src: &[T]) -> &'this [T]
    where
        T: Copy,
    {
        debug_assert!(!mem::needs_drop::<T>());
        self.alloc.alloc_slice_copy(src)
    }

    pub(crate) fn new_vec<'this>(&'this self) -> BumpVec<'this, u8> {
        BumpVec::new_in(&self.alloc)
    }
}

#[non_exhaustive]
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

bitflags::bitflags! {
    /// `Flags` control how types are printed during demangling. See each flag for more info on what exactly they do.
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    pub struct Flags: u8 {
        /// Suppress calling conventions (`__cdecl`/`__fastcall`/`__thiscall`) from being included in the output.
        /// ```rust
        /// use undname::Flags;
        /// let input = b"?func@MyClass@@UEAAHHH@Z".into();
        /// let without_flag = undname::demangle(input, Flags::default()).unwrap();
        /// let with_flag = undname::demangle(input, Flags::NO_CALLING_CONVENTION).unwrap();
        /// assert_eq!(without_flag, b"public: virtual int __cdecl MyClass::func(int, int)"[..]);
        /// assert_eq!(with_flag,    b"public: virtual int MyClass::func(int, int)"[..]);
        /// ```
        const NO_CALLING_CONVENTION = 1 << 0;

        /// Suppress tag specifiers (`class`/`struct`/`union`) from being included in the output.
        /// ```rust
        /// use undname::Flags;
        /// let input = b"?x@@3PEAVty@@EA".into();
        /// let without_flag = undname::demangle(input, Flags::default()).unwrap();
        /// let with_flag = undname::demangle(input, Flags::NO_TAG_SPECIFIER).unwrap();
        /// assert_eq!(without_flag, b"class ty *x"[..]);
        /// assert_eq!(with_flag,    b"ty *x"[..]);
        /// ```
        const NO_TAG_SPECIFIER = 1 << 1;

        /// Suppress access specifiers (`private`/`public`/`protected`) from being included in the output.
        /// ```rust
        /// use undname::Flags;
        /// let input = b"?func@MyClass@@UEAAHHH@Z".into();
        /// let without_flag = undname::demangle(input, Flags::default()).unwrap();
        /// let with_flag = undname::demangle(input, Flags::NO_ACCESS_SPECIFIER).unwrap();
        /// assert_eq!(without_flag, b"public: virtual int __cdecl MyClass::func(int, int)"[..]);
        /// assert_eq!(with_flag,    b"virtual int __cdecl MyClass::func(int, int)"[..]);
        /// ```
        const NO_ACCESS_SPECIFIER = 1 << 2;

        /// Suppress member types (`static`/`virtual`/`extern "C"`) from being included in the output.
        /// ```rust
        /// use undname::Flags;
        /// let input = b"?func@MyClass@@UEAAHHH@Z".into();
        /// let without_flag = undname::demangle(input, Flags::default()).unwrap();
        /// let with_flag = undname::demangle(input, Flags::NO_MEMBER_TYPE).unwrap();
        /// assert_eq!(without_flag, b"public: virtual int __cdecl MyClass::func(int, int)"[..]);
        /// assert_eq!(with_flag,    b"public: int __cdecl MyClass::func(int, int)"[..]);
        /// ```
        const NO_MEMBER_TYPE = 1 << 3;

        /// Suppress return types from being included in the output.
        /// ```rust
        /// use undname::Flags;
        /// let input = b"?func@MyClass@@UEAAHHH@Z".into();
        /// let without_flag = undname::demangle(input, Flags::default()).unwrap();
        /// let with_flag = undname::demangle(input, Flags::NO_RETURN_TYPE).unwrap();
        /// assert_eq!(without_flag, b"public: virtual int __cdecl MyClass::func(int, int)"[..]);
        /// assert_eq!(with_flag,    b"public: virtual __cdecl MyClass::func(int, int)"[..]);
        /// ```
        const NO_RETURN_TYPE = 1 << 4;

        /// Suppress variable types from being included in the output.
        /// ```rust
        /// use undname::Flags;
        /// let input = b"?x@@3PEAEEA".into();
        /// let without_flag = undname::demangle(input, Flags::default()).unwrap();
        /// let with_flag = undname::demangle(input, Flags::NO_VARIABLE_TYPE).unwrap();
        /// assert_eq!(without_flag, b"unsigned char *x"[..]);
        /// assert_eq!(with_flag,    b"x"[..]);
        /// ```
        const NO_VARIABLE_TYPE = 1 << 5;
    }
}

impl Flags {
    #[must_use]
    fn no_calling_convention(self) -> bool {
        self.contains(Self::NO_CALLING_CONVENTION)
    }

    #[must_use]
    fn no_tag_specifier(self) -> bool {
        self.contains(Self::NO_TAG_SPECIFIER)
    }

    #[must_use]
    fn no_access_specifier(self) -> bool {
        self.contains(Self::NO_ACCESS_SPECIFIER)
    }

    #[must_use]
    fn no_member_type(self) -> bool {
        self.contains(Self::NO_MEMBER_TYPE)
    }

    #[must_use]
    fn no_return_type(self) -> bool {
        self.contains(Self::NO_RETURN_TYPE)
    }

    #[must_use]
    fn no_variable_type(self) -> bool {
        self.contains(Self::NO_VARIABLE_TYPE)
    }
}

/// Demangles a Microsoft symbol stored in `mangled_name`.
pub fn demangle(mangled_name: &BStr, flags: Flags) -> Result<BString> {
    let mut result = BString::default();
    demangle_into(mangled_name, flags, &mut result)?;
    Ok(result)
}

/// Demangles a Microsoft symbol stored in `mangled_name` into `result`.
pub fn demangle_into(mangled_name: &BStr, flags: Flags, result: &mut BString) -> Result<()> {
    let mut d = Demangler::default();
    let alloc = Allocator::default();
    d.parse_into(&alloc, mangled_name, flags, result)
}
