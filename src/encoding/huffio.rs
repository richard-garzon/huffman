use std::fs::File;
use std::io::{self, BufWriter, Write};

pub fn write_size_header(file: &File, size: u32) -> io::Result<()> {
    let mut writer = BufWriter::new(file);
    writer.write_all(&size.to_be_bytes()).unwrap();
    writer.flush().unwrap();
    Ok(())
}

pub fn write_compressed_data(file: &File, data: Vec<u8>) -> io::Result<()> {
    let mut writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();
    writer.flush().unwrap();
    Ok(())
}
