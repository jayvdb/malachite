use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::pair_2_pair_float_integer_max_complexity_bucketer;
use malachite_float::test_util::generators::{float_integer_pair_gen, float_integer_pair_gen_rm};
use malachite_float::ComparableFloatRef;
use std::cmp::Ordering;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_partial_cmp_integer);
    register_demo!(runner, demo_float_partial_cmp_integer_debug);
    register_demo!(runner, demo_integer_partial_cmp_float);
    register_demo!(runner, demo_integer_partial_cmp_float_debug);

    register_bench!(
        runner,
        benchmark_float_partial_cmp_integer_library_comparison
    );
    register_bench!(
        runner,
        benchmark_integer_partial_cmp_float_library_comparison
    );
}

fn demo_float_partial_cmp_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_integer_pair_gen().get(gm, config).take(limit) {
        match x.partial_cmp(&y) {
            None => println!("{x} and {y} are incomparable"),
            Some(Ordering::Less) => println!("{x} < {y}"),
            Some(Ordering::Equal) => println!("{x} = {y}"),
            Some(Ordering::Greater) => println!("{x} > {y}"),
        }
    }
}

fn demo_float_partial_cmp_integer_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_integer_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        match x.partial_cmp(&y) {
            None => println!("{cx:#x} and {y:#x} are incomparable"),
            Some(Ordering::Less) => println!("{cx:#x} < {y:#x}"),
            Some(Ordering::Equal) => println!("{cx:#x} = {y:#x}"),
            Some(Ordering::Greater) => println!("{cx:#x} > {y:#x}"),
        }
    }
}

fn demo_integer_partial_cmp_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_integer_pair_gen().get(gm, config).take(limit) {
        match x.partial_cmp(&y) {
            None => println!("{x} and {y} are incomparable"),
            Some(Ordering::Less) => println!("{x} < {y}"),
            Some(Ordering::Equal) => println!("{x} = {y}"),
            Some(Ordering::Greater) => println!("{x} > {y}"),
        }
    }
}

fn demo_integer_partial_cmp_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_integer_pair_gen().get(gm, config).take(limit) {
        let cy = ComparableFloatRef(&y);
        match x.partial_cmp(&y) {
            None => println!("{x:x} and {cy:#x} are incomparable"),
            Some(Ordering::Less) => println!("{x:#x} < {cy:#x}"),
            Some(Ordering::Equal) => println!("{x:#x} = {cy:#x}"),
            Some(Ordering::Greater) => println!("{x:#x} > {cy:#x}"),
        }
    }
}

#[allow(unused_must_use)]
fn benchmark_float_partial_cmp_integer_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.partial_cmp(&Integer)",
        BenchmarkType::LibraryComparison,
        float_integer_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_integer_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.partial_cmp(&y))),
            ("rug", &mut |((x, y), _)| no_out!(x.partial_cmp(&y))),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_partial_cmp_float_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.partial_cmp(&Float)",
        BenchmarkType::LibraryComparison,
        float_integer_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_integer_max_complexity_bucketer("y", "x"),
        &mut [
            ("Malachite", &mut |(_, (y, x))| no_out!(x == y)),
            ("rug", &mut |((y, x), _)| no_out!(x == y)),
        ],
    );
}