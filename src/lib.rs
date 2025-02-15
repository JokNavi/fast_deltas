pub mod encoder;
pub mod lcs;

/// The byte that (on average) occurs most when taking the difference between 2 slices.
pub(crate) const MOST_COMMON_DIFF_BYTE: u8 = 0;

/// ### Special: Check next byte.
/// If the next byte IS a 0 it is a copy instruction.
/// If the next byte IS NOT a 0 it is a remove instruction.
/// If the that should have been INSTRUCTION_BYTE is not equal to INSTRUCTION_BYTE's value it is an add instruction.
pub(crate) const INSTRUCTION_BYTE: u8 = MOST_COMMON_DIFF_BYTE;

#[cfg(feature = "half_match")]
///The maximum percent of values in a copy instruction that **are not** equal to INSTRUCTION_BYTE's value.
pub(crate) const NON_INSTRUCTION_BYTE_COUNT_PERCENT: f32 = 50.0;
