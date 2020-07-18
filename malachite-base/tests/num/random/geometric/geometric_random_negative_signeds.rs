use malachite_base_test_util::num::float::nice_float::NiceFloat;
use malachite_base_test_util::stats::moments::{
    negative_truncated_geometric_dist_assertions, CheckedToF64, MomentStats,
};

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::random::geometric::geometric_random_negative_signeds;
use malachite_base::random::EXAMPLE_SEED;

fn geometric_random_negative_signeds_helper<T: CheckedToF64 + PrimitiveSigned>(
    abs_um_numerator: u64,
    abs_um_denominator: u64,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: (T, Option<T>),
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) where
    T: WrappingFrom<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: CheckedToF64 + PrimitiveUnsigned,
{
    negative_truncated_geometric_dist_assertions(
        geometric_random_negative_signeds::<T>(EXAMPLE_SEED, abs_um_numerator, abs_um_denominator),
        abs_um_numerator,
        abs_um_denominator,
        T::NEGATIVE_ONE,
        T::MIN,
        expected_values,
        expected_common_values,
        expected_pop_median,
        expected_sample_median,
        expected_pop_moment_stats,
        expected_sample_moment_stats,
    );
}

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_geometric_random_negative_signeds() {
    // i64, um = -65 / 64
    let values = &[
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -2,
    ];
    let common_values = &[(-1, 984537), (-2, 15210), (-3, 253)];
    let pop_median = (-1, None);
    let sample_median = (-1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-1.015625),
        standard_deviation: NiceFloat(0.12597277731716458),
        skewness: NiceFloat(-8.186292482887549),
        excess_kurtosis: NiceFloat(69.01538461538773),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.0157160000000027),
        standard_deviation: NiceFloat(0.12639233884624257),
        skewness: NiceFloat(-8.160460378454992),
        excess_kurtosis: NiceFloat(68.31033619043166),
    };
    geometric_random_negative_signeds_helper::<i64>(
        65,
        64,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i64, um = -12345/10000
    let values = &[
        -1, -1, -1, -1, -1, -1, -1, -1, -2, -1, -1, -1, -1, -1, -2, -2, -1, -1, -1, -1,
    ];
    let common_values = &[
        (-1, 809395),
        (-2, 154707),
        (-3, 29037),
        (-4, 5577),
        (-5, 1053),
        (-6, 185),
        (-7, 40),
        (-8, 5),
        (-9, 1),
    ];
    let pop_median = (-1, None);
    let sample_median = (-1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-1.2345),
        standard_deviation: NiceFloat(0.538042981554448),
        skewness: NiceFloat(-2.730265146765687),
        excess_kurtosis: NiceFloat(9.45434777164341),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.2349320000000084),
        standard_deviation: NiceFloat(0.5376590410783139),
        skewness: NiceFloat(-2.716807176148366),
        excess_kurtosis: NiceFloat(9.308727522629948),
    };
    geometric_random_negative_signeds_helper::<i64>(
        12_345,
        10_000,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i64, um = -2
    let values = &[
        -2, -1, -1, -4, -5, -5, -2, -1, -1, -2, -1, -1, -3, -3, -1, -1, -2, -1, -4, -2,
    ];
    let common_values = &[
        (-1, 500085),
        (-2, 249510),
        (-3, 125328),
        (-4, 62428),
        (-5, 31280),
        (-6, 15676),
        (-7, 7853),
        (-8, 3994),
        (-9, 1932),
        (-10, 942),
    ];
    let pop_median = (-2, Some(-1));
    let sample_median = (-1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-2.0),
        standard_deviation: NiceFloat(std::f64::consts::SQRT_2),
        skewness: NiceFloat(-2.1213203435596424),
        excess_kurtosis: NiceFloat(6.5),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-2.0006159999998947),
        standard_deviation: NiceFloat(1.414547850547892),
        skewness: NiceFloat(-2.114056944012543),
        excess_kurtosis: NiceFloat(6.4341815215340645),
    };
    geometric_random_negative_signeds_helper::<i64>(
        2,
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, um = -64
    let values = &[
        -53, -56, -71, -20, -113, -87, -55, -44, -4, -17, -44, -36, -5, -25, -55, -9, -114, -24,
        -15, -76,
    ];
    let common_values = &[
        (-1, 18323),
        (-2, 17814),
        (-3, 17620),
        (-4, 17269),
        (-5, 17057),
        (-6, 16684),
        (-7, 16509),
        (-8, 16164),
        (-9, 15914),
        (-10, 15665),
    ];
    let pop_median = (-37, None);
    let sample_median = (-37, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-44.32782748571844),
        standard_deviation: NiceFloat(33.57033730453578),
        skewness: NiceFloat(-0.6852077716881376),
        excess_kurtosis: NiceFloat(-0.5394700086619175),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-44.31098999999839),
        standard_deviation: NiceFloat(33.572216166252275),
        skewness: NiceFloat(-0.6830069791725721),
        excess_kurtosis: NiceFloat(-0.5435066974705061),
    };
    geometric_random_negative_signeds_helper::<i8>(
        64,
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, um = -1000
    let values = &[
        -25, -111, -118, -26, -101, -88, -81, -112, -38, -36, -68, -76, -41, -36, -118, -66, -123,
        -120, -128, -92,
    ];
    let common_values = &[
        (-1, 8409),
        (-6, 8360),
        (-2, 8357),
        (-7, 8328),
        (-5, 8309),
        (-14, 8300),
        (-22, 8268),
        (-12, 8262),
        (-18, 8254),
        (-9, 8249),
    ];
    let pop_median = (-62, None);
    let sample_median = (-63, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-63.134440160748206),
        standard_deviation: NiceFloat(36.934145822824206),
        skewness: NiceFloat(-0.04436357375743275),
        excess_kurtosis: NiceFloat(-1.1974286638022889),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-63.15733700000316),
        standard_deviation: NiceFloat(36.91734106024568),
        skewness: NiceFloat(-0.04254845146202895),
        excess_kurtosis: NiceFloat(-1.1968222027376492),
    };
    geometric_random_negative_signeds_helper::<i8>(
        1000,
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}

