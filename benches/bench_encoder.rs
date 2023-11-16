#![feature(test)]

use fast_deltas::ZERO_ITEM_COUNT_PERCENT;

extern crate test;

///Returns the amount of bytes the next Remove instruction will take
fn remove_instruction_length(source: &[u8], lcs: &[u8]) -> usize {
    let mut source_index: usize = 0;
    let source_len = source.len();
    while source_index < source_len && (lcs.is_empty() || lcs[0] != source[source_index]) {
        source_index += 1;
    }
    source_index
}

/// Returns the amount of bytes the next Copy instruction will take
fn copy_instruction_length(source: &[u8], target: &[u8], lcs: &[u8]) -> usize {
    let source_len = source.len();
    let target_len = target.len();
    let lcs_len = lcs.len();
    let mut index: usize = 0;
    let mut zero_count = 0;

    while ((index < lcs_len && (lcs[index] == source[index] && lcs[index] == target[index]))
        || (((zero_count * 100) / index) <= ZERO_ITEM_COUNT_PERCENT))
        && (index < source_len && index < target_len)
    {
        if target[index] == source[index] {
            zero_count += 1;
        }
        index += 1;
    }
    index
}


#[cfg(test)]
mod benchmarks {
    use super::*;
    use fast_deltas::{encoder::CHUNK_SIZE, lcs::Lcs};
    use test::{black_box, Bencher};

    #[bench]
    fn bench_copy_instruction_length(b: &mut Bencher) {
        let source = [0u8; CHUNK_SIZE as usize];
        let target = [0u8; CHUNK_SIZE as usize];
        let lcs = Lcs::new(&source, &target).subsequence();
        b.iter(|| {
            black_box(copy_instruction_length(black_box(&source), black_box(&target), black_box(&lcs)));
        });
    }    

    #[bench]
    fn bench_remove_instruction_length(b: &mut Bencher) {
        let source = [0u8; CHUNK_SIZE as usize];
        let target = [0u8; CHUNK_SIZE as usize];
        let lcs = Lcs::new(&source, &target).subsequence();
        b.iter(|| {
            black_box(remove_instruction_length(black_box(&source), black_box(&lcs)));
        });
    }    
}
