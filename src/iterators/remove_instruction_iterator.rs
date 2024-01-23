use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct RemoveInstructionIterator<'a> {
    source: Iter<'a, u8>,
    next_lcs_item: Option<&'a u8>,
}

impl<'a> RemoveInstructionIterator<'a> {
    pub fn new<T>(source: T, next_lcs_item: Option<&'a u8>) -> Self
    where
        T: IntoIterator<Item = &'a u8, IntoIter = std::slice::Iter<'a, u8>>,
    {
        RemoveInstructionIterator {
            source: source.into_iter(),
            next_lcs_item,
        }
    }
}

impl Iterator for RemoveInstructionIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let source_num = self.source.next();
        if source_num != self.next_lcs_item {
            return source_num.copied();
        }
        None
    }
}

#[cfg(test)]
mod remove_instruction_iterator_tests {
    use super::*;
    use crate::lcs::Lcs;

    #[test]
    fn test_next() {
        let source = vec![0, 0, 0, 2, 3, 0];
        let target = vec![1, 1, 1, 2, 2, 1];
        let lcs = Lcs::new(&source, &target).subsequence();
        let mut copy_instruction_iterator = RemoveInstructionIterator::new(&source, lcs.first());
        assert!(copy_instruction_iterator.next().is_some());
        assert!(copy_instruction_iterator.next().is_some());
        assert!(copy_instruction_iterator.next().is_some());
        assert!(copy_instruction_iterator.next().is_none());
        assert!(copy_instruction_iterator.next().is_some());
        assert!(copy_instruction_iterator.next().is_some());
    }
}