macro_rules! geometric_random_negative_signeds_fail {
    (
        $t:ident,
        $geometric_random_negative_signeds_fail_1:ident,
        $geometric_random_negative_signeds_fail_2:ident
    ) => {
        #[test]
        #[should_panic]
        fn $geometric_random_negative_signeds_fail_1() {
            geometric_random_negative_signeds::<$t>(EXAMPLE_SEED, 1, 0);
        }

        #[test]
        #[should_panic]
        fn $geometric_random_negative_signeds_fail_2() {
            geometric_random_negative_signeds::<$t>(EXAMPLE_SEED, 2, 3);
        }
    };
}

geometric_random_negative_signeds_fail!(
    i8,
    geometric_random_negative_signeds_i8_fail_1,
    geometric_random_negative_signeds_i8_fail_2
);
geometric_random_negative_signeds_fail!(
    i16,
    geometric_random_negative_signeds_i16_fail_1,
    geometric_random_negative_signeds_i16_fail_2
);
geometric_random_negative_signeds_fail!(
    i32,
    geometric_random_negative_signeds_i32_fail_1,
    geometric_random_negative_signeds_i32_fail_2
);
geometric_random_negative_signeds_fail!(
    i64,
    geometric_random_negative_signeds_i64_fail_1,
    geometric_random_negative_signeds_i64_fail_2
);
geometric_random_negative_signeds_fail!(
    i128,
    geometric_random_negative_signeds_i128_fail_1,
    geometric_random_negative_signeds_i128_fail_2
);
geometric_random_negative_signeds_fail!(
    isize,
    geometric_random_negative_signeds_isize_fail_1,
    geometric_random_negative_signeds_isize_fail_2
);
