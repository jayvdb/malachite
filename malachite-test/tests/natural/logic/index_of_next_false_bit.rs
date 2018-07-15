use common::test_properties;
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{BitAccess, BitScan, Zero};
use malachite_nz::natural::logic::bit_scan::limbs_index_of_next_false_bit;
use malachite_nz::natural::Natural;
use malachite_test::common::natural_to_rug_integer;
use malachite_test::inputs::base::{pairs_of_unsigned_vec_and_small_u64, unsigneds};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_small_u64};
use malachite_test::natural::logic::index_of_next_false_bit::natural_index_of_next_false_bit_alt;
use rug;
use std::str::FromStr;
use std::u32;

#[test]
fn test_limbs_index_of_next_false_bit() {
    let test = |limbs, u, out| {
        assert_eq!(limbs_index_of_next_false_bit(limbs, u), out);
    };
    test(&[], 0, 0);
    test(&[], 100, 100);
    test(&[0], 0, 0);
    test(&[0], 100, 100);
    test(&[0b100], 0, 0);
    test(&[0b100], 1, 1);
    test(&[0b100], 2, 3);
    test(&[0b100], 3, 3);
    test(&[0, 0b1011], 0, 0);
    test(&[0, 0b1011], 20, 20);
    test(&[0, 0b1011], 31, 31);
    test(&[0, 0b1011], 32, 34);
    test(&[0, 0b1011], 33, 34);
    test(&[0, 0b1011], 34, 34);
    test(&[0, 0b1011], 35, 36);
    test(&[0, 0b1011], 100, 100);
    test(&[0, 0b1011, 0xffff_fff0, u32::MAX, 1], 64, 64);
    test(&[0, 0b1011, 0xffff_fff0, u32::MAX, 1], 68, 129);
}

#[test]
fn test_index_of_next_false_bit() {
    let test = |n, u, out| {
        assert_eq!(
            Natural::from_str(n).unwrap().index_of_next_false_bit(u),
            out
        );
        assert_eq!(
            natural_index_of_next_false_bit_alt(&Natural::from_str(n).unwrap(), u),
            out
        );
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .find_zero(u32::checked_from(u).unwrap())
                .map(|u| u64::from(u)),
            out
        );
    };
    test("0", 0, Some(0));
    test("0", 100, Some(100));
    test("47244640256", 0, Some(0));
    test("47244640256", 20, Some(20));
    test("47244640256", 31, Some(31));
    test("47244640256", 32, Some(34));
    test("47244640256", 33, Some(34));
    test("47244640256", 34, Some(34));
    test("47244640256", 35, Some(36));
    test("47244640256", 100, Some(100));
    test("680564733841876926631601309731428237312", 64, Some(64));
    test("680564733841876926631601309731428237312", 68, Some(129));
}

#[test]
fn limbs_index_of_next_false_bit_properties() {
    test_properties(pairs_of_unsigned_vec_and_small_u64, |&(ref limbs, u)| {
        assert_eq!(
            Some(limbs_index_of_next_false_bit(limbs, u)),
            Natural::from_limbs_asc(limbs).index_of_next_false_bit(u)
        );
    });
}

#[test]
fn index_of_next_false_bit_properties() {
    test_properties(pairs_of_natural_and_small_u64, |&(ref n, u)| {
        let result = n.index_of_next_false_bit(u);
        assert_eq!(result, natural_index_of_next_false_bit_alt(n, u));
        assert_eq!(
            natural_to_rug_integer(n)
                .find_zero(u32::checked_from(u).unwrap())
                .map(|u| u64::from(u)),
            result
        );
        let result = result.unwrap();
        assert!(result >= u);
        assert!(!n.get_bit(result));
        assert_eq!(result == u, !n.get_bit(u));
        assert_eq!((!n).index_of_next_true_bit(u), Some(result));
    });

    test_properties(naturals, |n| {
        assert_eq!(n.index_of_next_false_bit(0), (!n).trailing_zeros());
    });

    test_properties(unsigneds, |&u: &u64| {
        assert_eq!(Natural::ZERO.index_of_next_false_bit(u), Some(u));
    });
}