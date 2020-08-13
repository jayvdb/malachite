use std::panic::catch_unwind;

use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn get_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n: u64, index, out| {
        assert_eq!(T::exact_from(n).get_bit(index), out);
    };

    test(0, 0, false);
    test(0, 100, false);
    test(123, 2, false);
    test(123, 3, true);
    test(123, 100, false);
    if T::WIDTH >= u64::WIDTH {
        test(1_000_000_000_000, 12, true);
        test(1_000_000_000_000, 100, false);
    }
}

fn get_bit_helper_signed<T: PrimitiveSigned>() {
    get_bit_helper_unsigned::<T>();

    let test = |n: i64, index, out| {
        assert_eq!(T::exact_from(n).get_bit(index), out);
    };

    test(-123, 0, true);
    test(-123, 1, false);
    test(-123, 100, true);
    if T::WIDTH >= u64::WIDTH {
        test(-1_000_000_000_000, 12, true);
        test(-1_000_000_000_000, 100, true);
        test(-i64::from(u32::MAX), 0, true);
        test(-i64::from(u32::MAX), 1, false);
        test(-i64::from(u32::MAX), 31, false);
        test(-i64::from(u32::MAX), 32, true);
        test(-i64::from(u32::MAX), 33, true);
        test(-i64::from(u32::MAX) - 1, 0, false);
        test(-i64::from(u32::MAX) - 1, 31, false);
        test(-i64::from(u32::MAX) - 1, 32, true);
        test(-i64::from(u32::MAX) - 1, 33, true);
    }
}

#[test]
fn test_get_bit() {
    apply_fn_to_unsigneds!(get_bit_helper_unsigned);
    apply_fn_to_signeds!(get_bit_helper_signed);
}

fn set_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n: u64, index, out: u64| {
        let mut n = T::exact_from(n);
        n.set_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(100, 0, 101);
    if T::WIDTH >= u16::WIDTH {
        test(0, 10, 1024);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1_000_000_000_000, 10, 1_000_000_001_024);
    }
}

fn set_bit_helper_signed<T: PrimitiveSigned>() {
    set_bit_helper_unsigned::<T>();

    let test = |n: i64, index, out: i64| {
        let mut n = T::exact_from(n);
        n.set_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(-1, 5, -1);
    test(-1, 100, -1);
    test(-33, 5, -1);
    test(-32, 0, -31);

    if T::WIDTH >= u64::WIDTH {
        test(-1_000_000_000_000, 10, -999_999_998_976);
        test(-1_000_000_000_000, 100, -1_000_000_000_000);
    }
}

#[test]
fn test_set_bit() {
    apply_fn_to_unsigneds!(set_bit_helper_unsigned);
    apply_fn_to_signeds!(set_bit_helper_signed);
}

fn set_bit_fail_helper<T: PrimitiveInteger>() {
    assert_panic!({
        let mut n = T::exact_from(5);
        n.set_bit(200);
    });
}

#[test]
fn set_bit_fail() {
    apply_fn_to_primitive_ints!(set_bit_fail_helper);
}

fn clear_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n: u64, index, out: u64| {
        let mut n = T::exact_from(n);
        n.clear_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(0, 10, 0);
    test(0, 100, 0);
    test(101, 0, 100);
    if T::WIDTH >= u16::WIDTH {
        test(1024, 10, 0);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1_000_000_001_024, 10, 1_000_000_000_000);
        test(1_000_000_001_024, 100, 1_000_000_001_024);
    }
}

