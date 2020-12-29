use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::{IsaacRng, SeedableRng};
use rust_wheels::iterators::adaptors::{generate_from_function, to_limited_string_binary};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::special_random_natural_with_bits_old;

use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use malachite_test::inputs::base::small_unsigneds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_natural_special_random_natural_with_bits);
    register_ns_bench!(
        registry,
        Large,
        benchmark_natural_special_random_natural_with_bits
    );
}

fn demo_natural_special_random_natural_with_bits(gm: NoSpecialGenerationMode, limit: usize) {
    for bits in small_unsigneds(gm).take(limit) {
        let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
        let mut xs =
            generate_from_function(|| special_random_natural_with_bits_old(&mut rng, bits));
        println!(
            "special_random_natural_with_bits({}) = {}",
            bits,
            to_limited_string_binary(10, &mut xs)
        );
    }
}

fn benchmark_natural_special_random_natural_with_bits(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    run_benchmark_old(
        "special_random_natural_with_bits(&mut Rng, u64)",
        BenchmarkType::Single,
        small_unsigneds(gm),
        gm.name(),
        limit,
        file_name,
        &(|&bits| usize::exact_from(bits)),
        "bits",
        &mut [(
            "Malachite",
            &mut (|bits| no_out!(special_random_natural_with_bits_old(&mut rng, bits))),
        )],
    );
}
