use std::io::{self, BufReader, BufWriter, Read, Write};

use crate::{lcs::Lcs, ChunkLength};

pub fn delta_encode<R: Read, W: Write>(source: R, target: R, patch: W) -> io::Result<()> {
    let mut source_reader = BufReader::new(source);
    let mut target_reader = BufReader::new(target);
    let mut patch_writer = BufWriter::new(patch);

    let mut source_buffer = [0u8; ChunkLength::MAX as usize];
    let mut target_buffer = [0u8; ChunkLength::MAX as usize];

    let mut source_bytes_read = source_reader.read(&mut source_buffer)?;
    let mut target_bytes_read = target_reader.read(&mut target_buffer)?;

    while source_bytes_read > 0 || target_bytes_read > 0 {
        let instruction_bytes = write_instructions_chunk(
            &source_buffer[..source_bytes_read],
            &target_buffer[..target_bytes_read],
        );
        patch_writer.write(&instruction_bytes)?;

        source_bytes_read = source_reader.read(&mut source_buffer)?;
        target_bytes_read = target_reader.read(&mut target_buffer)?;
    }

    Ok(())
}

fn write_instructions_chunk(source: &[u8], target: &[u8]) -> Vec<u8> {
    let lcs = Lcs::new(&source, &target).subsequence();
    let mut source_index: usize = 0;
    let mut target_index: usize = 0;
    let mut lcs_index: usize = 0;

    let mut instruction_buffer: Vec<u8> = Vec::with_capacity(ChunkLength::MAX as usize);
    while lcs_index < lcs.len() {
        debug_assert!(lcs_index <= source_index && lcs_index <= target_index);
        if source_index < source.len() && lcs[lcs_index] != source[source_index] {
            //Remove
            source_index += write_remove_instruction(source, target, &lcs, &mut instruction_buffer);
        } else if target_index < target.len() && lcs[lcs_index] != target[target_index] {
            //Add
            target_index += write_add_instruction(source, target, &lcs, &mut instruction_buffer);
        } else {
            //Copy
            let bytes_written =
                write_copy_instruction(source, target, &lcs, &mut instruction_buffer);
            source_index += bytes_written;
            target_index += bytes_written;
            lcs_index += bytes_written;
        }
    }
    while source_index < source.len() {
        //Remove
        source_index += write_remove_instruction(source, target, &lcs, &mut instruction_buffer);
    }
    while target_index < target.len() {
        //Add
        target_index += write_add_instruction(source, target, &lcs, &mut instruction_buffer);
    }
    instruction_buffer
}
///Returns the amount of bytes written
fn write_remove_instruction(
    source: &[u8],
    target: &[u8],
    lcs: &[u8],
    instruction_buffer: &mut [u8],
) -> usize {
    todo!();
}

/// Returns the amount of bytes written
fn write_add_instruction(
    source: &[u8],
    target: &[u8],
    lcs: &[u8],
    instruction_buffer: &mut [u8],
) -> usize {
    todo!();
}

/// Returns the amount of bytes written
fn write_copy_instruction(
    source: &[u8],
    target: &[u8],
    lcs: &[u8],
    instruction_buffer: &mut [u8],
) -> usize {
    todo!();
}
