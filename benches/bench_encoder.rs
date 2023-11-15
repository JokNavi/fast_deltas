#![feature(test)]

extern crate test;


#[cfg(test)]
mod benchmarks {
    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| {
            black_box(1)
        });
    }    
}
