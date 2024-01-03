use divan::{black_box, Bencher};
use fast_deltas::{iterators::copy_instruction_iterator::CopyInstructionIterator, lcs::Lcs};

#[divan::bench]
fn bench_copy_instruction_iterator(bencher: Bencher) {
    let source = vec![0; 255];
    let target = vec![0; 255];
    let lcs = Lcs::new(&source, &target).subsequence();
    bencher.bench_local(move || {
        let mut copy_instruction_iterator = black_box(CopyInstructionIterator::new(
            black_box(&lcs),
            black_box(&source),
            black_box(&target),
        ));
        black_box(for _ in 0..255 {
            black_box(copy_instruction_iterator.next());
        })
    });
}
