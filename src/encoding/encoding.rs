use super::{
    bitstuff::BitWriter,
    frequency::{self, Freq},
    tree::{generate_tree, HuffNode},
};
use std::io::{BufReader, Cursor, Read, Result};
use std::{collections::HashMap, fs::File};

fn get_prefix_size(its_a_byte: u8) -> u8 {
    if its_a_byte == 0 {
        return 1;
    }

    return 8 - its_a_byte.leading_zeros() as u8;
}

pub fn get_prefixes(node: &Option<Box<HuffNode>>, state: u8, prefix: &mut HashMap<char, (u8, u8)>) {
    if node.is_none() {
        return;
    }

    if let Some(curr_node) = node {
        if let Some(character) = curr_node.character {
            prefix.insert(character, (state, get_prefix_size(state)));
        } else {
            get_prefixes(&curr_node.left, state << 1, prefix);
            get_prefixes(&curr_node.right, state << 1 | 1, prefix);
        }
    }
}

/// Generates a prefix table from a Huffman tree
/// it takes char as a key. the tuple values are (prefix, number of meaningful bits)
/// the meaningful bits piece is used so we know how many to write while encoding
/// so we can pack bits tight
pub fn generate_prefix_table(node: Option<Box<HuffNode>>) -> HashMap<char, (u8, u8)> {
    let mut prefix_table = HashMap::new();
    let mut state: u8 = 0;

    get_prefixes(&node, state, &mut prefix_table);

    prefix_table
}

pub fn generate_header(node: &Option<Box<HuffNode>>, bw: &mut BitWriter) {
    if node.is_none() {
        return;
    }

    let curr_node = node.as_ref().unwrap();

    match curr_node.character {
        Some(character) => {
            let bits = character as u32;
            bw.write_bit(1);
            bw.write_bits(bits, 32);
        }

        None => {
            bw.write_bit(0);
        }
    }

    generate_header(&curr_node.left, bw);
    generate_header(&curr_node.right, bw);
}

pub fn get_tree_header_with_size(node: &Option<Box<HuffNode>>) -> (u32, Vec<u8>) {
    let mut bw = BitWriter::new();
    generate_header(&node, &mut bw);
    let header = bw.get_vec().unwrap();
    let header_size = header.len() as u32;

    (header_size, header)
}

fn get_encoded_data_with_header_impl(
    prefix_table: &HashMap<char, (u8, u8)>,
    bw: &mut BitWriter,
    incomplete: &mut Vec<u8>,
    chunk: &[u8],
) {
    let mut data = incomplete.clone();
    data.extend_from_slice(&chunk);

    let (valid, curr_incomplete) = match std::str::from_utf8(&data) {
        Ok(valid_str) => (valid_str, &[] as &[u8]),
        Err(e) => {
            let valid_up_to = e.valid_up_to();
            (
                std::str::from_utf8(&data[..valid_up_to]).unwrap(),
                &data[valid_up_to..],
            )
        }
    };

    for ch in valid.chars() {
        let &(curr_prefix, meaningful_bits) = prefix_table.get(&ch).unwrap();
        bw.write_bits(curr_prefix as u32, meaningful_bits);
    }

    incomplete.clear();
    incomplete.extend_from_slice(curr_incomplete);
}

