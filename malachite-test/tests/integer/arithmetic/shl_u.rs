use common::{test_properties, test_properties_no_special};
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{Abs, IsPowerOfTwo, One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::small_unsigneds;
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_small_unsigned};
use num::BigInt;
use rug;
use std::str::FromStr;

macro_rules! tests_and_properties {
    (
        $t:ident,
        $test_shl_u:ident,
        $shl_u_properties:ident,
        $u:ident,
        $v:ident,
        $out:ident,
        $library_comparison_tests:expr,
        $n:ident,
        $shifted:ident,
        $library_comparison_properties:expr
    ) => {
        #[test]
        fn $test_shl_u() {
            let test = |$u, $v: $t, $out| {
                let mut n = Integer::from_str($u).unwrap();
                n <<= $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = Integer::from_str($u).unwrap() << $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = &Integer::from_str($u).unwrap() << $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                $library_comparison_tests
            };
            test("0", 0, "0");
            test("0", 10, "0");

            test("123", 0, "123");
            test("123", 1, "246");
            test("123", 2, "492");
            test("123", 25, "4127195136");
            test("123", 26, "8254390272");
            test("123", 100, "155921023828072216384094494261248");
            test("2147483648", 1, "4294967296");
            test("1000000000000", 0, "1000000000000");
            test("1000000000000", 3, "8000000000000");
            test("1000000000000", 24, "16777216000000000000");
            test("1000000000000", 25, "33554432000000000000");
            test("1000000000000", 31, "2147483648000000000000");
            test("1000000000000", 32, "4294967296000000000000");
            test("1000000000000", 33, "8589934592000000000000");
            test(
                "1000000000000",
                100,
                "1267650600228229401496703205376000000000000",
            );

            test("-123", 0, "-123");
            test("-123", 1, "-246");
            test("-123", 2, "-492");
            test("-123", 25, "-4127195136");
            test("-123", 26, "-8254390272");
            test("-123", 100, "-155921023828072216384094494261248");
            test("-2147483648", 1, "-4294967296");
            test("-1000000000000", 0, "-1000000000000");
            test("-1000000000000", 3, "-8000000000000");
            test("-1000000000000", 24, "-16777216000000000000");
            test("-1000000000000", 25, "-33554432000000000000");
            test("-1000000000000", 31, "-2147483648000000000000");
            test("-1000000000000", 32, "-4294967296000000000000");
            test("-1000000000000", 33, "-8589934592000000000000");
            test(
                "-1000000000000",
                100,
                "-1267650600228229401496703205376000000000000",
            );
        }

        #[test]
        fn $shl_u_properties() {
            test_properties(
                pairs_of_integer_and_small_unsigned::<$t>,
                |&(ref $n, $u)| {
                    let mut mut_n = $n.clone();
                    mut_n <<= $u;
                    assert!(mut_n.is_valid());
                    let $shifted = mut_n;

                    let shifted_alt = $n << $u;
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, $shifted);
                    let shifted_alt = $n.clone() << $u;
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, $shifted);

                    assert!(($n << $u).abs() >= $n.abs());
                    assert_eq!(-$n << $u, -($n << $u));

                    assert_eq!($n << $u, $n * (Integer::ONE << $u));
                    assert_eq!($n << $u >> $u, *$n);

                    //TODO
                        /*if u <= (i32::MAX as u32) {
                            assert_eq!(n << (u as i32), shifted);
                            assert_eq!(n >> -(u as i32), shifted);
                        }*/

                    $library_comparison_properties
                },
            );

            #[allow(unknown_lints, identity_op)]
            test_properties(integers, |n| {
                assert_eq!(n << $t::ZERO, *n);
            });

            test_properties_no_special(small_unsigneds::<$t>, |&u| {
                assert_eq!(Integer::ZERO << u, 0);
                assert!(
                    Natural::checked_from(Integer::ONE << u)
                        .unwrap()
                        .is_power_of_two()
                );
            });
        }
    };
}
tests_and_properties!(
    u8,
    test_shl_u8,
    shl_u8_properties,
    u,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
tests_and_properties!(
    u16,
    test_shl_u16,
    shl_u16_properties,
    u,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
tests_and_properties!(
    u32,
    test_shl_u32,
    shl_u32_properties,
    u,
    v,
    out,
    {
        let mut n = rug::Integer::from_str(u).unwrap();
        n <<= v;
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() << v;
        assert_eq!(n.to_string(), out);

        let n = BigInt::from_str(u).unwrap() << v as usize;
        assert_eq!(n.to_string(), out);

        let n = &BigInt::from_str(u).unwrap() << v as usize;
        assert_eq!(n.to_string(), out);
    },
    n,
    shifted,
    {
        let mut rug_n = integer_to_rug_integer(n);
        rug_n <<= u;
        assert_eq!(rug_integer_to_integer(&rug_n), shifted);

        assert_eq!(
            bigint_to_integer(&(&integer_to_bigint(n) << u as usize)),
            shifted
        );
        assert_eq!(
            bigint_to_integer(&(integer_to_bigint(n) << u as usize)),
            shifted
        );

        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(n) << u)),
            shifted
        );
    }
);
tests_and_properties!(
    u64,
    test_shl_u64,
    shl_u64_properties,
    u,
    v,
    out,
    {},
    n,
    shifted,
    {}
);