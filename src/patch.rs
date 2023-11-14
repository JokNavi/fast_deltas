use std::io::{self, BufReader, BufWriter, Read, Write};

use crate::{lcs::Lcs, ChunkLength, REMOVE_INSTRUCTION_SIGN, ZERO_ITEM_COUNT_PERCENT};

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

    let mut instruction_buffer: Vec<u8> = Vec::with_capacity(ChunkLength::MAX as usize + 1);
    while lcs_index < lcs.len() {
        debug_assert!(lcs_index <= source_index && lcs_index <= target_index);
        if source_index < source.len() && lcs[lcs_index] != source[source_index] {
            //Remove
            source_index +=
                write_remove_instruction(&source[source_index..], &lcs, &mut instruction_buffer);
        } else if target_index < target.len() && lcs[lcs_index] != target[target_index] {
            //Add
            target_index +=
                write_add_instruction(&target[target_index..], &lcs, &mut instruction_buffer);
        } else {
            //Copy
            let bytes_written = write_copy_instruction(
                &source[source_index..],
                &target[target_index..],
                &lcs[lcs_index..],
                &mut instruction_buffer,
            );
            source_index += bytes_written;
            target_index += bytes_written;
            lcs_index += bytes_written;
        }
    }
    while source_index < source.len() {
        //Remove
        source_index +=
            write_remove_instruction(&source[source_index..], &lcs, &mut instruction_buffer);
    }
    while target_index < target.len() {
        //Add
        target_index +=
            write_add_instruction(&target[target_index..], &lcs, &mut instruction_buffer);
    }
    instruction_buffer
}

///Returns the amount of source bytes used
fn write_remove_instruction(source: &[u8], lcs: &[u8], instruction_buffer: &mut Vec<u8>) -> usize {
    instruction_buffer.push(REMOVE_INSTRUCTION_SIGN);
    let mut source_index: usize = 0;
    while source_index < source.len() && lcs[0] != source[source_index] {
        source_index += 1;
    }
    instruction_buffer.append(&mut ChunkLength::to_be_bytes(source_index as ChunkLength).to_vec());
    source_index
}

/// Returns the amount of target bytes used
fn write_add_instruction(target: &[u8], lcs: &[u8], instruction_buffer: &mut Vec<u8>) -> usize {
    let mut target_index: usize = 0;
    while target_index < target.len() && lcs[0] != target[target_index] {
        instruction_buffer.push(target[target_index]);
        target_index += 1;
    }
    target_index
}

/// Returns the amount of lcs bytes written
fn write_copy_instruction(
    source: &[u8],
    target: &[u8],
    lcs: &[u8],
    instruction_buffer: &mut Vec<u8>,
) -> usize {
    let mut lcs_index: usize = 0;
    let mut source_index: usize = 0;
    let mut target_index: usize = 0;
    let mut buffer_zero_count = buffer_zero_count(instruction_buffer);
    //((true && (true && true))
    //|| (true))
    //&& true
    //&& (true)
    while ((lcs_index < lcs.len() 
    && (lcs[lcs_index] == source[lcs_index] && lcs[lcs_index] == target[lcs_index]))
    || (calc_percent(buffer_zero_count, instruction_buffer.len()) <= ZERO_ITEM_COUNT_PERCENT))
    && (source_index < source.len() && target_index < target.len()) 
    {
        instruction_buffer.push(target[target_index].wrapping_sub(source[source_index]));
        source_index += 1;
        target_index += 1;
        lcs_index += 1;
    }
    lcs_index
}


fn buffer_zero_count(buffer: &mut Vec<u8>) -> usize {
    buffer.iter().filter(|item| **item == 0).count() 
}

fn calc_percent(value: usize, buff_length: usize) -> usize {
    ((value / 100) *  buff_length) as usize
}