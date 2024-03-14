pub mod encoder;
pub mod lcs;

/// The byte that (on average) occurs most when taking the difference between 2 slices.
pub(crate) const MOST_COMMON_DIFF_BYTE: u8 = 0;

/// ### Special: Check next byte.
/// If the next byte IS a 0 it is a copy instruction.
/// If the next byte IS NOT a 0 it is a remove instruction.
/// If the that should have been INSTRUCTION_BYTE is not equal to INSTRUCTION_BYTE's value it is an add instruction.
pub(crate) const INSTRUCTION_BYTE: u8 = MOST_COMMON_DIFF_BYTE;

///The maximum percent of values in a copy instruction that **are not** equal to INSTRUCTION_BYTE's value.
pub(crate) const NON_INSTRUCTION_BYTE_COUNT_PERCENT: f32 = 50.0;

pub(crate) const MIN_MATCHING_PREFIX_LENGTH: usize = 8;

#[cfg(test)]
mod tests {
    use std::{fs::OpenOptions, io};

    use crate::encoder::delta_encode;

    #[test]
    fn test_encoder_exe() -> io::Result<()> {
        let source = OpenOptions::new()
            .read(true)
            .open("test_files/exe/char_art_old.exe")?;
        let target = OpenOptions::new()
            .read(true)
            .open("test_files/exe/char_art_new.exe")?;
        let patch = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("test_files/exe/patch.dpatch")?;
        delta_encode(source, target, patch)?;
        Ok(())
    }

    #[test]
    fn test_encoder_text() -> io::Result<()> {
        let source = OpenOptions::new()
            .read(true)
            .open("test_files/text/source.txt")?;
        let target = OpenOptions::new()
            .read(true)
            .open("test_files/text/target.txt")?;
        let patch = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("test_files/text/patch.dpatch")?;
        delta_encode(source, target, patch)?;
        Ok(())
    }
}
