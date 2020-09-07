// SPDX-License-Identifier: Apache-2.0

//! This crate provides the `UnsignedAbs` trait containing a function `uabs()`
//! which calculates the absolute value of the input and returns it as an
//! unsigned integer of the same size as the input.

#![no_std]

use signrel::SignRel;

/// Trait exposing the `uabs()` operation.
pub trait Uabs: SignRel {
    /// Calculate the absolute value as an unsigned integer.
    fn uabs(self) -> Self::Unsigned;
}

macro_rules! uabs_impl {
    ($($s:ident:$u:ident)*) => (
        $(
            impl Uabs for $s {
                #[inline]
                fn uabs(self) -> $u {
                    use core::$s::MIN;

                    match self {
                        MIN => self as $u,
                          _ => self.abs() as $u
                    }
                }
            }

            #[cfg(test)]
            mod $s {
                use core::$s::{MAX, MIN};
                use super::Uabs;

                #[test]
                fn max() {
                    assert_eq!(MAX.uabs(), MAX as $u);
                }

                #[test]
                fn min() {
                    assert_eq!(MIN.uabs(), MAX as $u + 1);
                }
            }

            impl Uabs for $u {
                #[inline]
                fn uabs(self) -> $u {
                    self
                }
            }

            #[cfg(test)]
            mod $u {
                use core::$u::{MAX, MIN};
                use super::Uabs;

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

uabs_impl! { isize:usize i128:u128 i64:u64 i32:u32 i16:u16 i8:u8 }
