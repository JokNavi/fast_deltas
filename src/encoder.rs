use crate::{lcs::Lcs, AVERAGE_INSTRUCTION_AMOUNT, INSTRUCTION_BYTE, WANTED_CHUNK_SIZE, NON_INSTRUCTION_BYTE_COUNT_PERCENT};
use std::io::{self, BufReader, BufWriter, Read, Write};

///The actual chunk size used. A few bytes are subtracted to make place for the instruction identifiers.
pub const CHUNK_SIZE: u8 = WANTED_CHUNK_SIZE - (AVERAGE_INSTRUCTION_AMOUNT * 2);

/// The capcity allocated for the instruction buffer.
pub const BUFFER_SIZE: usize = WANTED_CHUNK_SIZE as usize;

pub fn delta_encode<R: Read, W: Write>(source: R, target: R, patch: W) -> io::Result<()> {
    todo!();
}

fn fill_instruction_buffer(lcs: &[u8], source: &[u8], target: &[u8]) {
    let lcs = Lcs::new(source, target).subsequence();
    let mut source_index: usize = 0;
    let mut target_index: usize = 0;
    let mut lcs_index: usize = 0;
}

fn add_instruction_length<'a, T>(target: T, next_lcs_item: Option<&u8>) -> usize
where
    T: IntoIterator<Item = &'a u8, IntoIter = std::slice::Iter<'a, u8>>,
{
    remove_instruction_length(target, next_lcs_item)
}

fn remove_instruction_length<'a, T>(source: T, next_lcs_item: Option<&u8>) -> usize
where
    T: IntoIterator<Item = &'a u8, IntoIter = std::slice::Iter<'a, u8>>,
{
    if let Some(&item) = next_lcs_item {
        source.into_iter().position(|&x| x == item).unwrap_or(0)
    } else {
        0
    }
}

fn copy_instruction_length<'a, T>(source: T, target: T, lcs: T) -> (usize, usize)
where
    T: IntoIterator<Item = &'a u8, IntoIter = std::slice::Iter<'a, u8>>,
{
    let mut non_instruction_byte_values_count: usize = 0;
    let mut zipped_iter = source.into_iter().zip(target.into_iter()).enumerate();
    let mut lcs = lcs.into_iter().enumerate().peekable();
    while let (Some((lcs_index, lcs_num)), Some((items_index, (source_num, target_num)))) =
        (lcs.peek(), zipped_iter.next())
    {
        if source_num == *lcs_num && target_num == *lcs_num {
            lcs.next();
        } else if (non_instruction_byte_values_count as f32 / items_index as f32) * 100.0
            < NON_INSTRUCTION_BYTE_COUNT_PERCENT as f32
        {
            non_instruction_byte_values_count += 1;
        }else {
            return (*lcs_index, items_index);
        }
        
    }

    (0, 0)
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
        assert_eq!(add_instruction_length(&target, lcs.first()), 3);
    }

    #[test]
    fn test_copy_instruction_length() {
        let source = vec![0, 0, 1, 1, 1, 0];
        let target = vec![0, 0, 2, 2, 2, 0];
        let lcs = Lcs::new(&source, &target).subsequence();
        assert_eq!(copy_instruction_length(&source, &target, &lcs), (2, 4));
    }
}
