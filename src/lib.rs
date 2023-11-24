pub mod encoder;
pub mod lcs;

/// ### Special: Check next byte.
/// If the next byte IS a 0 it is a copy instruction.
/// If the next byte IS NOT a 0 it is a remove instruction.
/// If the that should have been INSTRUCTION_BYTE is not equal to 0 it is an add instruction.
pub const INSTRUCTION_BYTE: u8 = 0;

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
        let source: Vec<u8> = vec![2,4,1,3,0,4,8,5,1,5,1,7,8,0,3,9,4,6,0,6,5,8,3,4,2,4,2,6,2,8,9,9,5,0,7,4,4,0,4,0,7,3,5,5,9,6,3,2,9,4,8,2,8,8,5,8,8,6,3,7,5,7,5,9,7,6,2,3,1,3,2,0,3,9,9,6,0,7,6,0,8,1,7,2,9,4,1,6,2,9,7,3,7,1,1,5,1,6,1,0];
        let target: Vec<u8> = vec![7,1,2,7,4,1,1,6,4,4,9,9,7,3,0,1,6,8,1,2,3,6,0,4,6,8,2,3,0,4,6,7,3,0,8,7,9,6,3,8,0,2,3,4,2,5,3,9,6,2,0,5,2,6,7,3,8,9,9,1,3,9,0,0,6,4,4,7,2,1,5,5,5,5,8,8,1,5,2,8,7,0,7,5,9,5,1,0,8,9,1,6,3,2,8,9,5,7,4,4];
        let source: Vec<u8> = vec![1, 2, 4, 1, 9, 7, 3, 0, 1, 6, 8, 3, 6, 0, 4, 6, 2, 4, 0, 7, 9, 0, 3, 4, 5, 3, 9, 6, 2, 5, 2, 6, 7, 8, 3, 9, 0, 6, 4, 7, 2, 1, 5, 1, 2, 8, 0, 7, 9, 1, 9, 1, 6, 3, 2, 5, 7, 4];
        let target: Vec<u8> = vec! [2, 1, 4, 1, 1, 7, 0, 3, 9, 6, 0, 6, 8, 3, 4, 4, 2, 6, 9, 0, 7, 4, 0, 3, 5, 6, 3, 2, 9, 4, 2, 5, 8, 6, 3, 7, 5, 9, 7, 6, 2, 1, 0, 0, 8, 1, 7, 2, 9, 4, 1, 6, 2, 9, 3, 7, 1, 5];
        let lcs = Lcs::new(&source, &target).subsequence();
        dbg!(&lcs);
        dbg!(lcs.len());
    }
}
