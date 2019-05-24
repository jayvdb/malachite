use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::triples_of_integer_integer_and_signed;
use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{SignificantBits, SubMul, SubMulAssign};
use malachite_nz::platform::SignedLimb;
use std::cmp::max;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_sub_mul_assign_signed_limb);
    register_demo!(registry, demo_integer_sub_mul_assign_signed_limb_ref);
    register_demo!(registry, demo_integer_sub_mul_signed_limb);
    register_demo!(registry, demo_integer_sub_mul_signed_limb_val_ref);
    register_demo!(registry, demo_integer_sub_mul_signed_limb_ref_val);
    register_demo!(registry, demo_integer_sub_mul_signed_limb_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_mul_assign_signed_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_mul_assign_signed_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_mul_assign_signed_limb_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_mul_signed_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_mul_signed_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_mul_signed_limb_val_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_mul_signed_limb_ref_val_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_mul_signed_limb_ref_ref_algorithms
    );
}

fn demo_integer_sub_mul_assign_signed_limb(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_integer_integer_and_signed::<SignedLimb>(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.sub_mul_assign(b, c);
        println!(
            "a := {}; x.sub_mul_assign({}, {}); x = {}",
            a_old, b_old, c, a
        );
    }
}

fn demo_integer_sub_mul_assign_signed_limb_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_integer_integer_and_signed::<SignedLimb>(gm).take(limit) {
        let a_old = a.clone();
        a.sub_mul_assign(&b, c);
        println!("a := {}; x.sub_mul_assign(&{}, {}); x = {}", a_old, b, c, a);
    }
}

fn demo_integer_sub_mul_signed_limb(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integer_integer_and_signed::<SignedLimb>(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!("{}.sub_mul({}, {}) = {}", a_old, b_old, c, a.sub_mul(b, c));
    }
}

fn demo_integer_sub_mul_signed_limb_val_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integer_integer_and_signed::<SignedLimb>(gm).take(limit) {
        let a_old = a.clone();
        println!("{}.sub_mul(&{}, {}) = {}", a_old, b, c, a.sub_mul(&b, c));
    }
}

fn demo_integer_sub_mul_signed_limb_ref_val(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integer_integer_and_signed::<SignedLimb>(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "(&{}).sub_mul({}, {}) = {}",
            a_old,
            b_old,
            c,
            (&a).sub_mul(b, c)
        );
    }
}

fn demo_integer_sub_mul_signed_limb_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integer_integer_and_signed::<SignedLimb>(gm).take(limit) {
        let a_old = a.clone();
        println!(
            "(&{}).sub_mul(&{}, {}) = {}",
            a_old,
            b,
            c,
            (&a).sub_mul(&b, c)
        );
    }
}

fn benchmark_integer_sub_mul_assign_signed_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.sub_mul_assign(Integer, SignedLimb)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integer_integer_and_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Integer.sub_mul_assign(Integer, SignedLimb)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(b, c)),
            ),
            (
                "Integer.sub_mul_assign(&Integer, SignedLimb)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(&b, c)),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_assign_signed_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.sub_mul_assign(Integer, SignedLimb)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Integer.sub_mul_assign(Integer, SignedLimb)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(b, c)),
            ),
            (
                "Integer -= Integer * SignedLimb",
                &mut (|(mut a, b, c)| a -= b * c),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_assign_signed_limb_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.sub_mul_assign(&Integer, SignedLimb)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Integer.sub_mul_assign(&Integer, SignedLimb)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(&b, c)),
            ),
            (
                "Integer -= &Integer * SignedLimb",
                &mut (|(mut a, b, c)| a -= &b * c),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_signed_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.sub_mul(Integer, SignedLimb)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integer_integer_and_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Integer.sub_mul(Integer, SignedLimb)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(b, c))),
            ),
            (
                "Integer.sub_mul(&Integer, SignedLimb)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(&b, c))),
            ),
            (
                "(&Integer).sub_mul(Integer, SignedLimb)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(b, c))),
            ),
            (
                "(&Integer).sub_mul(&Integer, SignedLimb)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(&b, c))),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_signed_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.sub_mul(Integer, SignedLimb)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Integer.sub_mul(Integer, SignedLimb)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(b, c))),
            ),
            (
                "Integer - Integer * SignedLimb",
                &mut (|(a, b, c)| no_out!(a - b * c)),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_signed_limb_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.sub_mul(&Integer, SignedLimb)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Integer.sub_mul(&Integer, SignedLimb)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(&b, c))),
            ),
            (
                "Integer - &Integer * SignedLimb",
                &mut (|(a, b, c)| no_out!(a - &b * c)),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_signed_limb_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "(&Integer).sub_mul(Integer, SignedLimb)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "(&Integer).sub_mul(Integer, SignedLimb)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(b, c))),
            ),
            (
                "(&Integer) - Integer * SignedLimb",
                &mut (|(a, b, c)| no_out!(&a - b * c)),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_signed_limb_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "(&Integer).sub_mul(&Integer, SignedLimb)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "(&Integer).sub_mul(&Integer, SignedLimb)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(&b, c))),
            ),
            (
                "(&Integer) - &Integer * SignedLimb",
                &mut (|(a, b, c)| no_out!(&a - &b * c)),
            ),
        ],
    );
}
