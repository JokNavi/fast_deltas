mod lcs;
pub mod patch;
pub type ChunkLength = u8;
pub const ZERO_ITEM_COUNT_PERCENT: u8 = 0;

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
