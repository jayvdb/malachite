use num::arithmetic::traits::{Pow, PowAssign};
use num::conversion::traits::ExactFrom;

macro_rules! impl_pow {
    ($t:ident) => {
        impl Pow<u64> for $t {
            type Output = $t;

            #[inline]
            fn pow(self, exp: u64) -> $t {
                $t::pow(self, u32::exact_from(exp))
            }
        }

        impl PowAssign<u64> for $t {
            /// Replaces `self` with `self ^ exp`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::PowAssign;
            ///
            /// let mut x = 3u8;
            /// x.pow_assign(3);
            /// assert_eq!(x, 27);
            ///
            /// let mut x = -10i32;
            /// x.pow_assign(9);
            /// assert_eq!(x, -1_000_000_000);
            /// ```
            #[inline]
            fn pow_assign(&mut self, exp: u64) {
                *self = Pow::pow(*self, exp);
            }
        }
    };
}
impl_pow!(u8);
impl_pow!(u16);
impl_pow!(u32);
impl_pow!(u64);
impl_pow!(u128);
impl_pow!(usize);
impl_pow!(i8);
impl_pow!(i16);
impl_pow!(i32);
impl_pow!(i64);
impl_pow!(i128);
impl_pow!(isize);