pub mod encoder;
pub mod lcs;

/// The size of the source and target chunk stored in memory, in bytes.
/// Increasing this will increase the memory required for each chunk exponentially. (Due to the current Lcs implementation.)
pub const CHUNK_SIZE: u8 = 255;

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
