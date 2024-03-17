use clap::{arg, crate_name, crate_version, value_parser, Command};
use fast_deltas::encoder::delta_encode;
use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

fn get_command() -> Command {
    Command::new(crate_name!())
        .author("Wannes Vantorre")
        .version(crate_version!())
        .about("Explains in brief what the program does")
        .args(&[
            arg!(<Source> "Path to the original version of the file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
            arg!(<Target> "Path to the new version of the file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
            arg!([Patch] "Path to save the patch file at")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
            arg!(-c --compress "Whether or not to xz compress the patch file."),
        ])
}

fn main() -> io::Result<()> {
    let matches = get_command().get_matches();
    let source_path: PathBuf = matches.get_one::<PathBuf>("Source").unwrap().to_path_buf();
    let target_path: PathBuf = matches.get_one::<PathBuf>("Target").unwrap().to_path_buf();
    let patch_path: PathBuf = matches
        .get_one::<PathBuf>("Patch")
        .unwrap_or(&PathBuf::from("patch.dpatch"))
        .to_path_buf();
    let should_compress = matches.get_flag("compress");

    write_delta(&source_path, &target_path, &patch_path)?;
    if should_compress {
        xz_compress(&patch_path, &patch_path.with_extension("dpatch.xz"), 9)?;
    }

    Ok(())
}

fn write_delta(source_path: &Path, target_path: &Path, patch_path: &Path) -> io::Result<()> {
    let source_file = OpenOptions::new().read(true).open(source_path)?;
    let target_file = OpenOptions::new().read(true).open(target_path)?;
    let patch_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(patch_path)?;
    delta_encode(source_file, target_file, patch_file)?;
    Ok(())
}

fn xz_compress(
    reader_path: &Path,
    writer_path: &Path,
    level: u8,
) -> io::Result<()> {
    use std::io::BufReader;
    use xz::write::XzEncoder;
    const CHUNK_SIZE: usize = 1024;
    let reader_file = OpenOptions::new()
        .read(true)
        .open(reader_path)?;
    let writer_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(writer_path)?;
    let mut reader = BufReader::with_capacity(CHUNK_SIZE, reader_file);
    let mut writer = XzEncoder::new(writer_file, level.into());
    let mut buffer: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];
    let mut bytes_read: usize = reader.read(buffer.as_mut())?;

    while bytes_read > 0 {
        writer.write(&buffer[..bytes_read])?;
        bytes_read = reader.read(buffer.as_mut())?;
    }

    Ok(())
}
