//
// Copyright 2018 Red Hat, Inc.
//
// Author: Nathaniel McCallum <npmccallum@redhat.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! This crate provides the `UnsignedAbs` trait containing a function `uabs()`
//! which calculates the absolute value of the input and returns it as an
//! unsigned integer of the same size as the input.

extern crate signrel;

use signrel::SignRel;

/// Trait exposing the `uabs()` operation.
pub trait UnsignedAbs: SignRel {
    /// Calculate the absolute value as an unsigned integer.
    fn uabs(self) -> Self::Unsigned;
}

macro_rules! uabs_impl {
    ($($s:ident:$u:ident)*) => (
        $(
            impl UnsignedAbs for $s {
                #[inline]
                fn uabs(self) -> $u {
                    use std::$s::MIN;

                    match self {
                        MIN => self as $u,
                          _ => self.abs() as $u
                    }
                }
            }

            #[cfg(test)]
            mod $s {
                use std::$s::{MAX, MIN};
                use super::UnsignedAbs;

                #[test]
                fn max() {
                    assert_eq!(MAX.uabs(), MAX as $u);
                }

                #[test]
                fn min() {
                    assert_eq!(MIN.uabs(), MAX as $u + 1);
                }
            }

            impl UnsignedAbs for $u {
                #[inline]
                fn uabs(self) -> $u {
                    self
                }
            }

            #[cfg(test)]
            mod $u {
                use std::$u::{MAX, MIN};
                use super::UnsignedAbs;

                #[test]
                fn max() {
                    assert_eq!(MAX.uabs(), MAX);
                }

                #[test]
                fn min() {
                    assert_eq!(MIN.uabs(), MIN);
                }
            }
        )*
    )
}

uabs_impl! { isize:usize i64:u64 i32:u32 i16:u16 i8:u8 }

#[cfg(has_i128)]
uabs_impl! { i128:u128 }