pub fn get_encoded_data_with_header<R: Read>(
    file: R,
    prefix_table: HashMap<char, (u8, u8)>,
) -> (u32, Vec<u8>) {
    let mut bw = BitWriter::new();
    let mut incomplete: Vec<u8> = Vec::new();

    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1024];

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        get_encoded_data_with_header_impl(
            &prefix_table,
            &mut bw,
            &mut incomplete,
            &buffer[..bytes_read],
        );
    }

    let data = bw.get_vec().unwrap();
    let data_size = data.len() as u32;

    (data_size, data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_letter_prefix_table() {
        let mut freq = Freq::new();
        let test_input = "aaa".as_bytes();
        freq.update(test_input);
        let root = generate_tree(&freq);

        let prefix_table = generate_prefix_table(root);

        assert_eq!(prefix_table.get(&'a').unwrap(), &(0, 1));
    }

    #[test]
    fn test_two_letter_prefix_table() {
        let mut freq = Freq::new();
        let test_input = "aaab".as_bytes();
        freq.update(test_input);
        let root = generate_tree(&freq);

        let prefix_table = generate_prefix_table(root);

        assert_eq!(prefix_table.get(&'a').unwrap(), &(1, 1));
        assert_eq!(prefix_table.get(&'b').unwrap(), &(0, 1));
    }

    #[test]
    fn test_three_letter_prefix_table() {
        let mut freq = Freq::new();
        let test_input = "aaabcccc".as_bytes();
        freq.update(test_input);
        let root = generate_tree(&freq);

        let prefix_table = generate_prefix_table(root);
        /*
        for (key, value) in prefix_table.into_iter() {
            println!("{} -> {}", key, value);
        }
        */
        assert_eq!(prefix_table.get(&'a').unwrap(), &(3, 2));
        assert_eq!(prefix_table.get(&'b').unwrap(), &(2, 2));
        assert_eq!(prefix_table.get(&'c').unwrap(), &(0, 1));
    }

    // Lil utility function for printing u8 as bits
    fn _print_as_bytes(byte_vec: Vec<u8>) {
        for byte in byte_vec {
            println!("{:08b}", byte);
        }
    }

    #[test]
    fn test_header_generation_one_node() {
        let mut freq = Freq::new();
        let test_input = "aaa".as_bytes();
        freq.update(test_input);
        let root = generate_tree(&freq);
        let mut bw = BitWriter::new();
        let expected = vec![
            0b10000000, // first bit is 1, for the one node, this is followed
            0b00000000, // by the bit representation of 'a' as a u32, since
            0b00000000, // char in Rust is 4 bytes. then we have the two trailing
            0b00110000, // 0s and padding in the fifth & last byte
            0b10000000,
        ];

        generate_header(&root, &mut bw);
        let result = bw.get_vec().ok().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_header_generation_two_nodes() {
        let mut freq = Freq::new();
        let test_input = "aab".as_bytes();
        freq.update(test_input);
        let root = generate_tree(&freq);
        let mut bw = BitWriter::new();
        let expected = vec![
            0b01000000, 0b00000000, 0b00000000, 0b00011000, 0b10100000, 0b00000000, 0b00000000,
            0b00001100, 0b00100000,
        ];

        generate_header(&root, &mut bw);
        let result = bw.get_vec().ok().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_header_generation_three_nodes() {
        let mut freq = Freq::new();
        let test_input = "abbccccc".as_bytes();
        freq.update(test_input);
        let root = generate_tree(&freq);
        let mut bw = BitWriter::new();
        let expected = vec![
            0b00100000, 0b00000000, 0b00000000, 0b00001100, 0b00110000, 0b00000000, 0b00000000,
            0b00000110, 0b00101000, 0b00000000, 0b00000000, 0b00000011, 0b00011000,
        ];

        generate_header(&root, &mut bw);
        let result = bw.get_vec().ok().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_tree_header_with_size() {
        let mut freq = Freq::new();
        let test_input = "aaa".as_bytes();
        freq.update(test_input);
        let root = generate_tree(&freq);
        let expected_size = 5;
        let expected = vec![0b10000000, 0b00000000, 0b00000000, 0b00110000, 0b10000000];

        let (header_size, header) = get_tree_header_with_size(&root);

        assert_eq!(header_size, expected_size);
        assert_eq!(header, expected);
    }

    #[test]
    fn test_get_tree_header_with_size_two_nodes() {
        let mut freq = Freq::new();
        let test_input = "aab".as_bytes();
        freq.update(test_input);
        let root = generate_tree(&freq);
        let expected_size = 9;
        let expected = vec![
            0b01000000, 0b00000000, 0b00000000, 0b00011000, 0b10100000, 0b00000000, 0b00000000,
            0b00001100, 0b00100000,
        ];

        let (header_size, header) = get_tree_header_with_size(&root);

        assert_eq!(header_size, expected_size);
        assert_eq!(header, expected);
    }

    #[test]
    fn test_get_tree_header_with_size_three_nodes() {
        let mut freq = Freq::new();
        let test_input = "abbccccc".as_bytes();
        freq.update(test_input);
        let root = generate_tree(&freq);
        let expected_size = 13;
        let expected = vec![
            0b00100000, 0b00000000, 0b00000000, 0b00001100, 0b00110000, 0b00000000, 0b00000000,
            0b00000110, 0b00101000, 0b00000000, 0b00000000, 0b00000011, 0b00011000,
        ];

        let (header_size, header) = get_tree_header_with_size(&root);

        assert_eq!(header, expected);
        assert_eq!(header_size, expected_size);
    }

    #[test]
    fn test_get_encoded_data_with_header() {
        let mut freq = Freq::new();
        let test_input = "aaa".as_bytes();
        freq.update(test_input);
        let root = generate_tree(&freq);
        let prefix_table = generate_prefix_table(root);
        let test_file = Cursor::new(test_input.to_vec());
        let expected_size = 1;
        let expected = vec![0b00000000];

        let (data_size, encoded_data) = get_encoded_data_with_header(test_file, prefix_table);

        assert_eq!(data_size, expected_size);
        assert_eq!(encoded_data, expected);
    }
}
