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

    while source_bytes_read > 0 && target_bytes_read > 0 {
        let instruction_bytes = write_instruction(
            &source_buffer[..source_bytes_read],
            &target_buffer[..target_bytes_read],
        );
        patch_writer.write(&instruction_bytes);

        source_bytes_read = source_reader.read(&mut source_buffer)?;
        target_bytes_read = target_reader.read(&mut target_buffer)?;
    }

    Ok(())
}

fn write_instruction(source: &[u8], target: &[u8]) -> Vec<u8> {
    let lcs = Lcs::new(&source, &target).subsequence();
    
}
