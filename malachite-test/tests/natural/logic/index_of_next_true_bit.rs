use common::test_properties;
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{BitAccess, BitScan, SignificantBits, Zero};
use malachite_nz::natural::logic::bit_scan::limbs_index_of_next_true_bit;
use malachite_nz::natural::Natural;
use malachite_test::common::natural_to_rug_integer;
use malachite_test::inputs::base::{pairs_of_unsigned_vec_and_small_u64, unsigneds};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_small_u64};
use malachite_test::natural::logic::index_of_next_true_bit::natural_index_of_next_true_bit_alt;
use rug;
use std::str::FromStr;

#[test]
fn test_limbs_index_of_next_true_bit() {
    let test = |limbs, u, out| {
        assert_eq!(limbs_index_of_next_true_bit(limbs, u), out);
    };
    test(&[], 0, None);
    test(&[], 100, None);
    test(&[0], 0, None);
    test(&[0], 100, None);
    test(&[0b100], 0, Some(2));
    test(&[0b100], 1, Some(2));
    test(&[0b100], 2, Some(2));
    test(&[0b100], 3, None);
    test(&[0, 0b1011], 0, Some(32));
    test(&[0, 0b1011], 20, Some(32));
    test(&[0, 0b1011], 31, Some(32));
    test(&[0, 0b1011], 32, Some(32));
    test(&[0, 0b1011], 33, Some(33));
    test(&[0, 0b1011], 34, Some(35));
    test(&[0, 0b1011], 35, Some(35));
    test(&[0, 0b1011], 36, None);
    test(&[0, 0b1011], 100, None);
    test(&[0, 0b1011, 0x0fff_ffff, 0, 1], 91, Some(91));
    test(&[0, 0b1011, 0x0fff_ffff, 0, 1], 92, Some(128));
}

#[test]
fn test_index_of_next_true_bit() {
    let test = |n, u, out| {
        assert_eq!(Natural::from_str(n).unwrap().index_of_next_true_bit(u), out);
        assert_eq!(
            natural_index_of_next_true_bit_alt(&Natural::from_str(n).unwrap(), u),
            out
        );
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .find_one(u32::checked_from(u).unwrap())
                .map(|u| u64::from(u)),
            out
        );
    };
    test("0", 0, None);
    test("0", 100, None);
    test("47244640256", 0, Some(32));
    test("47244640256", 20, Some(32));
    test("47244640256", 31, Some(32));
    test("47244640256", 32, Some(32));
    test("47244640256", 33, Some(33));
    test("47244640256", 34, Some(35));
    test("47244640256", 35, Some(35));
    test("47244640256", 36, None);
    test("47244640256", 100, None);
    test("340282366925890223602069384504899796992", 91, Some(91));
    test("340282366925890223602069384504899796992", 92, Some(128));
}

#[test]
fn limbs_index_of_next_true_bit_properties() {
    test_properties(pairs_of_unsigned_vec_and_small_u64, |&(ref limbs, u)| {
        assert_eq!(
            limbs_index_of_next_true_bit(limbs, u),
            Natural::from_limbs_asc(limbs).index_of_next_true_bit(u)
        );
    });
}

#[test]
fn index_of_next_true_bit_properties() {
    test_properties(pairs_of_natural_and_small_u64, |&(ref n, u)| {
        let result = n.index_of_next_true_bit(u);
        assert_eq!(result, natural_index_of_next_true_bit_alt(n, u));
        assert_eq!(
            natural_to_rug_integer(n)
                .find_one(u32::checked_from(u).unwrap())
                .map(|u| u64::from(u)),
            result
        );
        assert_eq!(result.is_some(), u < n.significant_bits());
        if let Some(result) = result {
            assert!(result >= u);
            assert!(n.get_bit(result));
            assert_eq!(result == u, n.get_bit(u));
        }
        assert_eq!((!n).index_of_next_false_bit(u), result);
    });

    test_properties(naturals, |n| {
        assert_eq!(n.index_of_next_true_bit(0), n.trailing_zeros());
    });

    test_properties(unsigneds, |&u: &u64| {
        assert_eq!(Natural::ZERO.index_of_next_true_bit(u), None);
    });
}