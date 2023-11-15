pub mod encoder;
pub mod lcs;

/// The type of the value used to represent the length of a chunk.
/// ChunkLength::MAX decides the maximum possible size of CHUNK_SIZE.
pub type ChunkLength = u8;

/// The size of the source and target chunk stored in memory, in bytes.
/// Increasing this will increase the memory required exponentially. (Due to the current Lcs implementation.) 
pub const CHUNK_SIZE: ChunkLength = 255;

pub const BUFFER_SIZE: usize = CHUNK_SIZE as usize + 1;

///The maximum percent of non 0 values in a copy instruction.
pub const ZERO_ITEM_COUNT_PERCENT: usize = 100;

/// ### Special: Check next byte.
/// If the next byte IS a 0 it is a copy instruction.
/// If the next byte IS NOT a 0 it is a remove instruction.
/// If the that should have been INSTRUCTION_BYTE is not equal to 0 it is an add instruction.
pub const INSTRUCTION_BYTE: u8 = 0;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcs::Lcs;

    #[test]
    fn testing() {
        let source: Vec<u8> = vec![1, 1, 2, 2, 2, 1, 1];
        let target: Vec<u8> = vec![1, 1, 3, 3, 3, 1, 1];
        let lcs = Lcs::new(&source, &target).subsequence();
    }
}
