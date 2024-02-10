pub mod lcs;
use std::fs::OpenOptions;

use divan::{black_box, Bencher};
use fast_deltas::encoder::delta_encode;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_delta_encode(bencher: Bencher) {
    let source = OpenOptions::new()
        .read(true)
        .open("test_files/source.txt")
        .unwrap();
    let target = OpenOptions::new()
        .read(true)
        .open("test_files/target.txt")
        .unwrap();
    let patch = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("test_files/patch.dpatch")
        .unwrap();
    bencher.bench_local(move || {
        black_box(delta_encode(&source, &target, &patch)).unwrap();
    });
}
