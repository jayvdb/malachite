use common::test_properties;
use malachite_base::misc::{CheckedFrom, Max, Min, WrappingFrom};
use malachite_base::num::{ModPowerOfTwo, PrimitiveInteger, PrimitiveUnsigned};
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::integer_to_rug_integer;
use malachite_test::inputs::integer::integers;
#[cfg(feature = "32_bit_limbs")]
use rug;
use std::str::FromStr;

#[test]
fn test_signed_limb_checked_from_integer() {
    let test = |n, out| {
        assert_eq!(SignedLimb::checked_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(
            SignedLimb::checked_from(&Integer::from_str(n).unwrap()),
            out
        );
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(rug::Integer::from_str(n).unwrap().to_i32(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("-123", Some(-123));
    #[cfg(feature = "32_bit_limbs")]
    {
        test("1000000000000", None);
        test("-1000000000000", None);
        test("2147483647", Some(SignedLimb::MAX));
        test("2147483648", None);
        test("-2147483648", Some(SignedLimb::MIN));
        test("-2147483649", None);
    }
    #[cfg(feature = "64_bit_limbs")]
    {
        test("1000000000000", Some(1000000000000));
        test("-1000000000000", Some(-1000000000000));
        test("9223372036854775807", Some(SignedLimb::MAX));
        test("9223372036854775808", None);
        test("-9223372036854775808", Some(SignedLimb::MIN));
        test("-9223372036854775809", None);
    }
}

#[test]
fn test_signed_limb_wrapping_from_integer() {
    let test = |n, out| {
        assert_eq!(
            SignedLimb::wrapping_from(Integer::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            SignedLimb::wrapping_from(&Integer::from_str(n).unwrap()),
            out
        );
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(rug::Integer::from_str(n).unwrap().to_i32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("-123", -123);
    #[cfg(feature = "32_bit_limbs")]
    {
        test("1000000000000", -727_379_968);
        test("-1000000000000", 727_379_968);
        test("2147483647", SignedLimb::MAX);
        test("2147483648", SignedLimb::MIN);
        test("-2147483648", SignedLimb::MIN);
        test("-2147483649", SignedLimb::MAX);
    }
    #[cfg(feature = "64_bit_limbs")]
    {
        test("1000000000000", 1000000000000);
        test("-1000000000000", -1000000000000);
        test("9223372036854775807", SignedLimb::MAX);
        test("9223372036854775808", SignedLimb::MIN);
        test("-9223372036854775808", SignedLimb::MIN);
        test("-9223372036854775809", SignedLimb::MAX);
    }
}

#[test]
fn signed_limb_checked_from_integer_properties() {
    test_properties(integers, |x| {
        let result = SignedLimb::checked_from(x);
        assert_eq!(SignedLimb::checked_from(x.clone()), result);
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(integer_to_rug_integer(x).to_i32(), result);
        if *x >= SignedLimb::MIN && *x <= SignedLimb::MAX {
            assert_eq!(Integer::from(result.unwrap()), *x);
            assert_eq!(result, Some(SignedLimb::wrapping_from(x)));
        } else {
            assert!(result.is_none());
        }
    });
}

#[test]
fn signed_limb_wrapping_from_integer_properties() {
    test_properties(integers, |x| {
        let result = SignedLimb::wrapping_from(x);
        assert_eq!(SignedLimb::wrapping_from(x.clone()), result);
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(integer_to_rug_integer(x).to_i32_wrapping(), result);
        assert_eq!(-result, SignedLimb::wrapping_from(&-x));
        assert_eq!(
            result,
            Limb::checked_from(&x.mod_power_of_two(Limb::WIDTH.into()))
                .unwrap()
                .to_signed_bitwise()
        );
    });
}