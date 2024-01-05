use divan::{black_box, Bencher};
use fast_deltas::{
    encoder::{add_instruction_length, copy_instruction_length, remove_instruction_length},
    lcs::Lcs,
};

#[divan::bench]
fn bench_copy_instruction_length(bencher: Bencher) {
    let source = vec![0; 255];
    let target = vec![0; 255];
    let lcs = Lcs::new(&source, &target).subsequence();
    bencher.bench_local(move || {
        black_box(copy_instruction_length(
            black_box(&source),
            black_box(&target),
            black_box(&lcs),
        ));
    });
}

#[divan::bench]
fn bench_add_instruction_length(bencher: Bencher) {
    let source = vec![0; 255];
    let target = vec![0; 255];
    let lcs = Lcs::new(&source, &target).subsequence();
    bencher.bench_local(move || {
        black_box(add_instruction_length(
            black_box(&target),
            black_box(lcs.first()),
        ));
    });
}

#[divan::bench]
fn bench_remove_instruction_length(bencher: Bencher) {
    let source = vec![0; 255];
    let target = vec![0; 255];
    let lcs = Lcs::new(&source, &target).subsequence();
    bencher.bench_local(move || {
        black_box(remove_instruction_length(
            black_box(&source),
            black_box(lcs.first()),
        ));
    });
}
