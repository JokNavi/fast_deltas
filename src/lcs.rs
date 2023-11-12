pub type ChunkLength = u8;

#[derive(Debug, Clone, PartialEq)]
pub struct Lcs<'a> {
    source: &'a [u8],
    target: &'a [u8],
    table: Vec<Vec<ChunkLength>>,
}

impl<'a> Lcs<'a> {
    ///Max length for Source and target is `ChunkLength::MAX`!
    pub fn new(source: &'a [u8], target: &'a [u8]) -> Self {
        let source_length = source.len();
        let target_length = target.len();
        assert!(source_length <= ChunkLength::MAX as usize);
        assert!(target_length <= ChunkLength::MAX as usize);
        let mut table: Vec<Vec<ChunkLength>> = vec![vec![0; target_length + 1]; source_length + 1];

        for x in 0..=source_length {
            for y in 0..=target_length {
                if x == 0 || y == 0 {
                    table[x][y] = 0
                } else if source[x - 1] == target[y - 1] {
                    table[x][y] = table[x - 1][y - 1] + 1
                } else {
                    table[x][y] = table[x - 1][y].max(table[x][y - 1])
                }
            }
        }

        Self {
            table,
            source,
            target,
        }
    }

    ///Returns the length of the longest common subsequence, NOT the amount of values in the table.
    pub fn len(&self) -> usize {
        let source_length = self.source.len();
        let target_length = self.target.len();
        self.table[source_length][target_length].into()
    }

    ///Checks if `Lcs.len()`` is equal to 0.
    pub fn is_empty(&self) -> bool {
        self.source.len() == 0 || self.target.len() == 0
    }

    ///Returns the longest common subsequence in source and target
    pub fn subsequence(&self) -> Vec<u8> {
        let mut index = self.len();
        let mut subsequence: Vec<u8> = vec![0; index + 1];

        let mut x = self.source.len();
        let mut y = self.target.len();
        while x > 0 && y > 0 {
            if self.source[x - 1] == self.target[y - 1] {
                subsequence[index - 1] = self.source[x - 1];
                x -= 1;
                y -= 1;
                index -= 1
            } else if self.table[x - 1][y] > self.table[x][y - 1] {
                x -= 1
            } else {
                y -= 1
            }
        }

        subsequence.pop();
        subsequence
    }
}

#[cfg(test)]
mod lcs_tests {
    use std::vec;

    use super::*;

    #[test]
    fn new_ok() {
        let source = vec![0; ChunkLength::MAX.into()];
        let target = source.clone();
        let lcs = Lcs::new(&source, &target);
        assert_eq!(
            lcs.table
                .iter()
                .flatten()
                .map(|num| *num as usize)
                .sum::<usize>(),
            5559680
        );
    }

    #[test]
    #[should_panic]
    fn new_panic() {
        let source = vec![0; usize::from(ChunkLength::MAX) + 1];
        let target = source.clone();
        Lcs::new(&source, &target);
    }

    #[test]
    fn is_empty() {
        let source = vec![0; ChunkLength::MAX.into()];
        let target = vec![];
        assert!(Lcs::new(&source, &target).is_empty());
    }

    #[test]
    fn len() {
        let source = vec![0; ChunkLength::MAX.into()];
        let target = source.clone();
        assert_eq!(Lcs::new(&source, &target).len(), 255);
    }

    #[test]
    fn subsequence() {
        let source: Vec<u8> = (0..ChunkLength::MAX).collect();
        let target = source.clone();
        let lcs = Lcs::new(&source, &target);
        let subsequence = lcs.subsequence();
        assert_eq!(subsequence, source);
        assert_eq!(subsequence.len(), lcs.len());

        let lcs = Lcs::new(b"XMJYAUZ", b"MZJAWXU");
        let subsequence = lcs.subsequence();
        assert_eq!(subsequence, b"MJAU");
        assert_eq!(subsequence.len(), lcs.len());

        let source = vec![0; ChunkLength::MAX.into()];
        let target = vec![];
        let lcs = Lcs::new(&source, &target);
        let subsequence = lcs.subsequence();
        assert_eq!(subsequence, vec![]);
        assert_eq!(subsequence.len(), lcs.len());
    }
}