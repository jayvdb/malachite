use std::cmp::{max, min};

use malachite_base::num::arithmetic::traits::{EqModPowerOfTwo, ModPowerOfTwo};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::arithmetic::eq_natural_mod_power_of_two::*;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1;
use inputs::integer::{
    triples_of_integer_natural_and_small_unsigned, triples_of_natural_integer_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_eq_mod_power_of_two_neg_pos);
    register_demo!(registry, demo_integer_eq_natural_mod_power_of_two);
    register_demo!(registry, demo_natural_eq_integer_mod_power_of_two);
    register_bench!(registry, Small, benchmark_limbs_eq_mod_power_of_two_neg_pos);
    register_bench!(
        registry,
        Large,
        benchmark_integer_eq_natural_mod_power_of_two_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_eq_integer_mod_power_of_two_algorithms
    );
}

fn demo_limbs_eq_mod_power_of_two_neg_pos(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys, pow) in
        triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1(gm).take(limit)
    {
        println!(
            "limbs_eq_mod_power_of_two_neg_pos({:?}, {:?}, {}) = {:?}",
            xs,
            ys,
            pow,
            limbs_eq_mod_power_of_two_neg_pos(xs, ys, pow)
        );
    }
}

fn demo_integer_eq_natural_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (ref x, ref y, pow) in triples_of_integer_natural_and_small_unsigned(gm).take(limit) {
        println!(
            "{}.eq_mod_power_of_two({}, {}) = {}",
            x,
            y,
            pow,
            x.eq_mod_power_of_two(y, pow)
        );
    }
}

fn demo_natural_eq_integer_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (ref x, ref y, pow) in triples_of_natural_integer_and_small_unsigned(gm).take(limit) {
        println!(
            "{}.eq_mod_power_of_two({}, {}) = {}",
            x,
            y,
            pow,
            x.eq_mod_power_of_two(y, pow)
        );
    }
}

fn benchmark_limbs_eq_mod_power_of_two_neg_pos(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_eq_mod_power_of_two_neg_pos(&[u32], &[u32], u64)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys, pow)| min(usize::checked_from(pow).unwrap(), max(xs.len(), ys.len()))),
        "min(pow, max(xs.len(), ys.len()))",
        &mut [(
            "malachite",
            &mut (|(ref xs, ref ys, pow)| no_out!(limbs_eq_mod_power_of_two_neg_pos(xs, ys, pow))),
        )],
    );
}

fn benchmark_integer_eq_natural_mod_power_of_two_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.eq_mod_power_of_two(&Natural, u64)",
        BenchmarkType::Algorithms,
        triples_of_integer_natural_and_small_unsigned::<u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y, pow)| {
            usize::checked_from(min(pow, max(x.significant_bits(), y.significant_bits()))).unwrap()
        }),
        "min(pow, max(x.significant_bits(), y.significant_bits()))",
        &mut [
            (
                "Integer.eq_mod_power_of_two(&Natural, u64)",
                &mut (|(ref x, ref y, pow)| no_out!(x.eq_mod_power_of_two(y, pow))),
            ),
            (
                "Integer.mod_power_of_two(u64) == Natural.mod_power_of_two(u64)",
                &mut (|(ref x, ref y, pow)| {
                    no_out!(x.mod_power_of_two(pow) == y.mod_power_of_two(pow))
                }),
            ),
        ],
    );
}

fn benchmark_natural_eq_integer_mod_power_of_two_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.eq_mod_power_of_two(&Integer, u64)",
        BenchmarkType::Algorithms,
        triples_of_natural_integer_and_small_unsigned::<u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y, pow)| {
            usize::checked_from(min(pow, max(x.significant_bits(), y.significant_bits()))).unwrap()
        }),
        "min(pow, max(x.significant_bits(), y.significant_bits()))",
        &mut [
            (
                "Natural.eq_mod_power_of_two(&Integer, u64)",
                &mut (|(ref x, ref y, pow)| no_out!(x.eq_mod_power_of_two(y, pow))),
            ),
            (
                "Natural.mod_power_of_two(u64) == Integer.mod_power_of_two(u64)",
                &mut (|(ref x, ref y, pow)| {
                    no_out!(x.mod_power_of_two(pow) == y.mod_power_of_two(pow))
                }),
            ),
        ],
    );
}
