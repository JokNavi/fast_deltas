use fast_deltas::encoder::delta_encode;
use std::{fs::OpenOptions, io};

fn main() -> io::Result<()> {
    let source = OpenOptions::new()
        .read(true)
        .open("test_files/char_art_old.exe")?;
    let target = OpenOptions::new()
        .read(true)
        .open("test_files/char_art_new.exe")?;
    let patch = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("char_art_patch_MCDB0.dpatch")?;
    delta_encode(source, target, patch)?;
    Ok(())
}
