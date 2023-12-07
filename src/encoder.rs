use crate::{
    lcs::Lcs, AVERAGE_INSTRUCTION_AMOUNT, INSTRUCTION_BYTE, WANTED_CHUNK_SIZE,
    ZERO_ITEM_COUNT_PERCENT,
};
use std::io::{self, BufReader, BufWriter, Read, Write};

///The actual chunk size used. A few bytes are subtracted to make place for the instruction identifiers.
pub const CHUNK_SIZE: u8 = WANTED_CHUNK_SIZE - (AVERAGE_INSTRUCTION_AMOUNT * 2);

/// The capcity allocated for the instruction buffer.
pub const BUFFER_SIZE: usize = WANTED_CHUNK_SIZE as usize;

pub fn delta_encode<R: Read, W: Write>(source: R, target: R, patch: W) -> io::Result<()> {
    todo!();
}

fn fill_instruction_buffer(lcs: &[u8], source: &[u8], target: &[u8]) {
    todo!();
}

fn copy_instruction_length(lcs: &[u8], source: &[u8], target: &[u8]) -> usize {
    todo!();
}

fn add_instruction_length(target: &[u8], next_lcs_item: Option<&u8>) -> usize {
    remove_instruction_length(&target, next_lcs_item)
}

fn remove_instruction_length(source: &[u8], next_lcs_item: Option<&u8>) -> usize {
    if let Some(&item) = next_lcs_item {
        source.iter().position(|&x| x == item).unwrap_or(0)
    } else {
        0
    }
}

#[cfg(test)]
mod encoder_tests {
    use super::*;

    #[test]
    fn test_remove_instruction_length() {
        let source = [0, 0, 0, 1, 1, 1];
        let target = [1, 1, 1];
        let lcs = Lcs::new(&source, &target).subsequence();
        assert_eq!(remove_instruction_length(&source, lcs.first()), 3);
    }

    #[test]
    fn test_add_instruction_length() {
        let source = [1, 1, 1];
        let target = [0, 0, 0, 1, 1, 1];
        let lcs = Lcs::new(&source, &target).subsequence();
        assert_eq!(remove_instruction_length(&target, lcs.first()), 3);
    }
}
