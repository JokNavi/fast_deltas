pub mod encoder;
pub mod lcs;

/// ### Special: Check next byte.
/// If the next byte IS a 0 it is a copy instruction.
/// If the next byte IS NOT a 0 it is a remove instruction.
/// If the that should have been INSTRUCTION_BYTE is not equal to INSTRUCTION_BYTE's value it is an add instruction.
pub const INSTRUCTION_BYTE: u8 = 0;

/// The size of the source and target chunk stored in memory, in bytes.
/// Increasing this will increase the memory required for each chunk exponentially. (Due to the current Lcs implementation.)
pub const WANTED_CHUNK_SIZE: u8 = 255;

/// The average amount of induvidual instructions per chunk
const AVERAGE_INSTRUCTION_AMOUNT: u8 = 5;

///The maximum percent of values in a copy instruction that **are not** equal to INSTRUCTION_BYTE's value.
pub const NON_INSTRUCTION_BYTE_COUNT_PERCENT: usize = 50;

#[cfg(test)]
mod tests {
    use crate::lcs::Lcs;

    #[test]
    fn testing() {
        let source = vec![4, 3, 2, 0, 7, 5, 2, 9, 0, 1, 3, 1, 8, 2, 9];
        let target = vec![0, 7, 4, 5, 2, 5, 1, 0, 1, 1, 1, 1, 6, 9, 1];
        let lcs = Lcs::new(&source, &target).subsequence();
        dbg!(&lcs);
        dbg!(lcs.len());
        dbg!(source.len(), target.len());
    }
}
