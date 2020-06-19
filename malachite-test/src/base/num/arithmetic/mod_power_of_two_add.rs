use std::cmp::max;

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use rand::distributions::range::SampleRange;
use rand::Rand;

use common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, NoSpecialGenerationMode, ScaleType,
};
use inputs::base::triples_of_unsigned_unsigned_and_small_u64_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_u8_mod_power_of_two_add);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_add);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_add);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_add);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_add);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_add_assign);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_add_assign);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_add_assign);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_add_assign);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_add_assign);

    register_ns_bench!(registry, None, benchmark_u8_mod_power_of_two_add);
    register_ns_bench!(registry, None, benchmark_u16_mod_power_of_two_add);
    register_ns_bench!(registry, None, benchmark_u32_mod_power_of_two_add);
    register_ns_bench!(registry, None, benchmark_u64_mod_power_of_two_add);
    register_ns_bench!(registry, None, benchmark_usize_mod_power_of_two_add);
    register_ns_bench!(registry, None, benchmark_u8_mod_power_of_two_add_assign);
    register_ns_bench!(registry, None, benchmark_u16_mod_power_of_two_add_assign);
    register_ns_bench!(registry, None, benchmark_u32_mod_power_of_two_add_assign);
    register_ns_bench!(registry, None, benchmark_u64_mod_power_of_two_add_assign);
    register_ns_bench!(registry, None, benchmark_usize_mod_power_of_two_add_assign);
}

fn demo_mod_power_of_two_add<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y, pow) in triples_of_unsigned_unsigned_and_small_u64_var_1::<T>(gm).take(limit) {
        println!(
            "{} + {} === {} mod 2^{}",
            x,
            y,
            x.mod_power_of_two_add(y, pow),
            pow
        );
    }
}

fn demo_mod_power_of_two_add_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (mut x, y, pow) in triples_of_unsigned_unsigned_and_small_u64_var_1::<T>(gm).take(limit) {
        let old_x = x;
        x.mod_power_of_two_add_assign(y, pow);
        println!(
            "x := {}; x.mod_power_of_two_add_assign({}, {}); x = {}",
            old_x, y, pow, x
        );
    }
}

fn benchmark_mod_power_of_two_add<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.mod_power_of_two_add({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigned_unsigned_and_small_u64_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(x, y, pow)| no_out!(x.mod_power_of_two_add(y, pow))),
        )],
    );
}

fn benchmark_mod_power_of_two_add_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.mod_power_of_two_add_assign({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigned_unsigned_and_small_u64_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(mut x, y, pow)| x.mod_power_of_two_add_assign(y, pow)),
        )],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_mod_power_of_two_add::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_mod_power_of_two_add_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_power_of_two_add::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_power_of_two_add_assign::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_mod_power_of_two_add,
    demo_u8_mod_power_of_two_add_assign,
    benchmark_u8_mod_power_of_two_add,
    benchmark_u8_mod_power_of_two_add_assign
);
unsigned!(
    u16,
    demo_u16_mod_power_of_two_add,
    demo_u16_mod_power_of_two_add_assign,
    benchmark_u16_mod_power_of_two_add,
    benchmark_u16_mod_power_of_two_add_assign
);
unsigned!(
    u32,
    demo_u32_mod_power_of_two_add,
    demo_u32_mod_power_of_two_add_assign,
    benchmark_u32_mod_power_of_two_add,
    benchmark_u32_mod_power_of_two_add_assign
);
unsigned!(
    u64,
    demo_u64_mod_power_of_two_add,
    demo_u64_mod_power_of_two_add_assign,
    benchmark_u64_mod_power_of_two_add,
    benchmark_u64_mod_power_of_two_add_assign
);
unsigned!(
    usize,
    demo_usize_mod_power_of_two_add,
    demo_usize_mod_power_of_two_add_assign,
    benchmark_usize_mod_power_of_two_add,
    benchmark_usize_mod_power_of_two_add_assign
);