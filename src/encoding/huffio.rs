use std::fs::File;
use std::io::{self, BufWriter, Read, Write};

use super::decoding::{decode_data, decode_tree_header_with_size};
use super::encoding::generate_prefix_table;

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

pub fn decompress_data(file: &mut File, output_filename: String) -> io::Result<()> {
    let mut output_file = File::create(output_filename).unwrap();

    /// 1. read four bytes to get header size
    let mut header_size_buf = [0u8; 4];
    file.read_exact(&mut header_size_buf).unwrap();
    let header_size = u32::from_be_bytes(header_size_buf);

    /// 2. read header and create prefix table
    let mut header_buf = vec![0u8; header_size as usize];
    file.read_exact(&mut header_buf).unwrap();
    let tree = decode_tree_header_with_size(&header_buf);
    let prefix_table = generate_prefix_table(tree);

    /// 3. get data header
    let mut data_size_buf = [0u8; 4];
    file.read_exact(&mut data_size_buf).unwrap();
    let data_size = u32::from_be_bytes(data_size_buf);

    /// 4. decode data and write to a file
    let mut data_buf = vec![0u8; data_size as usize];
    file.read_exact(&mut data_buf).unwrap();
    let data = decode_data(&data_buf, prefix_table);
    let data_str: String = data.iter().collect();
    output_file.write_all(data_str.as_bytes()).unwrap();

    Ok(())
}