fn clear_bit_helper_signed<T: PrimitiveSigned>() {
    clear_bit_helper_unsigned::<T>();

    let test = |n: i64, index, out: i64| {
        let mut n = T::exact_from(n);
        n.clear_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(-1, 5, -33);
    test(-31, 0, -32);

    if T::WIDTH >= u64::WIDTH {
        test(-999_999_998_976, 10, -1_000_000_000_000);
    }
}

#[test]
fn test_clear_bit() {
    apply_fn_to_unsigneds!(clear_bit_helper_unsigned);
    apply_fn_to_signeds!(clear_bit_helper_signed);
}

fn clear_bit_fail_helper<T: PrimitiveSigned>() {
    assert_panic!({
        let mut n = T::NEGATIVE_ONE;
        n.clear_bit(200);
    });
}

#[test]
fn clear_bit_fail() {
    apply_fn_to_signeds!(clear_bit_fail_helper);
}

fn assign_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n: u64, index, bit, out: u64| {
        let mut n = T::exact_from(n);
        n.assign_bit(index, bit);
        assert_eq!(n, T::exact_from(out));
    };

    test(100, 0, true, 101);
    test(0, 10, false, 0);
    test(0, 100, false, 0);
    test(101, 0, false, 100);
    if T::WIDTH >= u16::WIDTH {
        test(0, 10, true, 1024);
        test(1024, 10, false, 0);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1_000_000_000_000, 10, true, 1_000_000_001_024);
        test(1_000_000_001_024, 10, false, 1_000_000_000_000);
        test(1_000_000_001_024, 100, false, 1_000_000_001_024);
    }
}

fn assign_bit_helper_signed<T: PrimitiveSigned>() {
    assign_bit_helper_unsigned::<T>();

    let test = |n: i64, index, bit, out: i64| {
        let mut n = T::exact_from(n);
        n.assign_bit(index, bit);
        assert_eq!(n, T::exact_from(out));
    };

    test(-1, 5, true, -1);
    test(-1, 100, true, -1);
    test(-33, 5, true, -1);
    test(-32, 0, true, -31);
    test(-1, 5, false, -33);
    test(-31, 0, false, -32);

    if T::WIDTH >= u64::WIDTH {
        test(-1_000_000_000_000, 10, true, -999_999_998_976);
        test(-1_000_000_000_000, 100, true, -1_000_000_000_000);
        test(-999_999_998_976, 10, false, -1_000_000_000_000);
    }
}

#[test]
fn test_assign_bit() {
    apply_fn_to_unsigneds!(assign_bit_helper_unsigned);
    apply_fn_to_signeds!(assign_bit_helper_signed);
}

fn assign_bit_fail_helper<T: PrimitiveInteger>() {
    assert_panic!({
        let mut n = T::exact_from(5);
        n.assign_bit(200, true);
    });
}

fn assign_bit_fail_helper_signed<T: PrimitiveSigned>() {
    assert_panic!({
        let mut n = T::NEGATIVE_ONE;
        n.assign_bit(200, false);
    });
}

#[test]
fn assign_bit_fail() {
    apply_fn_to_primitive_ints!(assign_bit_fail_helper);
    apply_fn_to_signeds!(assign_bit_fail_helper_signed);
}

fn flip_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n: u64, index, out: u64| {
        let mut n = T::exact_from(n);
        n.flip_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(100, 0, 101);
    test(101, 0, 100);
    if T::WIDTH >= u16::WIDTH {
        test(0, 10, 1024);
        test(1024, 10, 0);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1_000_000_000_000, 10, 1_000_000_001_024);
        test(1_000_000_001_024, 10, 1_000_000_000_000);
    }
}

fn flip_bit_helper_signed<T: PrimitiveSigned>() {
    flip_bit_helper_unsigned::<T>();

    let test = |n: i64, index, out: i64| {
        let mut n = T::exact_from(n);
        n.flip_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(-1, 5, -33);
    test(-33, 5, -1);
    test(-32, 0, -31);
    test(-31, 0, -32);

    if T::WIDTH >= u64::WIDTH {
        test(-1_000_000_000_000, 10, -999_999_998_976);
        test(-999_999_998_976, 10, -1_000_000_000_000);
    }
}

#[test]
fn test_flip_bit() {
    apply_fn_to_unsigneds!(flip_bit_helper_unsigned);
    apply_fn_to_signeds!(flip_bit_helper_signed);
}

fn flip_bit_fail_helper_unsigned<T: PrimitiveUnsigned>() {
    assert_panic!(T::exact_from(5).flip_bit(200));
}

fn flip_bit_fail_helper_signed<T: PrimitiveSigned>() {
    assert_panic!(T::exact_from(5).flip_bit(200));
    assert_panic!(T::NEGATIVE_ONE.flip_bit(200));
}

#[test]
fn flip_bit_fail() {
    apply_fn_to_unsigneds!(flip_bit_fail_helper_unsigned);
    apply_fn_to_signeds!(flip_bit_fail_helper_signed);
}
