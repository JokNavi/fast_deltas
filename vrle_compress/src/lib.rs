pub mod encoder;

#[cfg(test)]
mod tests {
    use crate::encoder::vrle_encode;
    use std::{
        env, fs::OpenOptions, io::{self, BufWriter}, path::Path
    };

    #[test]
    fn encode_exe() -> io::Result<()> {
        let patch_path = Path::new("../test_files/exe/patch.dpatch");
        let encoded_patch_path = Path::new("../test_files/exe/encoded_patch.dpatch");
        buffer_vlre_encode(patch_path, encoded_patch_path)
    }

    fn buffer_vlre_encode(reader_path: &Path, writer_path: &Path) -> io::Result<()> {
        use std::io::BufReader;
        const CHUNK_SIZE: usize = 1024;
        let reader_file = OpenOptions::new().read(true).open(reader_path)?;
        let writer_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(writer_path)?;
        let mut reader = BufReader::with_capacity(CHUNK_SIZE, reader_file);
        let mut writer = BufWriter::with_capacity(CHUNK_SIZE, writer_file);
        vrle_encode(&mut reader, &mut writer)?;
        Ok(())
    }
}
