#![feature(test)]

use fast_deltas::{ChunkLength, REMOVE_INSTRUCTION_SIGN, ZERO_ITEM_COUNT_PERCENT};

extern crate test;

fn buffer_zero_count(buffer: &mut [u8]) -> usize {
    buffer.iter().filter(|item| item == &&0).count()
}

fn calc_percent(value: usize, buff_length: usize) -> usize {
    ((value as f32 * 100.0) / buff_length as f32).round() as usize
}

///Returns the amount of source bytes used
fn write_remove_instruction(source: &[u8], lcs: &[u8], instruction_buffer: &mut Vec<u8>) -> usize {
    instruction_buffer.push(REMOVE_INSTRUCTION_SIGN);
    let mut source_index: usize = 0;
    let source_len = source.len();
    while source_index < ChunkLength::MAX as usize
        && source_index < source_len
        && (lcs.is_empty() || lcs[0] != source[source_index])
    {
        source_index += 1;
    }
    instruction_buffer.append(&mut ChunkLength::to_be_bytes(source_index as ChunkLength).to_vec());
    source_index
}

/// Returns the amount of target bytes used
fn write_add_instruction(target: &[u8], lcs: &[u8], instruction_buffer: &mut Vec<u8>) -> usize {
    let mut target_index: usize = 0;
    let target_len = target.len();
    while target_index < ChunkLength::MAX as usize
        && target_index < target_len
        && (lcs.is_empty() || lcs[0] != target[target_index])
    {
        target_index += 1;
    }
    instruction_buffer.extend_from_slice(&target[0..target_index]);
    target_index
}

/// Returns the amount of lcs bytes written
fn write_copy_instruction(
    source: &[u8],
    target: &[u8],
    lcs: &[u8],
    instruction_buffer: &mut Vec<u8>,
    zero_count: usize,
) -> (usize, usize) {
    let mut zero_count: usize = zero_count;

    let source_len = source.len();
    let target_len = target.len();
    let lcs_len = lcs.len();
    let mut index: usize = 0;


    while ((index < lcs_len && (lcs[index] == source[index] && lcs[index] == target[index]))
        || (((zero_count * 100) / index) <= ZERO_ITEM_COUNT_PERCENT))
        && (index < source_len && index < target_len)
    {
        if target[index] == source[index] {
            zero_count += 1;
        }
        index += 1;
    }

    instruction_buffer.extend(
        (source[0..index])
            .iter()
            .zip(target)
            .map(|(source_item, target_item)| target_item.wrapping_sub(*source_item)),
    );
    (index, zero_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fast_deltas::{lcs::Lcs, ChunkLength};
    use test::{black_box, Bencher};

    #[bench]
    fn bench_write_add_instruction(b: &mut Bencher) {
        let source = vec![0; 255];
        let target = vec![];
        let lcs = Lcs::new(&source, &target).subsequence();
        b.iter(|| {
            let mut instruction_buffer = vec![];
            black_box(write_add_instruction(
                &source,
                &lcs,
                &mut instruction_buffer,
            ));
        });
    }

    #[bench]
    fn bench_write_remove_instruction(b: &mut Bencher) {
        let source = vec![];
        let target = vec![0; 255];
        let lcs = Lcs::new(&source, &target).subsequence();
        b.iter(|| {
            let mut instruction_buffer = vec![];
            black_box(write_remove_instruction(
                &target,
                &lcs,
                &mut instruction_buffer,
            ));
        });
    }

    #[bench]
    fn bench_write_copy_instruction(b: &mut Bencher) {
        let source = vec![0; 255];
        let target = vec![0; 255];
        let lcs = Lcs::new(&source, &target).subsequence();
        b.iter(|| {
            let mut instruction_buffer = vec![];
            black_box(write_copy_instruction(
                &source,
                &target,
                &lcs,
                &mut instruction_buffer,
                0,
            ));
        });
    }
}
