pub mod encoder;
pub mod lcs;

/// ### Special: Check next byte.
/// If the next byte IS a 0 it is a copy instruction.
/// If the next byte IS NOT a 0 it is a remove instruction.
/// If the that should have been INSTRUCTION_BYTE is not equal to 0 it is an add instruction.
const INSTRUCTION_BYTE: u8 = 0;

/// The size of the source and target chunk stored in memory, in bytes.
/// Increasing this will increase the memory required for each chunk exponentially. (Due to the current Lcs implementation.)
pub const WANTED_CHUNK_SIZE: u8 = 255;

/// The average amount of induvidual instructions per chunk 
const AVERAGE_INSTRUCTION_AMOUNT: u8 = 5;

///The maximum percent of non 0 values in a copy instruction.
pub const ZERO_ITEM_COUNT_PERCENT: usize = 100;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcs::Lcs;

    #[test]
    fn testing() {
        let source: Vec<u8> = vec![1, 1, 2, 2, 2, 1, 1];
        let target: Vec<u8> = vec![1, 1, 3, 3, 3, 1, 1];
        let lcs = Lcs::new(&source, &target).subsequence();
        dbg!();
    }
}
