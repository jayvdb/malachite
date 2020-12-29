#[macro_use]
extern crate malachite_base_test_util;
extern crate malachite_nz;

use malachite_base_test_util::runner::cmd::read_command_line_arguments;
use malachite_base_test_util::runner::Runner;

use crate::generate::digits_data::generate_string_data;
use demo_and_bench::register;

// Examples:
//
// cargo run -- -l 100000 -m special_random -d demo_natural_from_unsigned_u128 -c
// "mean_run_length_n 4 mean_run_length_d 1"
//
// or
//
// cargo run -- -g digits_data
fn main() {
    let args = read_command_line_arguments("malachite-nz test utils");
    let mut runner = Runner::new();
    register(&mut runner);
    if let Some(demo_key) = args.demo_key {
        runner.run_demo(&demo_key, args.generation_mode, args.config, args.limit);
    } else if let Some(bench_key) = args.bench_key {
        runner.run_bench(
            &bench_key,
            args.generation_mode,
            args.config,
            args.limit,
            &args.out,
        );
    } else {
        let codegen_key = args.codegen_key.unwrap();
        if codegen_key == "digits_data" {
            generate_string_data();
        } else {
            panic!("Invalid codegen key: {}", codegen_key);
        }
    }
}

mod demo_and_bench;
mod generate;
