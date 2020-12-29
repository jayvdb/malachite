use malachite_base::slices::slice_leading_zeros;
use malachite_base_test_util::bench::bucketers::vec_len_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_vec_gen;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_slice_leading_zeros);
    register_bench!(runner, benchmark_slice_leading_zeros);
}

fn demo_slice_leading_zeros(gm: GenMode, config: GenConfig, limit: usize) {
    for xs in unsigned_vec_gen::<u8>().get(gm, &config).take(limit) {
        println!(
            "slice_leading_zeros({:?}) = {}",
            xs,
            slice_leading_zeros(&xs)
        );
    }
}

fn benchmark_slice_leading_zeros(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "slice_leading_zeros(&[T])",
        BenchmarkType::Single,
        unsigned_vec_gen::<u8>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |xs| no_out!(slice_leading_zeros(&xs)))],
    );
}
