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

pub(crate) trait CharExt {
    fn is_rebased_ascii_hexdigit(&self) -> bool;
    fn try_convert_rebased_ascii_hexdigit_to_number(&self) -> Option<u8>;
}

impl CharExt for char {
    fn is_rebased_ascii_hexdigit(&self) -> bool {
        ('A'..='P').contains(self)
    }

    fn try_convert_rebased_ascii_hexdigit_to_number(&self) -> Option<u8> {
        if self.is_rebased_ascii_hexdigit() {
            let this = *self as u8;
            if this <= b'J' {
                Some(this - b'A')
            } else {
                Some(10 + this - b'K')
            }
        } else {
            None
        }
    }
}
