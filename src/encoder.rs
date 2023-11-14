use std::io::{self, BufReader, BufWriter, Read, Write};

use crate::{lcs::Lcs, ChunkLength, REMOVE_INSTRUCTION_SIGN, ZERO_ITEM_COUNT_PERCENT};

pub fn delta_encode<R: Read, W: Write>(source: R, target: R, patch: W) -> io::Result<()> {
    let mut source_reader = BufReader::new(source);
    let mut target_reader = BufReader::new(target);
    let mut patch_writer = BufWriter::new(patch);

    let mut source_buffer = [0u8; ChunkLength::MAX as usize];
    let mut target_buffer = [0u8; ChunkLength::MAX as usize];

    let mut source_bytes_read = source_reader.read(&mut source_buffer)?;
    let mut target_bytes_read = target_reader.read(&mut target_buffer)?;

    while source_bytes_read > 0 || target_bytes_read > 0 {
        let instruction_bytes = write_instructions_chunk(
            &source_buffer[..source_bytes_read],
            &target_buffer[..target_bytes_read],
        );
        patch_writer.write_all(&instruction_bytes)?;
        source_bytes_read = source_reader.read(&mut source_buffer)?;
        target_bytes_read = target_reader.read(&mut target_buffer)?;
    }

    Ok(())
}

fn write_instructions_chunk(source: &[u8], target: &[u8]) -> Vec<u8> {
    let lcs = Lcs::new(source, target).subsequence();
    let mut source_index: usize = 0;
    let mut target_index: usize = 0;
    let mut lcs_index: usize = 0;
    let mut buffer_zero_count = 0;

    let mut instruction_buffer: Vec<u8> = Vec::with_capacity(ChunkLength::MAX as usize + 1);
    while lcs_index < lcs.len() {
        debug_assert!(lcs_index <= source_index && lcs_index <= target_index);
        if source_index < source.len() && lcs[lcs_index] != source[source_index] {
            //Remove
            source_index +=
                write_remove_instruction(&source[source_index..], &lcs, &mut instruction_buffer);
        } else if target_index < target.len() && lcs[lcs_index] != target[target_index] {
            //Add
            target_index +=
                write_add_instruction(&target[target_index..], &lcs, &mut instruction_buffer);
        } else {
            //Copy
            let (bytes_written, local_buffer_zero_count) = write_copy_instruction(
                &source[source_index..],
                &target[target_index..],
                &lcs[lcs_index..],
                &mut instruction_buffer,
                buffer_zero_count,
            );
            source_index += bytes_written;
            target_index += bytes_written;
            lcs_index += bytes_written;
            buffer_zero_count += local_buffer_zero_count;
        }
    }
    while source_index < source.len() {
        //Remove
        source_index +=
            write_remove_instruction(&source[source_index..], &lcs, &mut instruction_buffer);
    }
    while target_index < target.len() {
        //Add
        target_index +=
            write_add_instruction(&target[target_index..], &lcs, &mut instruction_buffer);
    }
    instruction_buffer
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
mod encoder_tests {
    use super::*;

    #[test]
    fn write_remove_instruction_test() {
        let source = vec![0; 255];
        let target = vec![];
        let lcs = Lcs::new(&source, &target).subsequence();
        let mut instruction_buffer = vec![];
        assert_eq!(
            write_remove_instruction(&source, &lcs, &mut instruction_buffer),
            255
        );
        assert_eq!(
            instruction_buffer[1..],
            ChunkLength::to_be_bytes(255).to_vec()
        );
        assert_eq!(instruction_buffer[0], 45);

        let source = vec![0, 0, 0, 1, 1, 1];
        let target = vec![1, 1, 1];
        let lcs = Lcs::new(&source, &target).subsequence();
        let mut instruction_buffer = vec![];
        assert_eq!(
            write_remove_instruction(&source, &lcs, &mut instruction_buffer),
            3
        );
        assert_eq!(
            instruction_buffer[1..],
            ChunkLength::to_be_bytes(3).to_vec()
        );
    }

    #[test]
    fn write_add_instruction_test() {
        let source = vec![];
        let target = vec![0; 255];
        let lcs = Lcs::new(&source, &target).subsequence();
        let mut instruction_buffer = vec![];
        assert_eq!(
            write_add_instruction(&target, &lcs, &mut instruction_buffer),
            255
        );
        assert_eq!(instruction_buffer, vec![0; 255]);

        let source = vec![0, 0, 0, 1, 1, 1];
        let target = vec![2, 2, 2];
        let lcs = Lcs::new(&source, &target).subsequence();
        let mut instruction_buffer = vec![];
        assert_eq!(
            write_add_instruction(&target, &lcs, &mut instruction_buffer),
            3
        );
        assert_eq!(instruction_buffer, target);
    }

    #[test]
    fn write_copy_instruction_test() {
        let source = vec![1, 1, 1, 0, 0, 0];
        let target = vec![1, 1, 1, 2, 2, 2];
        let lcs = Lcs::new(&source, &target).subsequence();
        let mut instruction_buffer = vec![];
        assert_eq!(
            write_copy_instruction(&source, &target, &lcs, &mut instruction_buffer, 0),
            (6, 3)
        );
        assert_eq!(instruction_buffer, vec![0, 0, 0, 2, 2, 2]);
    }
}
