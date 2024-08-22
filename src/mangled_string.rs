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

use crate::extensions::CharExt as _;
use std::slice::SliceIndex;

#[derive(Clone, Copy)]
pub(crate) struct MangledString<'string> {
    string: &'string str,
    chars: usize,
}

impl<'string> MangledString<'string> {
    pub(crate) fn new(string: &'string str) -> Self {
        Self {
            string,
            chars: string.chars().count(),
        }
    }

    // advancing mangled string

    pub(crate) fn try_consume(&mut self) -> Option<char> {
        let mut iter = self.string.char_indices();
        let (_, c) = iter.next()?;
        self.string = iter.next().map_or("", |(i, _)| &self.string[i..]);
        self.chars -= 1;
        Some(c)
    }

    pub(crate) fn try_consume_char(&mut self, expected: char) -> Option<char> {
        let mut iter = self.string.char_indices();
        let (_, read) = iter.next()?;
        (read == expected).then(|| {
            self.string = iter.next().map_or("", |(i, _)| &self.string[i..]);
            self.chars -= 1;
            expected
        })
    }

    pub(crate) fn try_consume_char_if<F>(&mut self, f: F) -> Option<char>
    where
        F: FnOnce(&char) -> bool,
    {
        let mut iter = self.string.char_indices();
        let (_, c) = iter.next()?;
        f(&c).then(|| {
            self.string = iter.next().map_or("", |(i, _)| &self.string[i..]);
            self.chars -= 1;
            c
        })
    }

    pub(crate) fn try_consume_n_bytes(&mut self, n: usize) -> Option<&'string str> {
        self.string.split_at_checked(n).map(|(first, rest)| {
            self.string = rest;
            self.chars -= first.chars().count();
            first
        })
    }

    pub(crate) fn try_consume_n_chars<const N: usize>(&mut self) -> Option<[char; N]> {
        let mut iter = self.string.char_indices();
        let mut chars = ['\0'; N];
        for ch in &mut chars {
            let (_, c) = iter.next()?;
            *ch = c;
        }
        self.string = iter.next().map_or("", |(i, _)| &self.string[i..]);
        self.chars -= N;
        Some(chars)
    }

    pub(crate) fn try_consume_str<'s>(&mut self, s: &'s str) -> Option<&'s str> {
        self.string.starts_with(s).then(|| {
            self.string = &self.string[s.len()..];
            self.chars -= s.chars().count();
            s
        })
    }

    // inner string utils

    pub(crate) fn as_str(&self) -> &'string str {
        self.string
    }

    pub(crate) fn find_char(&self, needle: char) -> Option<usize> {
        for (i, c) in self.string.char_indices() {
            if c == needle {
                return Some(i);
            }
        }
        None
    }

    pub(crate) fn first_char(&self) -> Option<char> {
        self.string.chars().next()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.string.is_empty()
    }

    pub(crate) fn last_char(&self) -> Option<char> {
        self.string.chars().next_back()
    }

    pub(crate) fn len_bytes(&self) -> usize {
        self.string.len()
    }

    pub(crate) fn len_chars(&self) -> usize {
        self.chars
    }

    fn slice<I>(&self, index: I) -> Self
    where
        I: SliceIndex<str, Output = str>,
    {
        let string = &self.string[index];
        Self {
            string,
            chars: string.chars().count(),
        }
    }

    pub(crate) fn starts_with(&self, what: &str) -> bool {
        self.string.starts_with(what)
    }

    // mangled string utils

    pub(crate) fn starts_with_local_scope_pattern(mut self) -> bool {
        if self.try_consume_char('?').is_none() {
            return false;
        }

        let Some(end) = self.find_char('?') else {
            return false;
        };
        let candidate = self.slice(..end);
        if candidate.is_empty() {
            return false;
        }

        // \?[0-9]\?
        // ?@? is the discriminator 0.
        if candidate.len_chars() == 1 {
            // SAFETY: we just checked that the string has a length of 1
            let c = unsafe { candidate.first_char().unwrap_unchecked() };
            return c == '@' || c.is_ascii_digit();
        }

        // If it's not 0-9, then it's an encoded number terminated with an @
        let Some('@') = candidate.last_char() else {
            return false;
        };
        let mut candidate = candidate.slice(..candidate.len_bytes() - 1);

        // An encoded number starts with B-P and all subsequent digits are in A-P.
        // Note that the reason the first digit cannot be A is two fold. First, it
        // would create an ambiguity with ?A which delimits the beginning of an
        // anonymous namespace.  Second, A represents 0, and you don't start a multi
        // digit number with a leading 0.  Presumably the anonymous namespace
        // ambiguity is also why single digit encoded numbers use 0-9 rather than A-J.
        if candidate
            .try_consume_char_if(|x| ('B'..='P').contains(x))
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

    pub(crate) fn is_array_type(self) -> bool {
        self.first_char().is_some_and(|x| x == 'Y')
    }

    pub(crate) fn is_custom_type(self) -> bool {
        self.first_char().is_some_and(|x| x == '?')
    }

    pub(crate) fn is_function_type(self) -> bool {
        self.starts_with("$$A8@@") || self.starts_with("$$A6")
    }

    pub(crate) fn is_member_pointer(mut self) -> Option<bool> {
        match self.try_consume()? {
            // This is probably an rvalue reference (e.g. $$Q), and you cannot have an
            // rvalue reference to a member.
            // 'A' indicates a reference, and you cannot have a reference to a member
            // function or member.
            '$' | 'A' => return Some(false),
            // These 4 values indicate some kind of pointer, but we still don't know
            // what.
            'P' | 'Q' | 'R' | 'S' => (),
            _ => return None,
        }

        // If it starts with a number, then 6 indicates a non-member function
        // pointer, and 8 indicates a member function pointer.
        if let Some(digit) = self.try_consume_char_if(char::is_ascii_digit) {
            return match digit {
                '6' => Some(false),
                '8' => Some(true),
                _ => None,
            };
        }

        // Remove ext qualifiers since those can appear on either type and are
        // therefore not indicative.
        _ = self.try_consume_char('E'); // 64-bit
        _ = self.try_consume_char('I'); // restrict
        _ = self.try_consume_char('F'); // unaligned

        if self.is_empty() {
            return None;
        }

        // The next value should be either ABCD (non-member) or QRST (member).
        match self.first_char() {
            Some('A' | 'B' | 'C' | 'D') => Some(false),
            Some('Q' | 'R' | 'S' | 'T') => Some(true),
            _ => None,
        }
    }

    pub(crate) fn is_pointer_type(self) -> bool {
        if self.starts_with("$$Q") {
            // foo &&
            true
        } else {
            // A -> foo &
            // P -> foo *
            // Q -> foo *const
            // R -> foo *volatile
            // S -> foo *const volatile
            matches!(self.first_char(), Some('A' | 'P' | 'Q' | 'R' | 'S'))
        }
    }

    pub(crate) fn is_tag_type(self) -> bool {
        // T -> union
        // U -> struct
        // V -> class
        // W -> enum
        matches!(self.first_char(), Some('T' | 'U' | 'V' | 'W'))
    }
}
