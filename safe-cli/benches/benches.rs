// #![feature(test)]


// extern crate safe_utilities;
// extern crate test;
#[macro_use]
extern crate duct;

use safe_utilities::{
    get_bin_location
    // create_preload_and_get_keys, get_bin_location, get_random_nrs_string, parse_cat_wallet_output,
    // parse_files_put_or_sync_output, CLI,
};
use assert_cmd::prelude::*;
const TEST_FILE: &str = "../testdata/test.md";

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn calling_safe_cat() -> () {
    // trace!("bionnnnnnnnn{}", get_bin_location());
    let content = cmd!(get_bin_location(), "files", "put", TEST_FILE, "--json")
        .read()
        .unwrap();

    // safe_utilities::shared_code();
}

// fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n-1) + fibonacci(n-2),
//     }
// }

// #[cfg(test)]
// mod tests {
//         use super::*;
        // use test::Bencher;
        // use tests::cli_cat::calling_safe_cat;
        // use tests::calling_safe_cat;
        
        // #[bench]
        // fn c: &mut Criterion(b: &mut Bencher) {
        //     b.iter(|| calling_safe_cat());
        // }

        fn bench_cli_put(c: &mut Criterion) {
            c.bench_function("cliii_put", |b| b.iter(|| cmd!(get_bin_location(), "files", "put", TEST_FILE, "--json")
            .read()
            .unwrap()
        ));
        }
    
        criterion_group!(benches, bench_cli_put);
        criterion_main!(benches);
// }
