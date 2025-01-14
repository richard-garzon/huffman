mod encoding;

use std::fs::File;

use clap::Parser;

use encoding::frequency::Freq;

#[derive(Parser)]
#[command(name = "huff")]
#[command(about = "huffman encoder/decoder", long_about = None)]

struct Cli {
    file_name: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut freq = Freq::new();

    let file = match cli.file_name {
        Some(file_name) => File::open(&file_name)
            .unwrap_or_else(|_| panic!("Failed while opening file {}", &file_name)),
        None => {
            panic!("Must pass valid file path to huff")
        }
    };

    freq.count_chars(file);

    for (key, value) in freq.counter.into_iter() {
        println!("{} -> {}", key, value);
    }
}
