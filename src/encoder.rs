use std::{cmp::max, io::{self, BufReader, BufWriter, Read, Write}};

use crate::{lcs::Lcs, INSTRUCTION_BYTE, NON_INSTRUCTION_BYTE_COUNT_PERCENT};
const CHUNK_SIZE: usize = u8::MAX as usize;

pub fn delta_encode<R: Read, W: Write>(source: R, target: R, patch: W) -> io::Result<()> {
    let mut source_reader = BufReader::with_capacity(CHUNK_SIZE, source);
    let mut target_reader = BufReader::with_capacity(CHUNK_SIZE, target);
    let mut patch_writer = BufWriter::with_capacity(CHUNK_SIZE + 15, patch);

    let mut source_buffer = Vec::with_capacity(CHUNK_SIZE);
    let mut target_buffer = Vec::with_capacity(CHUNK_SIZE);

    loop {
        let source_len = source_reader
            .by_ref()
            .take(CHUNK_SIZE as u64)
            .read_to_end(&mut source_buffer)?;
        let target_len = target_reader
            .by_ref()
            .take(CHUNK_SIZE as u64)
            .read_to_end(&mut target_buffer)?;

        if source_len == 0 && target_len == 0 {
            break;
        }

        let instructions = create_instructions(&source_buffer, &target_buffer);
        patch_writer.write_all(&instructions)?;
        source_buffer.clear();
        target_buffer.clear();
    }

    patch_writer.flush()?;
    Ok(())
}

pub fn create_instructions(source: &[u8], target: &[u8]) -> Vec<u8> {
    debug_assert!(source.len() <= CHUNK_SIZE as usize);
    debug_assert!(target.len() <= CHUNK_SIZE as usize);
    let lcs = Lcs::new(source, target).subsequence();
    let mut bytes: Vec<u8> = Vec::with_capacity(max(source.len(), target.len()) + lcs.len());
    let mut source_index: usize = 0;
    let mut target_index: usize = 0;
    let mut lcs_index: usize = 0;
    while !lcs.is_empty()
        && lcs_index < lcs.len()
        && source_index < source.len()
        && target_index < target.len()
    {
        if source[source_index] == target[target_index] {
            //line 48
            //copy
            bytes.push(INSTRUCTION_BYTE);
            let (lcs_count, items_count) = copy_instruction_length(
                &source[source_index..],
                &target[target_index..],
                &lcs[lcs_index..],
            );
            bytes.extend(
                source[source_index..]
                    .iter()
                    .zip(target[target_index..].iter())
                    .take(items_count)
                    .map(|(source_num, target_num)| target_num.wrapping_sub(*source_num)),
            );
            lcs_index += lcs_count;
            source_index += items_count;
            target_index += items_count;
        } else if source[source_index] != lcs[lcs_index] {
            //remove
            bytes.push(INSTRUCTION_BYTE);
            let source_count =
                remove_instruction_length(&source[source_index..], Some(lcs[lcs_index]));
            bytes.push(source_count.try_into().unwrap());
            source_index += source_count;
        } else if target[target_index] != lcs[lcs_index] {
            //add
            let target_count =
                add_instruction_length(&target[target_index..], Some(lcs[lcs_index]));
            bytes.push(target_count.try_into().unwrap());
            bytes.extend(target[target_index..].iter().take(target_count));
            target_index += target_count;
        }
    }
    if !source[source_index..].is_empty() {
        //remove
        bytes.push(INSTRUCTION_BYTE);
        bytes.push(source[source_index..].len().try_into().unwrap());
    }
    if !target[target_index..].is_empty() {
        //add
        bytes.push(target[target_index..].len().try_into().unwrap());
        bytes.extend(target[target_index..].iter());
    }
    bytes
}

fn add_instruction_length(target: &[u8], next_lcs_item: Option<u8>) -> usize {
    remove_instruction_length(target, next_lcs_item)
}

fn remove_instruction_length(source: &[u8], next_lcs_item: Option<u8>) -> usize {
    if let Some(item) = next_lcs_item {
        source
            .iter()
            .position(|&x| x == item)
            .unwrap_or(source.len())
    } else {
        source.len()
    }
}

fn copy_instruction_length(source: &[u8], target: &[u8], lcs: &[u8]) -> (usize, usize) {
    let mut non_instruction_byte_values_count: usize = 0;
    let (mut item_index, mut lcs_index) = (0, 0);
    while item_index < source.len() && item_index < target.len() && lcs_index < lcs.len() {
        let diff_byte = target[item_index].wrapping_sub(source[item_index]);
        if !(diff_byte == INSTRUCTION_BYTE || diff_byte == u8::MAX - INSTRUCTION_BYTE) {
            non_instruction_byte_values_count += 1;
        }
        if source[item_index] == lcs[lcs_index] || target[item_index] == lcs[lcs_index] {
            lcs_index += 1;
        }
        if (non_instruction_byte_values_count as f32 / (item_index + 1) as f32) * 100.0
            > NON_INSTRUCTION_BYTE_COUNT_PERCENT
        {
            break;
        }
        item_index += 1;
    }
    (item_index, lcs_index)
}

#[cfg(test)]
mod encoder_tests {
    use super::*;
    use crate::lcs::Lcs;

    #[test]
    fn test_remove_instruction_length() {
        let source = [1, 1, 1, 0, 0, 0];
        let target = [0, 0, 0];
        let lcs = Lcs::new(&source, &target).subsequence();
        assert_eq!(remove_instruction_length(&source, lcs.first().copied()), 3);
    }

    #[test]
    fn test_add_instruction_length() {
        let source = [0, 0, 0];
        let target = [1, 1, 1, 0, 0, 0];
        let lcs = Lcs::new(&source, &target).subsequence();
        assert_eq!(add_instruction_length(&target, lcs.first().copied()), 3);
    }

    #[test]
    fn test_copy_instruction_length() {
        let source = [0, 0, 3, 1, 2, 4];
        let target = [0, 0, 1, 2, 3, 4];
        // let source = [5, 5, 0, 0, 3, 1, 2, 4];
        // let target = [6, 0, 0, 1, 2, 3, 4];
        // Lcs = [0, 0, 1, 2, 4];
        // Patch = [0, 2, 1, 6, 0, 0, 0, 2, 254, 0, 1, 1, 254, 0];
        let lcs = Lcs::new(&source, &target).subsequence();
        assert_eq!(copy_instruction_length(&source, &target, &lcs), (4, 4));
    }
}
