use std::fs::File;
use std::io::{BufReader, Read};

use clap::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "huff")]
#[command(about = "huffman encoder/decoder", long_about = None)]

struct Cli {
    file_name: Option<String>,
}

fn count_chars(file: File) -> HashMap<char, usize> {
    let mut counter: HashMap<char, usize> = HashMap::new();

    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1024];

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let chunk = &buffer[..bytes_read];
        for &byte in chunk {
            if let Some(ch) = char::from_u32(byte as u32) {
                counter
                    .entry(ch)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }

    return counter;
}

fn main() {
    let cli = Cli::parse();

    let file = match cli.file_name {
        Some(file_name) => File::open(&file_name)
            .unwrap_or_else(|_| panic!("Failed while opening file {}", &file_name)),
        None => {
            panic!("Must pass valid file path to huff")
        }
    };

    let counter = count_chars(file);

    for (key, value) in counter.into_iter() {
        println!("{} -> {}", key, value);
    }
}
