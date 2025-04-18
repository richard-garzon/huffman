mod encoding;

use std::fs::File;
use std::io::{Seek, SeekFrom};

use clap::Parser;

use encoding::encoding::{generate_prefix_table, get_encoded_data, get_tree_header_with_size};
use encoding::frequency::Freq;
use encoding::huffio::{decompress_data, write_compressed_data, write_size_header};
use encoding::tree::generate_tree;

#[derive(Parser)]
#[command(name = "huff")]
#[command(about = "huffman encoder/decoder", long_about = None)]

struct Cli {
    file_name: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut freq = Freq::new();

    let mut file = match &cli.file_name {
        Some(file_name) => File::open(&file_name)
            .unwrap_or_else(|_| panic!("Failed while opening file {}", &file_name)),
        None => {
            panic!("Must pass valid file path to huff")
        }
    };

    freq.count_chars(&file);

    let root = generate_tree(&freq);

    let (header_size, header) = get_tree_header_with_size(&root);

    let prefix_table = generate_prefix_table(root);

    let _ = &file.seek(SeekFrom::Start(0));

    let data = get_encoded_data(&file, prefix_table);

    let output_filename = &mut cli.file_name.unwrap().clone();
    output_filename.push_str("_huff");
    {
        let encoded_file = File::create(&output_filename).unwrap();

        write_size_header(&encoded_file, header_size).unwrap();
        write_compressed_data(&encoded_file, header).unwrap();
        write_compressed_data(&encoded_file, data).unwrap();
    }
    let mut compressed_file = File::open(output_filename).unwrap();
    decompress_data(&mut compressed_file, String::from("decoded_file.txt")).unwrap();
}
