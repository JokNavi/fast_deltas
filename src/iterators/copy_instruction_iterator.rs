use std::{
    iter::{Enumerate, Peekable, Zip},
    slice::Iter,
};

use crate::NON_INSTRUCTION_BYTE_COUNT_PERCENT;

#[derive(Debug, Clone)]
pub struct CopyInstructionIterator<'a> {
    lcs: Peekable<Iter<'a, u8>>,
    zipped_iter: Enumerate<Zip<Iter<'a, u8>, Iter<'a, u8>>>,
    non_instruction_byte_values_count: usize,
}

impl<'a> CopyInstructionIterator<'a> {
    pub fn new<T>(lcs: T, source: T, target: T) -> Self
    where
        T: IntoIterator<Item = &'a u8, IntoIter = std::slice::Iter<'a, u8>>,
    {
        CopyInstructionIterator {
            lcs: lcs.into_iter().peekable(),
            zipped_iter: source.into_iter().zip(target.into_iter()).enumerate(),
            non_instruction_byte_values_count: 0,
        }
    }
}

impl Iterator for CopyInstructionIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let (Some(lcs_num), Some((index, (source_num, target_num)))) =
            (self.lcs.peek(), self.zipped_iter.next())
        {
            if source_num == *lcs_num && target_num == *lcs_num {
                self.lcs.next();
                return Some(target_num.wrapping_sub(*source_num));
            } else if (self.non_instruction_byte_values_count as f32 / index as f32) * 100.0
                < NON_INSTRUCTION_BYTE_COUNT_PERCENT as f32
            {
                self.non_instruction_byte_values_count += 1;
                return Some(target_num.wrapping_sub(*source_num));
            }
            return None;
        }

        None
    }
}

#[cfg(test)]
mod copy_instruction_iterator_tests {
    use super::*;
    use crate::lcs::Lcs;

    #[test]
    fn test_next() {
        let source = vec![0, 0, 1, 1, 1, 0];
        let target = vec![0, 0, 2, 2, 2, 0];
        let lcs = Lcs::new(&source, &target).subsequence();
        let mut copy_instruction_iterator = CopyInstructionIterator::new(&lcs, &source, &target);
        assert!(copy_instruction_iterator.next().is_some());
        assert!(copy_instruction_iterator.next().is_some());
        assert!(copy_instruction_iterator.next().is_some());
        assert!(copy_instruction_iterator.next().is_some());
        assert!(copy_instruction_iterator.next().is_none());
        assert!(copy_instruction_iterator.next().is_some());
    }
}
