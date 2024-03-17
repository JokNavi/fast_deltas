use std::io;
use fast_deltas::encoder::delta_encode;

fn main() -> io::Result<()>{
    if cfg!(feature = "exe") {
        return test_encoder_exe();
    }
    if cfg!(feature = "txt") {
        return test_encoder_text();
    }
    else {
        println!("Nothing happend. Please try again with EXE or TXT features enabled.");
    }
    Ok(())
}

fn test_encoder_exe() -> io::Result<()> {
    use std::fs::OpenOptions;
    let source = OpenOptions::new()
        .read(true)
        .open("test_files/exe/source.exe")?;
    let target = OpenOptions::new()
        .read(true)
        .open("test_files/exe/target.exe")?;
    let patch = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("test_files/exe/patch.dpatch")?;
    delta_encode(source, target, patch)?;
    Ok(())
}

fn test_encoder_text() -> io::Result<()> {
    use std::fs::OpenOptions;
    let source = OpenOptions::new()
        .read(true)
        .open("test_files/txt/source.txt")?;
    let target = OpenOptions::new()
        .read(true)
        .open("test_files/txt/target.txt")?;
    let patch = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("test_files/txt/patch.dpatch")?;
    delta_encode(source, target, patch)?;
    Ok(())
}
