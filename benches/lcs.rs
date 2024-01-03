use divan::{black_box, Bencher};
use fast_deltas::lcs::Lcs;

#[divan::bench]
fn bench_lcs(bencher: Bencher) {
    let source = vec![0; 255];
    let target = vec![0; 255];
    bencher.bench_local(move || {
        black_box(Lcs::new(black_box(&source), black_box(&target)).subsequence());
    });
}
