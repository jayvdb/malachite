use std::panic::catch_unwind;

use malachite_base_test_util::num::float::nice_float::NiceFloat;
use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;
use malachite_base_test_util::stats::moments::{moment_stats, CheckedToF64, MomentStats};

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::striped::striped_random_unsigneds;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToBinaryString;

fn striped_random_unsigneds_helper<T: CheckedToF64 + PrimitiveUnsigned>(
    m_numerator: u64,
    m_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (T, Option<T>),
    expected_sample_moment_stats: MomentStats,
) {
    let xs = striped_random_unsigneds::<T>(EXAMPLE_SEED, m_numerator, m_denominator);
    let actual_values = xs
        .clone()
        .map(|x| x.to_binary_string())
        .take(20)
        .collect::<Vec<_>>();
    let actual_common_values = common_values_map(1_000_000, 10, xs.clone())
        .iter()
        .map(|(x, frequency)| (x.to_binary_string(), *frequency))
        .collect::<Vec<_>>();
    let actual_sample_median = median(xs.clone().take(1_000_000));
    let actual_sample_moment_stats = moment_stats(xs.take(1_000_000));
    assert_eq!(
        (
            actual_values,
            actual_common_values,
            actual_sample_median,
            actual_sample_moment_stats
        ),
        (
            expected_values
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
            expected_common_values
                .iter()
                .map(|(x, frequency)| (x.to_string(), *frequency))
                .collect::<Vec<_>>(),
            expected_sample_median,
            expected_sample_moment_stats
        )
    );
}

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_striped_random_unsigneds() {
    // u8, m = 4
    let values = &[
        "1", "1001100", "1111111", "11000011", "0", "10000000", "1111", "1110110", "0", "11111000",
        "11111111", "11111101", "1111001", "0", "11110000", "11", "0", "1111111", "1", "0",
    ];
    let common_values = &[
        ("0", 66602),
        ("11111111", 66546),
        ("11100000", 22466),
        ("11110000", 22460),
        ("11111", 22373),
        ("111111", 22356),
        ("11111100", 22356),
        ("11111110", 22352),
        ("1111", 22281),
        ("11000000", 22273),
    ];
    let sample_median = (128, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(127.56886100000068),
        standard_deviation: NiceFloat(95.37976309187316),
        skewness: NiceFloat(-0.0013289716890443589),
        excess_kurtosis: NiceFloat(-1.5650405989826497),
    };
    striped_random_unsigneds_helper::<u8>(
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // u8, m = 2
    let values = &[
        "110011", "1110010", "1010110", "10110100", "1000001", "11000011", "101000", "110111",
        "11", "11000100", "10111001", "11111011", "111101", "1110011", "10100110", "1010010",
        "1110110", "110011", "11111", "1010011",
    ];
    let common_values = &[
        ("1000100", 4102),
        ("10100100", 4091),
        ("11100100", 4062),
        ("10100010", 4038),
        ("11011010", 4037),
        ("1010100", 4028),
        ("10001011", 4028),
        ("10010010", 4024),
        ("11100011", 4019),
        ("1101011", 4010),
    ];
    let sample_median = (128, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(127.61267700000472),
        standard_deviation: NiceFloat(73.77702717610372),
        skewness: NiceFloat(-0.0007666301824401424),
        excess_kurtosis: NiceFloat(-1.196710018532242),
    };
    striped_random_unsigneds_helper::<u8>(
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // u8, m = 5/4
    let values = &[
        "1010010", "1101010", "110010", "10010100", "1010101", "10111010", "1010100", "1011010",
        "1010110", "10001101", "10000100", "11100011", "1010", "1001011", "10101010", "1010110",
        "1010001", "1010100", "1010101", "1010101",
    ];
    let common_values = &[
        ("1010101", 105174),
        ("10101010", 104734),
        ("10101101", 26535),
        ("11010101", 26470),
        ("1010010", 26420),
        ("101010", 26383),
        ("1001010", 26310),
        ("10010101", 26290),
        ("10110101", 26229),
        ("10101001", 26220),
    ];
    let sample_median = (130, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(127.53448699999743),
        standard_deviation: NiceFloat(54.22754686756281),
        skewness: NiceFloat(-0.0015080269385326522),
        excess_kurtosis: NiceFloat(-1.1008502131352262),
    };
    striped_random_unsigneds_helper::<u8>(
        5,
        4,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // u64, m = 32
    let values = &[
        "111111111111111111111111111111",
        "11111111111111111111111111111111111111111111111111111111111",
        "1111111111111111111111111111111",
        "1111111111111111111111100000000000000000000000000000000000000000",
        "11111111111111111111111111111111111111111111111111111111111111",
        "1111000000000000000000000000000000000000001100000000111111111111",
        "111111111111111000000000000",
        "1111111111111111111111111",
        "111111111111111111111",
        "1111111111111111111111111111111111111111111000000000000000000000",
        "1110000000000000000000000000000000000000000000000000000000000111",
        "1111111111111111111111111111111111111111111111111111111111111111",
        "11111111111111111111111111111000000000000000000000000000",
        "100000001111111111111111111111111000000000",
        "1111111000000000000000000000000000000000000000000000000000000000",
        "1111",
        "1111111111111111111111111111111111111111111111111111000",
        "1111111111111111110000000011111111111111111",
        "0",
        "111111111111111111111111100",
    ];
    let common_values = &[
        (
            "1111111111111111111111111111111111111111111111111111111111111111",
            68034,
        ),
        ("0", 67854),
        ("111111111111111111111111111111", 2299),
        (
            "1111111111111111111111111110000000000000000000000000000000000000",
            2298,
        ),
        ("111111111", 2270),
        ("111111111111111111111111111111111111", 2255),
        ("111111111111", 2254),
        (
            "1111111111111111111111111111111111111111111111100000000000000000",
            2245,
        ),
        (
            "1111111111111110000000000000000000000000000000000000000000000000",
            2240,
        ),
        (
            "1111111111111111111111111111111111111111111111111111111111000000",
            2237,
        ),
    ];
    let sample_median = (9223372036854775808, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(9.232321508867713e18),
        standard_deviation: NiceFloat(8.854393011414399e18),
        skewness: NiceFloat(-0.00195547778002319),
        excess_kurtosis: NiceFloat(-1.9432719051653975),
    };
    striped_random_unsigneds_helper::<u64>(
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // u64, m = 2
    let values = &[
        "11001100011010101001011010010000011000011101011110010000000011",
        "11101101110011111011100001011100110100110101001011101100110011",
        "1111101011000111001110010011000110110101010001110111011111100",
        "1011100110000101000110000000111101100011100111101101111111000000",
        "110011001100101100000100111100101010001011010001101001000111000",
        "1010001101111001011010101011011000110011011110010101100100000100",
        "111100011000100010101011011011001000000100111011110100111011",
        "100100110010110010010111100011001000100110111001010000100101101",
        "1101100111000010011100101110010101101001100110000011111011",
        "1101110011101000110001100100011011100001110011100001101110001000",
        "1101100100110110101110000011000111010011101101101111111101111100",
        "1111001111110101110110010100001100001001011101001101101011101011",
        "111010010110111111001011100000010101010001111000010000110010001",
        "11010101111001000111010001011100101000010001110110001001100111",
        "1101011111000100111101011110111101110011010100111111100001000100",
        "100101000001001011001001101001011100101000110101111110111010101",
        "110111101110000101100101111100101101011100100010101011010100000",
        "101011100111011000001101010001000101111111010001100001111100011",
        "1111100011000100010111100101000001010110011001010100000000011",
        "11100111100011111000111111010010100000111110101001010111011001",
    ];
    let common_values = &[
        ("11011011101110000001110101101011110010111101", 1),
        ("111100000110100100110100100001100010110000010", 1),
        ("1011110101111010110111100110111101111101111011", 1),
        ("1110100010111110000100101010110100101101110111", 1),
        ("10111011110000001110101010110111111001011011100", 1),
        ("11000001010110011001100110101011001100000111111", 1),
        ("100100100011101001001000001010010111100010110100", 1),
        ("100110100101010111001000010010100111110111010110", 1),
        ("110110101100010110011100011100110111001111000000", 1),
        ("111001100010011111010011111010110100101110001000", 1),
    ];
    let sample_median = (9232300347074497346, Some(9232359143244030439));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(9.22650235650532e18),
        standard_deviation: NiceFloat(5.325785817923598e18),
        skewness: NiceFloat(-0.0012562071401776408),
        excess_kurtosis: NiceFloat(-1.1996143045434082),
    };
    striped_random_unsigneds_helper::<u64>(
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // u64, m = 33/32
    let values = &[
        "101010101010101010101011010101010101010101010101010101010101010",
        "101010101010101110101010110101001010101010101010010101010010101",
        "101010010101011010101010101010101101010101010101010101010101010",
        "1010101010101010100101010101010101010101010101010101010101010101",
        "101010101010101010101101010101010100101001010100101010101010101",
        "1010101010101010101010101010101011010101101010101001010101010101",
        "101010101010101010101010101010101010101010101101010101010101010",
        "101010101010110101010101010101010101010101010101010101010101010",
        "10101010101010101010101010101010101010101010101010101010101011",
        "1001010101010101010101010101010101010101010100101010101010101010",
        "1010101010101010101010101100101010101010110101010101010010101010",
        "1010011010101010101010101010101010101010101010101010101010010101",
        "101010101001010101010101010101010101010101010101010101010101010",
        "101010010110110100101010101010101010101010101010101010101010101",
        "1010110101010101010101010101010010101010101010101010101010101010",
        "101010101010101010101010101010101010101010101010101010101010101",
        "101010101010101010101010101010101101010101010101010101010101010",
        "101010101010101010101010010101010101010101010101010101010101010",
        "110110101001010101010010101010101010101010101010101010101010101",
        "101010101010010101010101010101010101010101010101010101010101010",
    ];
    let common_values = &[
        (
            "101010101010101010101010101010101010101010101010101010101010101",
            72208,
        ),
        (
            "1010101010101010101010101010101010101010101010101010101010101010",
            71633,
        ),
        (
            "1010101010101010101010101010101010101010101010101010101010101001",
            2387,
        ),
        (
            "101010101010101010101010101010101010110101010101010101010101010",
            2371,
        ),
        (
            "101010101010101010101001010101010101010101010101010101010101010",
            2350,
        ),
        (
            "1010101010101010101010101010101010101010101010101010101010100101",
            2343,
        ),
        (
            "1010101010101010101010101010101001010101010101010101010101010101",
            2321,
        ),
        (
            "101010101010101010101010101010101010101001010101010101010101010",
            2317,
        ),
        (
            "1010101010101010101010101010101010101010100101010101010101010101",
            2316,
        ),
        (
            "1010101010101010101101010101010101010101010101010101010101010101",
            2314,
        ),
    ];
    let sample_median = (10184128240689698133, Some(10184139957360479594));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(9.22742898450889e18),
        standard_deviation: NiceFloat(3.1984799302251884e18),
        skewness: NiceFloat(-0.0008313832988426654),
        excess_kurtosis: NiceFloat(-1.7364190763714287),
    };
    striped_random_unsigneds_helper::<u64>(
        33,
        32,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

fn striped_random_unsigneds_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(striped_random_unsigneds::<T>(EXAMPLE_SEED, 1, 0));
    assert_panic!(striped_random_unsigneds::<T>(EXAMPLE_SEED, 2, 3));
}

#[test]
fn striped_random_unsigneds_fail() {
    apply_fn_to_unsigneds!(striped_random_unsigneds_fail_helper);
}
