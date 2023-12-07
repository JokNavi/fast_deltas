use crate::{
    lcs::Lcs, AVERAGE_INSTRUCTION_AMOUNT, WANTED_CHUNK_SIZE, INSTRUCTION_BYTE, ZERO_ITEM_COUNT_PERCENT,
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

fn add_instruction_length(lcs: &[u8], target: &[u8]) -> usize {
    todo!();
}

fn remove_instruction_length(lcs: &[u8], source: &[u8]) -> usize {
    todo!();
}


#[cfg(test)]
mod encoder_tests {
    use super::*;

}
