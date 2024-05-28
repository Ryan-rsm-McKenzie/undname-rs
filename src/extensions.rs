use bstr::{
    BStr,
    ByteSlice as _,
};

#[allow(clippy::wrong_self_convention)]
pub(crate) trait BStrExt: Sized {
    #[must_use]
    fn try_consume(&mut self) -> Option<u8>;

    #[must_use]
    fn try_consume_n(&mut self, n: usize) -> Option<Self>;

    #[must_use]
    fn try_consume_n_fixed<const N: usize>(&mut self) -> Option<[u8; N]>;

    #[must_use]
    fn try_consume_byte(&mut self, c: u8) -> Option<u8>;

    #[must_use]
    fn try_consume_byte_if<F: FnOnce(&u8) -> bool>(&mut self, f: F) -> Option<u8>;

    #[must_use]
    fn try_consume_str<const N: usize>(&mut self, s: &'static [u8; N]) -> Option<&'static [u8; N]>;

    #[must_use]
    fn starts_with_local_scope_pattern(self) -> bool;

    #[must_use]
    fn is_tag_type(self) -> bool;

    #[must_use]
    fn is_pointer_type(self) -> bool;

    #[must_use]
    fn is_member_pointer(self) -> Option<bool>;

    #[must_use]
    fn is_array_type(self) -> bool;

    #[must_use]
    fn is_function_type(self) -> bool;

    #[must_use]
    fn is_custom_type(self) -> bool;
}

impl BStrExt for &BStr {
    fn try_consume(&mut self) -> Option<u8> {
        let (first, rest) = self.split_first()?;
        *self = rest.into();
        Some(*first)
    }

    fn try_consume_n(&mut self, n: usize) -> Option<Self> {
        if n <= self.len() {
            let (first, rest) = (&self[..n], &self[n..self.len() - n]);
            *self = rest;
            Some(first)
        } else {
            None
        }
    }

    fn try_consume_n_fixed<const N: usize>(&mut self) -> Option<[u8; N]> {
        let first = *self.first_chunk::<N>()?;
        *self = &self[N..];
        Some(first)
    }

    fn try_consume_byte(&mut self, c: u8) -> Option<u8> {
        let (first, rest) = self.split_first()?;
        if *first == c {
            *self = rest.into();
            Some(*first)
        } else {
            None
        }
    }

    fn try_consume_byte_if<F>(&mut self, f: F) -> Option<u8>
    where
        F: FnOnce(&u8) -> bool,
    {
        let (first, rest) = self.split_first()?;
        if f(first) {
            *self = rest.into();
            Some(*first)
        } else {
            None
        }
    }

    fn try_consume_str<const N: usize>(&mut self, s: &'static [u8; N]) -> Option<&'static [u8; N]> {
        let mid = s.len();
        if self.len() < mid {
            None
        } else {
            let (first, rest) = (&self[..mid], &self[mid..]);
            if first == s.as_slice() {
                *self = rest;
                Some(s)
            } else {
                None
            }
        }
    }

    fn starts_with_local_scope_pattern(mut self) -> bool {
        if self.try_consume_byte(b'?').is_none() {
            return false;
        }

        let Some(end) = self.find_byte(b'?') else {
            return false;
        };
        let candidate = &self[..end];
        if candidate.is_empty() {
            return false;
        }

        // \?[0-9]\?
        // ?@? is the discriminator 0.
        if let Some(first) = candidate.first() {
            return *first == b'@' || first.is_ascii_digit();
        }

        // If it's not 0-9, then it's an encoded number terminated with an @
        let Some(b'@') = candidate.last() else {
            return false;
        };
        let mut candidate = &candidate[..candidate.len() - 1];

        // An encoded number starts with B-P and all subsequent digits are in A-P.
        // Note that the reason the first digit cannot be A is two fold. First, it
        // would create an ambiguity with ?A which delimits the beginning of an
        // anonymous namespace.  Second, A represents 0, and you don't start a multi
        // digit number with a leading 0.  Presumably the anonymous namespace
        // ambiguity is also why single digit encoded numbers use 0-9 rather than A-J.
        if candidate
            .try_consume_byte_if(|x| (b'B'..=b'P').contains(x))
            .is_none()
        {
            return false;
        }
        while let Some(c) = candidate.try_consume() {
            if !c.is_rebased_ascii_hexdigit() {
                return false;
            }
        }

        true
    }

    fn is_tag_type(self) -> bool {
        matches!(
            self.first(),
            Some(b'T') // union
                | Some(b'U') // struct
                | Some(b'V') // class
                | Some(b'W') // enum
        )
    }

    fn is_pointer_type(self) -> bool {
        if self.starts_with(b"$$Q") {
            // foo &&
            true
        } else {
            matches!(
                self.first(),
                Some(b'A') // foo &
                    | Some(b'P') // foo *
                    | Some(b'Q') // foo *const
                    | Some(b'R') // foo *volatile
                    | Some(b'S') // foo *const volatile
            )
        }
    }

    fn is_member_pointer(mut self) -> Option<bool> {
        match self.try_consume()? {
            // This is probably an rvalue reference (e.g. $$Q), and you cannot have an
            // rvalue reference to a member.
            // 'A' indicates a reference, and you cannot have a reference to a member
            // function or member.
            b'$' | b'A' => return Some(false),
            // These 4 values indicate some kind of pointer, but we still don't know
            // what.
            b'P' | b'Q' | b'R' | b'S' => (),
            _ => return None,
        }

        // If it starts with a number, then 6 indicates a non-member function
        // pointer, and 8 indicates a member function pointer.
        if let Some(digit) = self.try_consume_byte_if(u8::is_ascii_digit) {
            return match digit {
                b'6' => Some(false),
                b'8' => Some(true),
                _ => None,
            };
        }

        // Remove ext qualifiers since those can appear on either type and are
        // therefore not indicative.
        _ = self.try_consume_byte(b'E'); // 64-bit
        _ = self.try_consume_byte(b'I'); // restrict
        _ = self.try_consume_byte(b'F'); // unaligned

        if self.is_empty() {
            return None;
        }

        // The next value should be either ABCD (non-member) or QRST (member).
        match self.first() {
            Some(b'A') | Some(b'B') | Some(b'C') | Some(b'D') => Some(false),
            Some(b'Q') | Some(b'R') | Some(b'S') | Some(b'T') => Some(true),
            _ => None,
        }
    }

    fn is_array_type(self) -> bool {
        self.first().is_some_and(|&x| x == b'Y')
    }

    fn is_function_type(self) -> bool {
        self.starts_with(b"$$A8@@") || self.starts_with(b"$$A6")
    }

    fn is_custom_type(self) -> bool {
        self.first().is_some_and(|&x| x == b'?')
    }
}

#[allow(clippy::wrong_self_convention)]
pub(crate) trait U8Ext: Sized {
    fn is_rebased_ascii_hexdigit(self) -> bool;
    fn try_convert_rebased_ascii_hexdigit_to_number(self) -> Option<Self>;
}

impl U8Ext for u8 {
    fn is_rebased_ascii_hexdigit(self) -> bool {
        (b'A'..=b'P').contains(&self)
    }

    fn try_convert_rebased_ascii_hexdigit_to_number(self) -> Option<Self> {
        if self.is_rebased_ascii_hexdigit() {
            if self <= b'J' {
                Some(self - b'A')
            } else {
                Some(10 + self - b'K')
            }
        } else {
            None
        }
    }
}
