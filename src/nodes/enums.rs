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
    nodes::Result,
    Writer,
};
use std::io;

bitflags::bitflags! {
    // Storage classes
    #[derive(Clone, Copy, Default, Eq, PartialEq)]
    pub(crate) struct Qualifiers: u8 {
        const Q_None = 0;
        const Q_Const = 1 << 0;
        const Q_Volatile = 1 << 1;
        const Q_Far = 1 << 2;
        const Q_Huge = 1 << 3;
        const Q_Unaligned = 1 << 4;
        const Q_Restrict = 1 << 5;
        const Q_Pointer64 = 1 << 6;
    }
}

impl Qualifiers {
    #[must_use]
    pub(super) fn is_const(self) -> bool {
        self.contains(Self::Q_Const)
    }

    #[must_use]
    pub(super) fn is_volatile(self) -> bool {
        self.contains(Self::Q_Volatile)
    }

    #[must_use]
    pub(super) fn is_unaligned(self) -> bool {
        self.contains(Self::Q_Unaligned)
    }

    #[must_use]
    pub(super) fn is_restrict(self) -> bool {
        self.contains(Self::Q_Restrict)
    }

    pub(super) fn output<W: Writer>(
        self,
        ob: &mut W,
        space_before: bool,
        space_after: bool,
    ) -> Result<()> {
        if self != Self::Q_None {
            let len_before = ob.len();
            let space_before = self.output_if_present(ob, Self::Q_Const, space_before)?;
            let space_before = self.output_if_present(ob, Self::Q_Volatile, space_before)?;
            self.output_if_present(ob, Self::Q_Restrict, space_before)?;
            let len_after = ob.len();
            if space_after && len_after > len_before {
                write!(ob, " ")?;
            }
        }

        Ok(())
    }

    pub(super) fn output_if_present<W: Writer>(
        self,
        ob: &mut W,
        mask: Qualifiers,
        needs_space: bool,
    ) -> Result<bool> {
        if !self.contains(mask) {
            return Ok(needs_space);
        }

        if needs_space {
            write!(ob, " ")?;
        }

        mask.output_single_qualifier(ob)?;
        Ok(true)
    }

    pub(super) fn output_single_qualifier<W: Writer>(self, ob: &mut W) -> Result<()> {
        let qualifier = match self {
            Self::Q_Const => "const",
            Self::Q_Volatile => "volatile",
            Self::Q_Restrict => "__restrict",
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    format!("failed to output unsupported qualifier(s): {}", self.bits()),
                ))
            }
        };
        write!(ob, "{qualifier}")
    }
}

