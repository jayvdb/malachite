use std::ops::{Neg, Shl, Shr};

use comparison::traits::Min;
use num::arithmetic::traits::{ArithmeticCheckedShl, UnsignedAbs};
use num::basic::integers::PrimitiveInteger;
use num::basic::traits::Zero;
use num::conversion::traits::{CheckedFrom, WrappingFrom};

pub fn _arithmetic_checked_shl_unsigned_unsigned<
    T: PrimitiveInteger,
    U: Copy + Ord + WrappingFrom<u64>,
>(
    x: T,
    bits: U,
) -> Option<T>
where
    T: Shl<U, Output = T> + Shr<U, Output = T>,
{
    if x == T::ZERO {
        Some(x)
    } else if bits >= U::wrapping_from(T::WIDTH) {
        None
    } else {
        let result = x << bits;
        if result >> bits == x {
            Some(result)
        } else {
            None
        }
    }
}

macro_rules! impl_arithmetic_checked_shl_unsigned_unsigned {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shl_unsigned_unsigned_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShl<$u> for $t {
                    type Output = $t;

                    /// Shifts `self` left (multiplies it by a power of 2). If the result is too
                    /// large to fit in a `$t`, `None` is returned. Zero may be shifted by any
                    /// amount.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ArithmeticCheckedShl;
                    ///
                    /// assert_eq!(3u8.arithmetic_checked_shl(6), Some(192u8));
                    /// assert_eq!(3u8.arithmetic_checked_shl(7), None);
                    /// assert_eq!(3u8.arithmetic_checked_shl(100), None);
                    /// assert_eq!(0u8.arithmetic_checked_shl(100), Some(0u8));
                    /// ```
                    #[inline]
                    fn arithmetic_checked_shl(self, bits: $u) -> Option<$t> {
                        _arithmetic_checked_shl_unsigned_unsigned(self, bits)
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_arithmetic_checked_shl_unsigned_unsigned_inner);
    };
}
apply_to_unsigneds!(impl_arithmetic_checked_shl_unsigned_unsigned);

pub fn _arithmetic_checked_shl_unsigned_signed<
    T: PrimitiveInteger,
    U: Ord + WrappingFrom<u64>,
    S: Copy + Ord + UnsignedAbs + Zero,
>(
    x: T,
    bits: S,
) -> Option<T>
where
    S: UnsignedAbs<Output = U>,
    T: ArithmeticCheckedShl<U, Output = T> + Shr<U, Output = T>,
{
    if bits >= S::ZERO {
        x.arithmetic_checked_shl(bits.unsigned_abs())
    } else {
        let abs_bits = bits.unsigned_abs();
        Some(if abs_bits >= U::wrapping_from(T::WIDTH) {
            T::ZERO
        } else {
            x >> abs_bits
        })
    }
}

macro_rules! impl_arithmetic_checked_shl_unsigned_signed {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shl_unsigned_signed_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShl<$u> for $t {
                    type Output = $t;

                    /// Shifts `self` left (multiplies it by a power of 2). If the result is too
                    /// large to fit in a `$t`, `None` is returned. Zero may be shifted by any
                    /// amount, and any number may be shifted by any negative amount; shifting by a
                    /// negative amount with a high absolute value returns `Some(0)`.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ArithmeticCheckedShl;
                    ///
                    /// assert_eq!(3u8.arithmetic_checked_shl(6), Some(192u8));
                    /// assert_eq!(3u8.arithmetic_checked_shl(7), None);
                    /// assert_eq!(3u8.arithmetic_checked_shl(100), None);
                    /// assert_eq!(0u8.arithmetic_checked_shl(100), Some(0u8));
                    /// assert_eq!(100u8.arithmetic_checked_shl(-3), Some(12u8));
                    /// assert_eq!(100u8.arithmetic_checked_shl(-100), Some(0u8));
                    /// ```
                    #[inline]
                    fn arithmetic_checked_shl(self, bits: $u) -> Option<$t> {
                        _arithmetic_checked_shl_unsigned_signed(self, bits)
                    }
                }
            };
        }
        apply_to_signeds!(impl_arithmetic_checked_shl_unsigned_signed_inner);
    };
}
apply_to_unsigneds!(impl_arithmetic_checked_shl_unsigned_signed);

