use crate::{
    lcs::Lcs, AVERAGE_INSTRUCTION_AMOUNT, WANTED_CHUNK_SIZE, INSTRUCTION_BYTE, ZERO_ITEM_COUNT_PERCENT,
};
use std::io::{self, BufReader, BufWriter, Read, Write};

///The actual chunk size used. A few bytes are subtracted to make place for the instruction identifiers.
const CHUNK_SIZE: u8 = WANTED_CHUNK_SIZE - (AVERAGE_INSTRUCTION_AMOUNT * 2);

/// The capcity allocated for the instruction buffer.
const BUFFER_SIZE: usize = WANTED_CHUNK_SIZE as usize;

pub fn delta_encode<R: Read, W: Write>(source: R, target: R, patch: W) -> io::Result<()> {
    let mut source_reader = BufReader::new(source);
    let mut target_reader = BufReader::new(target);
    let mut patch_writer = BufWriter::new(patch);

    let mut source_buffer = [0u8; CHUNK_SIZE as usize];
    let mut target_buffer = [0u8; CHUNK_SIZE as usize];

    let mut source_bytes_read = source_reader.read(&mut source_buffer)?;
    let mut target_bytes_read = target_reader.read(&mut target_buffer)?;

    while source_bytes_read > 0 || target_bytes_read > 0 {
        let instruction_buffer = fill_instructions_buffer(
            &source_buffer[..source_bytes_read],
            &target_buffer[..target_bytes_read],
        );
        patch_writer.write_all(&instruction_buffer)?;
        source_bytes_read = source_reader.read(&mut source_buffer)?;
        target_bytes_read = target_reader.read(&mut target_buffer)?;
    }
    Ok(())
}

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
mod encoder_tests {
    use super::*;

    #[test]
    fn remove_instruction_length_test() {
        let source = vec![0; 255];
        let target = vec![];
        let lcs = Lcs::new(&source, &target).subsequence();
        assert_eq!(remove_instruction_length(&source, &lcs), 255);

        let source = vec![0, 0, 0, 1, 1, 1];
        let target = vec![1, 1, 1];
        let lcs = Lcs::new(&source, &target).subsequence();
        assert_eq!(remove_instruction_length(&source, &lcs), 3);
    }

    #[test]
    fn add_instruction_length_test() {
        let source = vec![];
        let target = vec![0; 255];
        let lcs = Lcs::new(&source, &target).subsequence();
        assert_eq!(add_instruction_length(&target, &lcs), 255);

        let source = vec![0, 0, 0, 1, 1, 1];
        let target = vec![2, 2, 2];
        let lcs = Lcs::new(&source, &target).subsequence();
        assert_eq!(add_instruction_length(&target, &lcs), 3);
    }

    #[test]
    fn copy_instruction_length_test() {
        let source = vec![1, 1, 1, 0, 0, 0];
        let target = vec![1, 1, 1, 2, 2, 2];
        let lcs = Lcs::new(&source, &target).subsequence();
        assert_eq!(copy_instruction_length(&source, &target, &lcs), 6);
    }

    #[test]
    fn fill_instructions_buffer_test() {
        let source = vec![0, 0, 0, 1, 1, 1, 0, 0, 0];
        let target = vec![1, 1, 1, 2, 2, 2, 3, 3, 3];
        assert_eq!(
            fill_instructions_buffer(&source, &target),
            [0, 3, 0, 0, 0, 0, 2, 2, 2, 3, 3, 3, 3]
        );

        let source = vec![0, 0, 0, 1, 1, 1];
        let target = vec![1, 1, 1, 2, 2, 2, 3, 3, 3];
        assert_eq!(
            fill_instructions_buffer(&source, &target),
            [0, 3, 0, 0, 0, 0, 6, 2, 2, 2, 3, 3, 3]
        );

        let source = vec![0, 0, 0, 1, 1, 1, 0, 0, 0];
        let target = vec![1, 1, 1];
        assert_eq!(
            fill_instructions_buffer(&source, &target),
            [0, 3, 0, 0, 0, 0, 0, 3]
        );
        
        let source = vec![];
        let target = vec![0; 255];
        let mut expected_output = vec![255];
        expected_output.extend(vec![0; 255]);
        assert_eq!(
            fill_instructions_buffer(&source, &target),
            expected_output
        );

        let source = vec![0; 255];
        let target = vec![];
        assert_eq!(
            fill_instructions_buffer(&source, &target),
            vec![0, 255]
        );
    }
}
