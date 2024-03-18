use std::io::{self, Read, Write};

pub fn vrle_encode<R: Read, W: Write>(mut reader: R, mut writer: W) -> io::Result<()> {
    let mut buffer = [0; 1024];
    let mut bytes_read = reader.read(&mut buffer)?;
    let mut zero_count: u8 = 0;
    while bytes_read > 0 {
        for &byte in &buffer[..bytes_read] {
            if byte == 0 && zero_count < 255 {
                zero_count += 1;
            }
            else {
                if zero_count != 0 {
                    writer.write(&[0])?;
                    writer.write(&[zero_count])?;   
                }
                zero_count = 0;
                writer.write(&[byte])?;
            }
        }
        bytes_read = reader.read(&mut buffer)?;
    }
    Ok(())
}

#[cfg(test)]
mod encoder_tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_encode_chunk() -> io::Result<()> {
        let uncompressed = [0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1];
        let mut compressed: Vec<u8> = vec![];
        let input = Cursor::new(&uncompressed);
        let output = Cursor::new(&mut compressed);
        vrle_encode(input, output)?;
        println!("{:?}", &compressed);
        Ok(())
    }
}
