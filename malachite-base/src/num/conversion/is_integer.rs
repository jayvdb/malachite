use num::basic::floats::PrimitiveFloat;
use num::conversion::traits::{IsInteger, WrappingFrom};
use num::logic::traits::TrailingZeros;

fn is_integer_float<T: PrimitiveFloat>(x: T) -> bool {
    if x.is_nan() || x.is_infinite() {
        false
    } else if x == T::ZERO {
        true
    } else {
        let (raw_mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
        raw_exponent != 0
            && i64::wrapping_from(
                raw_exponent
                    + if raw_mantissa == 0 {
                        T::MANTISSA_WIDTH
                    } else {
                        TrailingZeros::trailing_zeros(raw_mantissa)
                    },
            ) > -T::MIN_EXPONENT
    }
}

macro_rules! impl_is_integer_primitive_int {
    ($t:ident) => {
        impl IsInteger for $t {
            /// Determines whether a value is an integer.
            ///
            /// For primitive integer types this always returns `true`.
            ///
            /// $f(x) = \textrm{true}$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::is_integer#is_integer).
            #[inline]
            fn is_integer(self) -> bool {
                true
            }
        }
    };
}
apply_to_primitive_ints!(impl_is_integer_primitive_int);

macro_rules! impl_is_integer_primitive_float {
    ($t:ident) => {
        impl IsInteger for $t {
            /// Determines whether a value is an integer.
            ///
            /// $f(x) = (x \in \Z)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::is_integer#is_integer).
            #[inline]
            fn is_integer(self) -> bool {
                is_integer_float(self)
            }
        }
    };
}
apply_to_primitive_floats!(impl_is_integer_primitive_float);