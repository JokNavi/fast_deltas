pub mod encoder;
pub mod lcs;

/// ### Special: Check next byte.
/// If the next byte IS a 0 it is a copy instruction.
/// If the next byte IS NOT a 0 it is a remove instruction.
/// If the that should have been INSTRUCTION_BYTE is not equal to INSTRUCTION_BYTE's value it is an add instruction.
pub const INSTRUCTION_BYTE: u8 = 0;

///The maximum percent of values in a copy instruction that **are not** equal to INSTRUCTION_BYTE's value.
pub const NON_INSTRUCTION_BYTE_COUNT_PERCENT: usize = 50;

pub const CHUNK_SIZE: usize = 255;

#[cfg(test)]
mod tests {
    use std::{io, fs::OpenOptions};
    use crate::{encoder::{delta_encode, copy_instruction_length}, lcs::Lcs};

    #[test]
    fn test_encoder() -> io::Result<()> {
        let source = OpenOptions::new().read(true).open("test_files/source.txt")?;
        let target = OpenOptions::new().read(true).open("test_files/target.txt")?;
        let mut patch = OpenOptions::new().read(true).write(true).create(true).open("test_files/patch.dpatch")?;
        delta_encode(source, target, &mut patch)?;
        Ok(())
    }

   
}
