use crate::{lcs, INSTRUCTION_BYTE, NON_INSTRUCTION_BYTE_COUNT_PERCENT};
const CHUNK_SIZE: usize = u8::MAX as usize;

fn add_instruction_length(target: &[u8], next_lcs_item: Option<u8>) -> usize {
    remove_instruction_length(target, next_lcs_item)
}

fn remove_instruction_length(source: &[u8], next_lcs_item: Option<u8>) -> usize {
    if let Some(item) = next_lcs_item {
        source
            .iter()
            .position(|&x| x == item)
            .unwrap_or(source.len())
    } else {
        source.len()
    }
}

fn copy_instruction_length(source: &[u8], target: &[u8], lcs: &[u8]) -> (usize, usize) {
    let mut non_instruction_byte_values_count: usize = 0;
    let (mut item_index, mut lcs_index) = (0, 0);
    while item_index < source.len() && item_index < target.len() && lcs_index < lcs.len() {
        let diff_byte = target[item_index].wrapping_sub(source[item_index]);
        if !(diff_byte == INSTRUCTION_BYTE || diff_byte == u8::MAX - INSTRUCTION_BYTE) {
            non_instruction_byte_values_count += 1;
        }
        if source[item_index] == lcs[lcs_index] || target[item_index] == lcs[lcs_index] {
            lcs_index += 1;
        }
        if (non_instruction_byte_values_count as f32 / (item_index + 1) as f32) * 100.0
            > NON_INSTRUCTION_BYTE_COUNT_PERCENT
        {
            break;
        }
        item_index += 1;
    }
    (item_index, lcs_index)
}

#[cfg(test)]
mod encoder_tests {
    use super::*;
    use crate::lcs::Lcs;

    #[test]
    fn test_remove_instruction_length() {
        let source = [1, 1, 1, 0, 0, 0];
        let target = [0, 0, 0];
        let lcs = Lcs::new(&source, &target).subsequence();
        assert_eq!(remove_instruction_length(&source, lcs.first().copied()), 3);
    }

    #[test]
    fn test_add_instruction_length() {
        let source = [0, 0, 0];
        let target = [1, 1, 1, 0, 0, 0];
        let lcs = Lcs::new(&source, &target).subsequence();
        assert_eq!(add_instruction_length(&target, lcs.first().copied()), 3);
    }

    #[test]
    fn test_copy_instruction_length() {
        let source = [0, 0, 3, 1, 2, 4];
        let target = [0, 0, 1, 2, 3, 4];
        // let source = [5, 5, 0, 0, 3, 1, 2, 4];
        // let target = [6, 0, 0, 1, 2, 3, 4];
        // Lcs = [0, 0, 1, 2, 4];
        // Patch = [0, 2, 1, 6, 0, 0, 0, 2, 254, 0, 1, 1, 254, 0];
        let lcs = Lcs::new(&source, &target).subsequence();
        assert_eq!(copy_instruction_length(&source, &target, &lcs), (4, 4));
    }
}
