#![feature(test)]
extern crate test;

fn remove_instruction_length(source: &[u8], next_lcs_item: Option<&u8>) -> usize {
    if let Some(&item) = next_lcs_item {
        source.iter().position(|&x| x == item).unwrap_or(0)
    } else {
        0
    }
}

fn add_instruction_length(target: &[u8], next_lcs_item: Option<&u8>) -> usize {
    remove_instruction_length(&target, next_lcs_item)
}

#[cfg(test)]
mod encoder_benchmarks {
    use super::*;
    use fast_deltas::lcs::Lcs;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_remove_instruction_length(b: &mut Bencher) {
        let source = [0; u16::MAX as usize];
        let target = [];
        let lcs = Lcs::new(&source, &target).subsequence();
        b.iter(|| {
            remove_instruction_length(black_box(&source), black_box(lcs.first()));
        })
    }

    #[bench]
    fn bench_add_instruction_length(b: &mut Bencher) {
        let source = [];
        let target = [0; u16::MAX as usize];
        let lcs = Lcs::new(&source, &target).subsequence();
        b.iter(|| {
            add_instruction_length(black_box(&target), black_box(lcs.first()));
        })
    }    

    
}
