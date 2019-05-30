use malachite_base::crement::Crementable;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::integers;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_increment);
    register_bench!(registry, Large, benchmark_integer_increment);
}

fn demo_integer_increment(gm: GenerationMode, limit: usize) {
    for mut n in integers(gm).take(limit) {
        let n_old = n.clone();
        n.increment();
        println!("n := {:?}; n.increment(); n = {:?}", n_old, n);
    }
}

fn benchmark_integer_increment(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.increment()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [("malachite", &mut (|mut n| n.increment()))],
    );
}