#[derive(Clone, Copy)]
pub(crate) enum StorageClass {
    PrivateStatic,
    ProtectedStatic,
    PublicStatic,
    Global,
    FunctionLocalStatic,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum PointerAffinity {
    Pointer,
    Reference,
    RValueReference,
}

#[derive(Clone, Copy)]
pub(crate) enum FunctionRefQualifier {
    Reference,
    RValueReference,
}

// Calling conventions
#[derive(Clone, Copy)]
pub(crate) enum CallingConv {
    Cdecl,
    Pascal,
    Thiscall,
    Stdcall,
    Fastcall,
    Clrcall,
    Eabi,
    Vectorcall,
    Swift,      // Clang-only
    SwiftAsync, // Clang-only
}

impl CallingConv {
    pub(super) fn output<W: Writer>(self, ob: &mut W) -> Result<()> {
        super::output_space_if_necessary(ob)?;
        let cc = match self {
            CallingConv::Cdecl => "__cdecl",
            CallingConv::Fastcall => "__fastcall",
            CallingConv::Pascal => "__pascal",
            CallingConv::Stdcall => "__stdcall",
            CallingConv::Thiscall => "__thiscall",
            CallingConv::Eabi => "__eabi",
            CallingConv::Vectorcall => "__vectorcall",
            CallingConv::Clrcall => "__clrcall",
            CallingConv::Swift => "__attribute__((__swiftcall__)) ",
            CallingConv::SwiftAsync => "__attribute__((__swiftasynccall__)) ",
        };
        write!(ob, "{cc}")
    }
}

#[derive(Clone, Copy)]
pub(crate) enum PrimitiveKind {
    Void,
    Bool,
    Char,
    Schar,
    Uchar,
    Char8,
    Char16,
    Char32,
    Short,
    Ushort,
    Int,
    Uint,
    Long,
    Ulong,
    Int64,
    Uint64,
    Wchar,
    Float,
    Double,
    Ldouble,
    Nullptr,
}

#[derive(Clone, Copy)]
pub(crate) enum CharKind {
    Char,
    Char16,
    Char32,
    Wchar,
}

#[derive(Clone, Copy)]
pub(crate) enum IntrinsicFunctionKind {
    New,                        // ?2 # operator new
    Delete,                     // ?3 # operator delete
    Assign,                     // ?4 # operator=
    RightShift,                 // ?5 # operator>>
    LeftShift,                  // ?6 # operator<<
    LogicalNot,                 // ?7 # operator!
    Equals,                     // ?8 # operator==
    NotEquals,                  // ?9 # operator!=
    ArraySubscript,             // ?A # operator[]
    Pointer,                    // ?C # operator->
    Dereference,                // ?D # operator*
    Increment,                  // ?E # operator++
    Decrement,                  // ?F # operator--
    Minus,                      // ?G # operator-
    Plus,                       // ?H # operator+
    BitwiseAnd,                 // ?I # operator&
    MemberPointer,              // ?J # operator->*
    Divide,                     // ?K # operator/
    Modulus,                    // ?L # operator%
    LessThan,                   // ?M operator<
    LessThanEqual,              // ?N operator<=
    GreaterThan,                // ?O operator>
    GreaterThanEqual,           // ?P operator>=
    Comma,                      // ?Q operator,
    Parens,                     // ?R operator()
    BitwiseNot,                 // ?S operator~
    BitwiseXor,                 // ?T operator^
    BitwiseOr,                  // ?U operator|
    LogicalAnd,                 // ?V operator&&
    LogicalOr,                  // ?W operator||
    TimesEqual,                 // ?X operator*=
    PlusEqual,                  // ?Y operator+=
    MinusEqual,                 // ?Z operator-=
    DivEqual,                   // ?_0 operator/=
    ModEqual,                   // ?_1 operator%=
    RshEqual,                   // ?_2 operator>>=
    LshEqual,                   // ?_3 operator<<=
    BitwiseAndEqual,            // ?_4 operator&=
    BitwiseOrEqual,             // ?_5 operator|=
    BitwiseXorEqual,            // ?_6 operator^=
    VbaseDtor,                  // ?_D # vbase destructor
    VecDelDtor,                 // ?_E # vector deleting destructor
    DefaultCtorClosure,         // ?_F # default constructor closure
    ScalarDelDtor,              // ?_G # scalar deleting destructor
    VecCtorIter,                // ?_H # vector constructor iterator
    VecDtorIter,                // ?_I # vector destructor iterator
    VecVbaseCtorIter,           // ?_J # vector vbase constructor iterator
    VdispMap,                   // ?_K # virtual displacement map
    EHVecCtorIter,              // ?_L # eh vector constructor iterator
    EHVecDtorIter,              // ?_M # eh vector destructor iterator
    EHVecVbaseCtorIter,         // ?_N # eh vector vbase constructor iterator
    CopyCtorClosure,            // ?_O # copy constructor closure
    LocalVftableCtorClosure,    // ?_T # local vftable constructor closure
    ArrayNew,                   // ?_U operator new[]
    ArrayDelete,                // ?_V operator delete[]
    ManVectorCtorIter,          // ?__A managed vector ctor iterator
    ManVectorDtorIter,          // ?__B managed vector dtor iterator
    EHVectorCopyCtorIter,       // ?__C EH vector copy ctor iterator
    EHVectorVbaseCopyCtorIter,  // ?__D EH vector vbase copy ctor iterator
    VectorCopyCtorIter,         // ?__G vector copy constructor iterator
    VectorVbaseCopyCtorIter,    // ?__H vector vbase copy constructor iterator
    ManVectorVbaseCopyCtorIter, // ?__I managed vector vbase copy constructor
    CoAwait,                    // ?__L operator co_await
    Spaceship,                  // ?__M operator<=>
}

#[derive(Clone, Copy)]
pub(crate) enum SpecialIntrinsicKind {
    Vftable,
    Vbtable,
    Typeof,
    VcallThunk,
    LocalStaticGuard,
    StringLiteralSymbol,
    UdtReturning,
    DynamicInitializer,
    DynamicAtexitDestructor,
    RttiTypeDescriptor,
    RttiBaseClassDescriptor,
    RttiBaseClassArray,
    RttiClassHierarchyDescriptor,
    RttiCompleteObjLocator,
    LocalVftable,
    LocalStaticThreadGuard,
}

bitflags::bitflags! {
    // Function classes
    #[derive(Clone, Copy, Default)]
    pub(crate) struct FuncClass: u16  {
        const FC_None = 0;
        const FC_Public = 1 << 0;
        const FC_Protected = 1 << 1;
        const FC_Private = 1 << 2;
        const FC_Global = 1 << 3;
        const FC_Static = 1 << 4;
        const FC_Virtual = 1 << 5;
        const FC_Far = 1 << 6;
        const FC_ExternC = 1 << 7;
        const FC_NoParameterList = 1 << 8;
        const FC_VirtualThisAdjust = 1 << 9;
        const FC_VirtualThisAdjustEx = 1 << 10;
        const FC_StaticThisAdjust = 1 << 11;
    }
}

impl FuncClass {
    #[must_use]
    pub(crate) fn is_public(self) -> bool {
        self.contains(Self::FC_Public)
    }

    #[must_use]
    pub(crate) fn is_protected(self) -> bool {
        self.contains(Self::FC_Protected)
    }

    #[must_use]
    pub(crate) fn is_private(self) -> bool {
        self.contains(Self::FC_Private)
    }

    #[must_use]
    pub(crate) fn is_global(self) -> bool {
        self.contains(Self::FC_Global)
    }

    #[must_use]
    pub(crate) fn is_static(self) -> bool {
        self.contains(Self::FC_Static)
    }

    #[must_use]
    pub(crate) fn is_virtual(self) -> bool {
        self.contains(Self::FC_Virtual)
    }

    #[must_use]
    pub(crate) fn is_extern_c(self) -> bool {
        self.contains(Self::FC_ExternC)
    }

    #[must_use]
    pub(crate) fn no_parameter_list(self) -> bool {
        self.contains(Self::FC_NoParameterList)
    }

    #[must_use]
    pub(crate) fn has_virtual_this_adjust(self) -> bool {
        self.contains(Self::FC_VirtualThisAdjust)
    }

    #[must_use]
    pub(crate) fn has_virtual_this_adjust_ex(self) -> bool {
        self.contains(Self::FC_VirtualThisAdjustEx)
    }

    #[must_use]
    pub(crate) fn has_static_this_adjust(self) -> bool {
        self.contains(Self::FC_StaticThisAdjust)
    }
}

#[derive(Clone, Copy)]
pub(crate) enum TagKind {
    Class,
    Struct,
    Union,
    Enum,
}