pub fn _arithmetic_checked_shl_signed_unsigned<U: Eq, S: Copy + Min + Ord + Zero, B>(
    x: S,
    bits: B,
) -> Option<S>
where
    S: CheckedFrom<U> + Neg<Output = S> + UnsignedAbs<Output = U>,
    U: ArithmeticCheckedShl<B, Output = U>,
{
    let abs = x.unsigned_abs();
    if x >= S::ZERO {
        abs.arithmetic_checked_shl(bits).and_then(S::checked_from)
    } else {
        abs.arithmetic_checked_shl(bits).and_then(|x| {
            if x == S::MIN.unsigned_abs() {
                Some(S::MIN)
            } else {
                S::checked_from(x).map(|y| -y)
            }
        })
    }
}

macro_rules! impl_arithmetic_checked_shl_signed_unsigned {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shl_signed_unsigned_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShl<$u> for $t {
                    type Output = $t;

                    /// Shifts `self` left (multiplies it by a power of 2). If the result is too
                    /// large to fit in a `$t`, `None` is returned. Zero may be shifted by any
                    /// amount.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ArithmeticCheckedShl;
                    ///
                    /// assert_eq!(3i8.arithmetic_checked_shl(5), Some(96i8));
                    /// assert_eq!(3i8.arithmetic_checked_shl(6), None);
                    /// assert_eq!((-3i8).arithmetic_checked_shl(5), Some(-96i8));
                    /// assert_eq!((-3i8).arithmetic_checked_shl(6), None);
                    /// assert_eq!(3i8.arithmetic_checked_shl(100), None);
                    /// assert_eq!((-3i8).arithmetic_checked_shl(100), None);
                    /// assert_eq!(0i8.arithmetic_checked_shl(100), Some(0i8));
                    /// ```
                    #[inline]
                    fn arithmetic_checked_shl(self, bits: $u) -> Option<$t> {
                        _arithmetic_checked_shl_signed_unsigned(self, bits)
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_arithmetic_checked_shl_signed_unsigned_inner);
    };
}
apply_to_signeds!(impl_arithmetic_checked_shl_signed_unsigned);

pub fn _arithmetic_checked_shl_signed_signed<
    T: PrimitiveInteger,
    U: Copy + Ord + WrappingFrom<u64> + Zero,
    S: Copy + Ord + Zero,
>(
    x: T,
    bits: S,
) -> Option<T>
where
    S: UnsignedAbs<Output = U>,
    T: ArithmeticCheckedShl<U, Output = T> + Neg<Output = T> + Shr<U, Output = T>,
{
    if bits >= S::ZERO {
        x.arithmetic_checked_shl(bits.unsigned_abs())
    } else {
        let width = U::wrapping_from(T::WIDTH);
        let abs_bits = bits.unsigned_abs();
        Some(if width != U::ZERO && abs_bits >= width {
            -T::iverson(x < T::ZERO)
        } else {
            x >> abs_bits
        })
    }
}

macro_rules! impl_arithmetic_checked_shl_signed_signed {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shl_signed_signed_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShl<$u> for $t {
                    type Output = $t;

                    /// Shifts `self` left (multiplies it by a power of 2). If the result is too
                    /// large to fit in a `$t`, `None` is returned. Zero may be shifted by any
                    /// amount, and any number may be shifted by any negative amount; shifting by a
                    /// negative amount with a high absolute value returns `Some(0)` if `self` is
                    /// positive, and `Some(-1)` if `self` is negative.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ArithmeticCheckedShl;
                    ///
                    /// assert_eq!(3i8.arithmetic_checked_shl(5), Some(96i8));
                    /// assert_eq!(3i8.arithmetic_checked_shl(6), None);
                    /// assert_eq!((-3i8).arithmetic_checked_shl(5), Some(-96i8));
                    /// assert_eq!((-3i8).arithmetic_checked_shl(6), None);
                    /// assert_eq!(3i8.arithmetic_checked_shl(100), None);
                    /// assert_eq!((-3i8).arithmetic_checked_shl(100), None);
                    /// assert_eq!(0i8.arithmetic_checked_shl(100), Some(0i8));
                    /// assert_eq!(100i8.arithmetic_checked_shl(-3), Some(12i8));
                    /// assert_eq!((-100i8).arithmetic_checked_shl(-3), Some(-13i8));
                    /// assert_eq!(100i8.arithmetic_checked_shl(-100), Some(0i8));
                    /// assert_eq!((-100i8).arithmetic_checked_shl(-100), Some(-1i8));
                    /// ```
                    fn arithmetic_checked_shl(self, bits: $u) -> Option<$t> {
                        _arithmetic_checked_shl_signed_signed(self, bits)
                    }
                }
            };
        }
        apply_to_signeds!(impl_arithmetic_checked_shl_signed_signed_inner);
    };
}
apply_to_signeds!(impl_arithmetic_checked_shl_signed_signed);