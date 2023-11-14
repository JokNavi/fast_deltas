pub mod encoder;
mod lcs;
pub type ChunkLength = u8;
pub const ZERO_ITEM_COUNT_PERCENT: usize = 100;
pub const REMOVE_INSTRUCTION_SIGN: u8 = b'-';

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
