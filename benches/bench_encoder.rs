#![feature(test)]

use fast_deltas::{ZERO_ITEM_COUNT_PERCENT, encoder::BUFFER_SIZE, lcs::Lcs, INSTRUCTION_BYTE};

extern crate test;

fn fill_instructions_buffer(source: &[u8], target: &[u8]) -> Vec<u8> {
    let lcs = Lcs::new(source, target).subsequence();
    let mut source_index: usize = 0;
    let mut target_index: usize = 0;
    let mut lcs_index: usize = 0;

    let mut instruction_buffer = Vec::with_capacity(BUFFER_SIZE);
    while lcs_index < lcs.len() {
        debug_assert!(lcs_index <= source_index && lcs_index <= target_index);
        if source_index < source.len() && lcs[lcs_index] != source[source_index] {
            //Remove
            let instruction_length = remove_instruction_length(&source[source_index..], &lcs);
            instruction_buffer.push(INSTRUCTION_BYTE);
            instruction_buffer.push(instruction_length as u8);
            source_index += instruction_length;
        } else if target_index < target.len() && lcs[lcs_index] != target[target_index] {
            //Add
            let instruction_length = add_instruction_length(&target[target_index..], &lcs);
            instruction_buffer.push(instruction_length as u8);
            instruction_buffer
                .extend_from_slice(&target[target_index..target_index + instruction_length]);
            target_index += instruction_length;
        } else {
            //Copy
            let instruction_length =
                copy_instruction_length(&source[source_index..], &target[target_index..], &lcs);
            instruction_buffer.push(INSTRUCTION_BYTE);
            instruction_buffer.extend(
                (source[source_index..source_index + instruction_length])
                    .iter()
                    .zip(&target[target_index..target_index + instruction_length])
                    .map(|(source_item, target_item)| target_item.wrapping_sub(*source_item)),
            );
            source_index += instruction_length;
            target_index += instruction_length;
            lcs_index += instruction_length;
        }
    }
    while source_index < source.len() {
        //Remove
        let instruction_length = remove_instruction_length(&source[source_index..], &lcs);
        instruction_buffer.push(INSTRUCTION_BYTE);
        instruction_buffer.push(instruction_length as u8);
        source_index += instruction_length;
    }
    while target_index < target.len() {
        //Add
        let instruction_length = add_instruction_length(&target[target_index..], &lcs);
        instruction_buffer.push(instruction_length as u8);
        instruction_buffer
            .extend_from_slice(&target[target_index..target_index + instruction_length]);
        target_index += instruction_length;
    }
    instruction_buffer
}

///Returns the amount of bytes the next Remove instruction will take
fn remove_instruction_length(source: &[u8], lcs: &[u8]) -> usize {
    let mut source_index: usize = 0;
    let source_len = source.len();
    while source_index < source_len && (lcs.is_empty() || lcs[0] != source[source_index]) {
        source_index += 1;
    }
    source_index
}

/// Returns the amount of bytes the next Add instruction will take
fn add_instruction_length(target: &[u8], lcs: &[u8]) -> usize {
    remove_instruction_length(target, lcs)
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
mod encoder_benchmarks {
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

    #[bench]
    fn bench_flll_instructions_buffer(b: &mut Bencher) {
        let source = [0u8; CHUNK_SIZE as usize];
        let target = [1u8; CHUNK_SIZE as usize];
        b.iter(|| {
            black_box(fill_instructions_buffer(black_box(&source), black_box(&target)));
        });
    }    
}
