#![feature(test)]

extern crate test;

#[cfg(test)]
mod lcs_benchmarks {
    use fast_deltas::{encoder::CHUNK_SIZE, lcs::Lcs};
    use test::{black_box, Bencher};


    #[bench]
    fn bench_lcs_new(b: &mut Bencher) {
        let source = [0u8; CHUNK_SIZE as usize];
        let target = [1u8; CHUNK_SIZE as usize];
        b.iter(|| {
            black_box(Lcs::new(&source, &target));
        });
    }  

    #[bench]
    fn bench_lcs(b: &mut Bencher) {
        let source = [0u8; CHUNK_SIZE as usize];
        let target = [1u8; CHUNK_SIZE as usize];
        let lcs = Lcs::new(&source, &target);
        b.iter(|| {
            black_box(lcs.subsequence());
        });
    }  
}