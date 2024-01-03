use super::remove_instruction_iterator::RemoveInstructionIterator;

#[derive(Debug, Clone)]
pub struct AddInstructionIterator<'a> {
    iterator: RemoveInstructionIterator<'a>,
}

impl<'a> AddInstructionIterator<'a> {
    pub fn new<T>(target: T, next_lcs_item: Option<&'a u8>) -> Self
    where
        T: IntoIterator<Item = &'a u8, IntoIter = std::slice::Iter<'a, u8>>,
    {
        AddInstructionIterator {
            iterator: RemoveInstructionIterator::new(target, next_lcs_item),
        }
    }
}

impl Iterator for AddInstructionIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

#[cfg(test)]
mod remove_instruction_iterator_tests {
    use super::*;
    use crate::lcs::Lcs;

    #[test]
    fn test_next() {
        let source = vec![1, 1, 1, 2, 2, 1];
        let target = vec![0, 0, 0, 2, 3, 0];
        let lcs = Lcs::new(&source, &target).subsequence();
        let mut copy_instruction_iterator = AddInstructionIterator::new(&target, lcs.first());
        assert!(copy_instruction_iterator.next().is_some());
        assert!(copy_instruction_iterator.next().is_some());
        assert!(copy_instruction_iterator.next().is_some());
        assert!(copy_instruction_iterator.next().is_none());
        assert!(copy_instruction_iterator.next().is_some());
        assert!(copy_instruction_iterator.next().is_some());
    }
}
