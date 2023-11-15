pub mod encoder;
pub mod lcs;
pub type ChunkLength = u8;

pub const CHUNK_SIZE: ChunkLength = 255;
pub const CHUNK_SIZE_BYTE_COUNT: usize = ChunkLength::BITS as usize / 8;
pub const BUFFER_SIZE: usize = CHUNK_SIZE as usize + 1;

pub const ZERO_ITEM_COUNT_PERCENT: usize = 100;

pub const REMOVE_INSTRUCTION_SIGN: u8 = b'-';

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcs::Lcs;

    #[test]
    fn testing() {
        let source: Vec<u8> = vec![1, 1, 2, 2, 2, 1, 1];
        let target: Vec<u8> = vec![1, 1, 3, 3, 3, 1, 1];
        let lcs = Lcs::new(&source, &target).subsequence();
        let test_vec: Vec<u8> = vec![0, 1, 1, 1, 0, 0, 0];
        let zero_amount = test_vec.iter().filter(|num| **num == 0).count();
        const P: usize = 25;
        dbg!(0u8.wrapping_sub(2));
        dbg!(2u8.wrapping_add(254));
        dbg!((zero_amount * 100) / test_vec.len());
    }
}
