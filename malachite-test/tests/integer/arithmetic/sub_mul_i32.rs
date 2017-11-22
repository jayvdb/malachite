use common::LARGE_LIMIT;
use malachite_base::traits::{NegativeOne, One, SubMul, SubMulAssign, Zero};
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_gmp};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_pairs_from_single,
                                     exhaustive_triples, random_pairs, random_pairs_from_single,
                                     random_triples};
use std::str::FromStr;

#[test]
fn test_sub_mul_i32() {
    #[allow(cyclomatic_complexity)]
    let test = |u, v, c: i32, out| {
        let mut a = native::Integer::from_str(u).unwrap();
        a.sub_mul_assign(native::Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = gmp::Integer::from_str(u).unwrap();
        a.sub_mul_assign(gmp::Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = native::Integer::from_str(u).unwrap();
        a.sub_mul_assign(&native::Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = gmp::Integer::from_str(u).unwrap();
        a.sub_mul_assign(&gmp::Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = native::Integer::from_str(u).unwrap().sub_mul(
            native::Integer::from_str(v).unwrap(),
            c,
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = gmp::Integer::from_str(u).unwrap().sub_mul(
            gmp::Integer::from_str(v)
                .unwrap(),
            c,
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = native::Integer::from_str(u).unwrap().sub_mul(
            &native::Integer::from_str(v).unwrap(),
            c,
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = gmp::Integer::from_str(u).unwrap().sub_mul(
            &gmp::Integer::from_str(v)
                .unwrap(),
            c,
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&native::Integer::from_str(u).unwrap()).sub_mul(
            native::Integer::from_str(v).unwrap(),
            c,
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a =
            (&gmp::Integer::from_str(u).unwrap()).sub_mul(gmp::Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&native::Integer::from_str(u).unwrap()).sub_mul(
            &native::Integer::from_str(v).unwrap(),
            c,
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a =
            (&gmp::Integer::from_str(u).unwrap()).sub_mul(&gmp::Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());
    };
    test("0", "0", 0, "0");
    test("0", "0", -123, "0");
    test("123", "0", -5, "123");
    test("-123", "0", -5, "-123");
    test("123", "5", -1, "128");
    test("-123", "5", -1, "-118");
    test("123", "5", -100, "623");
    test("-123", "5", -100, "377");
    test("10", "3", -4, "22");
    test("10", "-3", -4, "-2");
    test("1000000000000", "0", -123, "1000000000000");
    test("1000000000000", "1", -123, "1000000000123");
    test("1000000000000", "123", -1, "1000000000123");
    test("1000000000000", "123", -100, "1000000012300");
    test("1000000000000", "100", -123, "1000000012300");
    test("1000000000000", "65536", -65536, "1004294967296");
    test("-1000000000000", "-65536", -65536, "-1004294967296");
    test("-1000000000000", "65536", -65536, "-995705032704");
    test("1000000000000", "-65536", -65536, "995705032704");
    test("1000000000000", "1000000000000", 0, "1000000000000");
    test("1000000000000", "1000000000000", -1, "2000000000000");
    test("1000000000000", "1000000000000", -100, "101000000000000");
    test("0", "1000000000000", -100, "100000000000000");
    test("-1", "1000000000000", -100, "99999999999999");
    test("0", "-1000000000000", -100, "-100000000000000");
    test("1", "-1000000000000", -100, "-99999999999999");
    test("0", "0", 123, "0");
    test("-123", "0", 5, "-123");
    test("123", "0", 5, "123");
    test("-123", "5", 1, "-128");
    test("123", "5", 1, "118");
    test("-123", "5", 100, "-623");
    test("123", "5", 100, "-377");
    test("-10", "3", 4, "-22");
    test("-10", "-3", 4, "2");
    test("-1000000000000", "0", 123, "-1000000000000");
    test("-1000000000000", "1", 123, "-1000000000123");
    test("-1000000000000", "123", 1, "-1000000000123");
    test("-1000000000000", "123", 100, "-1000000012300");
    test("-1000000000000", "100", 123, "-1000000012300");
    test("-1000000000000", "65536", 65536, "-1004294967296");
    test("1000000000000", "-65536", 65536, "1004294967296");
    test("1000000000000", "65536", 65536, "995705032704");
    test("-1000000000000", "-65536", 65536, "-995705032704");
    test("-1000000000000", "1000000000000", 1, "-2000000000000");
    test("-1000000000000", "1000000000000", 100, "-101000000000000");
    test("0", "1000000000000", 100, "-100000000000000");
    test("1", "1000000000000", 100, "-99999999999999");
    test("0", "-1000000000000", 100, "100000000000000");
    test("-1", "-1000000000000", 100, "99999999999999");
    test("1000000000000", "1000000000000", 1, "0");
    test("1000000000000", "-1000000000000", -1, "0");
    test("-1000000000000", "-1000000000000", 1, "0");
    test("1000000000000", "-1000000000000", -1, "0");
    test("1000000000000000000000", "1000000000000", 1000000000, "0");
    test("1000000000000000000000", "-1000000000000", -1000000000, "0");
    test("-1000000000000000000000", "-1000000000000", 1000000000, "0");
    test("1000000000000000000000", "-1000000000000", -1000000000, "0");
}

#[test]
fn sub_mul_i32_properties() {
    // a.sub_mul_assign(b, c) is equivalent for malachite-gmp and malachite-native.
    // a.sub_mul_assign(&b, c) is equivalent for malachite-gmp and malachite-native.
    // a.sub_mul(b, c) is equivalent for malachite-gmp and malachite-native.
    // a.sub_mul(&b, c) is equivalent for malachite-gmp and malachite-native.
    // (&a).sub_mul(b, c) is equivalent for malachite-gmp and malachite-native.
    // (&a).sub_mul(&b, c) is equivalent for malachite-gmp and malachite-native.
    // a.sub_mul_assign(b, c); a is valid.
    // a.sub_mul_assign(&b, c); a is valid.
    // a.sub_mul(b, c) is valid.
    // a.sub_mul(&b, c) is valid.
    // (&a).sub_mul(b, c) is valid.
    // (&a).sub_mul(&b, c) is valid.
    // a.sub_mul_assign(b, c), a.sub_mul_assign(&b, c), a.sub_mul(b, c), a.sub_mul(&b, c),
    //      (&a).sub_mul(b, c), and (&a).sub_mul(&b, c) give the same result.
    // a.sub_mul(&b, c) is equivalent to a - b * c.
    // a.sub_mul(b, c) = a.sub_mul(-b, -c)
    // -(a.sub_mul(b, c)) = (-a).sub_mul(-b, c) = (-a).sub_mul(b, -c)
    // a.sub_mul(&b, c) is equivalent to a.sub_mul(&b, Integer::from(c))
    let integer_integer_and_i32 = |mut gmp_a: gmp::Integer, gmp_b: gmp::Integer, c: i32| {
        let mut a = gmp_integer_to_native(&gmp_a);
        let b = gmp_integer_to_native(&gmp_b);
        let old_a = a.clone();
        gmp_a.sub_mul_assign(gmp_b.clone(), c);
        assert!(gmp_a.is_valid());

        let mut gmp_a_2 = native_integer_to_gmp(&old_a);
        gmp_a_2.sub_mul_assign(&gmp_b, c);
        assert!(gmp_a_2.is_valid());
        assert_eq!(gmp_a_2, gmp_a);

        a.sub_mul_assign(b.clone(), c);
        assert!(a.is_valid());
        assert_eq!(gmp_integer_to_native(&gmp_a), a);

        let mut a2 = old_a.clone();
        a2.sub_mul_assign(&b, c);
        assert!(a2.is_valid());
        assert_eq!(a2, a);

        let gmp_a_2 = native_integer_to_gmp(&old_a);
        let result = (&gmp_a_2).sub_mul(gmp_b.clone(), c);
        assert!(result.is_valid());
        assert_eq!(result, gmp_a);

        let result = (&gmp_a_2).sub_mul(&gmp_b, c);
        assert!(result.is_valid());
        assert_eq!(result, gmp_a);

        let result = gmp_a_2.clone().sub_mul(gmp_b.clone(), c);
        assert!(result.is_valid());
        assert_eq!(result, gmp_a);

        let result = gmp_a_2.sub_mul(&gmp_b, c);
        assert!(result.is_valid());
        assert_eq!(result, gmp_a);

        let a2 = old_a.clone();
        let result = (&a2).sub_mul(b.clone(), c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        let a2 = old_a.clone();
        let result = (&a2).sub_mul(&b, c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = a2.clone().sub_mul(b.clone(), c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = a2.sub_mul(&b, c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        assert_eq!(&old_a - &b * c, result);
        assert_eq!((&old_a).sub_mul(-&b, -c), result);
        assert_eq!((-&old_a).sub_mul(-&b, c), -&result);
        assert_eq!((-&old_a).sub_mul(&b, -c), -&result);
        assert_eq!(old_a.sub_mul(b, native::Integer::from(c)), result);
    };

    // n.sub_mul(n, 1) == 0
    // n.sub_mul(-n, -1) == 0
    let single_integer = |gmp_n: gmp::Integer| {
        let n = &gmp_integer_to_native(&gmp_n);
        assert_eq!(n.sub_mul(n, 1), 0);
        assert_eq!(n.sub_mul(-n, -1), 0);
    };

    // n.sub_mul(0, c) == n
    // n.sub_mul(1, c) == n - c
    // n.sub_mul(-1, c) == n + c
    // 0.sub_mul(n, c) == -n * c
    // (n * c).sub_mul(n, c) == 0
    // (n * c).sub_mul(-n, -c) == 0
    let integer_and_i32 = |gmp_n: gmp::Integer, c: i32| {
        let n = &gmp_integer_to_native(&gmp_n);
        assert_eq!(n.sub_mul(&native::Integer::zero(), c), *n);
        assert_eq!(n.sub_mul(&native::Integer::one(), c), n - c);
        assert_eq!(n.sub_mul(&native::Integer::negative_one(), c), n + c);
        assert_eq!(native::Integer::zero().sub_mul(n, c), -n * c);
        assert_eq!((n * c).sub_mul(n, c), 0);
        assert_eq!((n * c).sub_mul(-n, -c), 0);
    };

    // a.sub_mul(b, 0) == a
    // a.sub_mul(b, 1) == a - b
    // a.sub_mul(b, -1) == a + b
    let two_integers = |gmp_a: gmp::Integer, gmp_b: gmp::Integer| {
        let a = &gmp_integer_to_native(&gmp_a);
        let b = &gmp_integer_to_native(&gmp_b);
        assert_eq!(a.sub_mul(b, 0), *a);
        assert_eq!(a.sub_mul(b, 1), a - b);
        assert_eq!(a.sub_mul(b, -1), a + b);
    };

    for (a, b, c) in exhaustive_triples(
        exhaustive_integers(),
        exhaustive_integers(),
        exhaustive_i::<i32>(),
    ).take(LARGE_LIMIT)
    {
        integer_integer_and_i32(a, b, c);
    }

    for (a, b, c) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x(seed)),
    ).take(LARGE_LIMIT)
    {
        integer_integer_and_i32(a, b, c);
    }

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        single_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        single_integer(n);
    }

    for (n, c) in exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()).take(LARGE_LIMIT) {
        integer_and_i32(n, c);
    }

    for (n, c) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x(seed)),
    ).take(LARGE_LIMIT)
    {
        integer_and_i32(n, c);
    }

    for (a, b) in exhaustive_pairs_from_single(exhaustive_integers()).take(LARGE_LIMIT) {
        two_integers(a, b);
    }

    for (a, b) in random_pairs_from_single(random_integers(&EXAMPLE_SEED, 32)).take(LARGE_LIMIT) {
        two_integers(a, b);
    }
}