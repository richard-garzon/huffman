use crate::encoding::frequency::Freq;
use crate::encoding::tree::generate_tree;
use std::collections::HashMap;
use std::hash::Hash;

use super::bitreader::BitReader;
use super::bitwriter::BitWriter;
use super::tree::HuffNode;

/// how to start decoding...
/// we should first read the header from the file and rebuild the huffman tree to decode
/// and then build the prefix table
/// we should go from <string of bits> -> prefix table
///
/// next we just decode the file and restore the data to its original state
///

pub fn decode_tree_header_with_size_impl(
    tree_data: &Vec<u8>,
    br: &mut BitReader,
) -> Option<Box<HuffNode>> {
    let curr_bit = br.next().unwrap();

    if curr_bit == 1u8 {
        // it's a leaf node
        let char_bits = br.read_bits(32);

        let u32_check: [u8; 4] = char_bits
            .try_into()
            .expect("Vec<u8> must have exactly 4 elements");
        let char_as_u32 = u32::from_be_bytes(u32_check);

        let decode_char = match char::from_u32(char_as_u32) {
            Some(c) => c,
            None => panic!(
                "from_u32 error: {} is not a valid Unicode scalar value.",
                char_as_u32
            ),
        };

        let ret_node = HuffNode::new(Some(decode_char), 0);
        Some(Box::new(ret_node))
    } else {
        let left: Option<Box<HuffNode>> = decode_tree_header_with_size_impl(&tree_data, br);
        let right = decode_tree_header_with_size_impl(&tree_data, br);

        let mut ret_node = HuffNode::new(None, 0);

        ret_node.left = left;
        ret_node.right = right;

        Some(Box::new(ret_node))
    }
}

pub fn decode_tree_header_with_size(tree_data: &Vec<u8>) -> Option<Box<HuffNode>> {
    let mut br = BitReader::new(tree_data.to_vec());
    decode_tree_header_with_size_impl(tree_data, &mut br)
}

pub fn invert_prefix_table(prefix_table: HashMap<char, (u8, u8)>) -> HashMap<(u8, u8), char> {
    let mut inverted_prefix_table: HashMap<(u8, u8), char> = HashMap::new();

    for (c, (prefix, prefix_length)) in prefix_table {
        if inverted_prefix_table.contains_key(&(prefix, prefix_length)) {
            panic!(
                "Error in invert_prefix_table(), key already exists: {:?}",
                &(prefix, prefix_length)
            )
        }
        inverted_prefix_table.insert((prefix, prefix_length), c);
    }

    inverted_prefix_table
}

pub fn decode_data(data: &Vec<u8>, prefix_table: HashMap<char, (u8, u8)>) -> Vec<char> {
    // number of bits to read from the encoded data
    let last_byte = data.last().unwrap();
    let end = match last_byte {
        0 => 1,
        _ => 2,
    };
    let data_length = data.len() - end;

    let mut characters: Vec<char> = Vec::new();

    let mut br = BitReader::new(data.to_vec()); // yes i know this copies, i am lazy
    let inverted_prefix_table = invert_prefix_table(prefix_table);
    let mut curr_prefix = 0u8;
    let mut curr_prefix_length = 0u8;

    while br.get_current_byte() < data_length {
        curr_prefix = (curr_prefix << 1) | (br.next().unwrap() & 1);
        curr_prefix_length += 1;
        if inverted_prefix_table.contains_key(&(curr_prefix, curr_prefix_length)) {
            characters.push(
                inverted_prefix_table
                    .get(&(curr_prefix, curr_prefix_length))
                    .unwrap()
                    .clone(),
            );
            curr_prefix = 0u8;
            curr_prefix_length = 0u8;
        }
    }

    for _ in 0..*last_byte {
        curr_prefix = (curr_prefix << 1) | (br.next().unwrap() & 1);
        curr_prefix_length += 1;
        if inverted_prefix_table.contains_key(&(curr_prefix, curr_prefix_length)) {
            characters.push(
                inverted_prefix_table
                    .get(&(curr_prefix, curr_prefix_length))
                    .unwrap()
                    .clone(),
            );
            curr_prefix = 0u8;
            curr_prefix_length = 0u8;
        }
    }

    characters
}

#[cfg(test)]
mod tests {
    use crate::encoding::{
        encoding::{generate_prefix_table, get_encoded_data_with_header},
        test_cases::{self, SAMPLE_TEST},
    };

    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_decode_tree_header() {
        // Assume that we've read the 4 size bytes already and then returned
        // the encoded tree in a vec
        let input: Vec<u8> = vec![
            0b10000000, 0b00000000, 0b00000000, 0b00110000,
            0b10000000, // This is the encoded tree
        ];
        let expected = "aaa";
        let mut freq = Freq::new();
        freq.update(expected.as_bytes());
        let root = generate_tree(&freq);
        let expected_prefix = generate_prefix_table(root);

        let result_tree = decode_tree_header_with_size(&input);
        let result_prefix = generate_prefix_table(result_tree);

        assert_eq!(result_prefix, expected_prefix);
    }

    #[test]
    fn test_decode_two_letter_tree_header() {
        let input = vec![
            0b01000000, 0b00000000, 0b00000000, 0b00011000, 0b10100000, 0b00000000, 0b00000000,
            0b00001100, 0b00100000,
        ];
        let expected = "aab";
        let mut freq = Freq::new();
        freq.update(expected.as_bytes());
        let root = generate_tree(&freq);
        let expected_prefix = generate_prefix_table(root);

        let result_tree = decode_tree_header_with_size(&input);
        let result_prefix = generate_prefix_table(result_tree);

        assert_eq!(result_prefix, expected_prefix);
    }

    /// These tests for decoding data assume that we've consumed and read
    /// the size of the data already, which are the first 4 bytes after the
    /// tree header data.
    #[test]
    fn test_decode_data_one_distinct_char() {
        let input: Vec<u8> = vec![0b00000000, 0b00000011];
        let expected = "aaa";

        let mut freq = Freq::new();
        freq.update(expected.as_bytes());
        let root = generate_tree(&freq);
        let prefix_table = generate_prefix_table(root);

        let result = decode_data(&input, prefix_table);

        assert_eq!(expected.chars().collect::<Vec<char>>(), result);
    }

    #[test]
    fn test_decode_data_two_distinct_chars() {
        let input: Vec<u8> = vec![0b11000000, 0b00000011];

        let expected = "aab";

        let mut freq = Freq::new();
        freq.update(expected.as_bytes());
        let root = generate_tree(&freq);
        let prefix_table = generate_prefix_table(root);

        let result = decode_data(&input, prefix_table);

        assert_eq!(expected.chars().collect::<Vec<char>>(), result);
    }

    #[test]
    fn test_decode_data_three_distinct_chars() {
        let input: Vec<u8> = vec![0b11111110, 0b00000000, 0b00000100];

        let expected = "aaabcccc";

        let mut freq = Freq::new();
        freq.update(expected.as_bytes());
        let root = generate_tree(&freq);
        let prefix_table = generate_prefix_table(root);

        let result = decode_data(&input, prefix_table);

        assert_eq!(expected.chars().collect::<Vec<char>>(), result);
    }

    #[test]
    fn test_decode_data_no_distinct_chars() {
        let input: Vec<u8> = vec![0b00011011, 0b00000000];

        let expected = "abcd";

        let mut freq = Freq::new();
        freq.update(expected.as_bytes());
        let root = generate_tree(&freq);
        let prefix_table = generate_prefix_table(root);

        let result = decode_data(&input, prefix_table);

        assert_eq!(expected.chars().collect::<Vec<char>>(), result);
    }
    #[test]
    fn test_encode_decode_small_string() {
        let mut freq = Freq::new();
        let test_input = "aaabccccDJEisérables
Com";
        freq.update(test_input.as_bytes());
        let root = generate_tree(&freq);
        let prefix_table = generate_prefix_table(root);
        let test_file = Cursor::new(test_input.as_bytes());
        let expected_size = 3;
        let expected = vec![0b11111110, 0b00000000, 0b00000100];

        let (data_size, encoded_data) =
            get_encoded_data_with_header(test_file, prefix_table.clone());

        let rez = decode_data(&encoded_data, prefix_table);

        assert_eq!(rez, test_input.chars().collect::<Vec<char>>());
    }

    #[test]
    fn test_decode_sample_string() {
        let input: Vec<u8> = vec![
            0b00101010, 0b11110100, 0b10111101, 0b10011100, 0b11001011, 0b10111010, 0b00111000,
            0b10111101, 0b10100111, 0b10110101, 0b11000011, 0b10101111, 0b01000011, 0b11001111,
            0b01011100, 0b11111011, 0b11101001, 0b11000101, 0b10100011, 0b11011111, 0b01001110,
            0b00100011, 0b11010110, 0b10101111, 0b00010010, 0b01100001, 0b11010111, 0b10101001,
            0b10010000, 0b10000011, 0b00110110, 0b11100101, 0b00011111, 0b00001111, 0b11011011,
            0b11101000, 0b01101001, 0b00101001, 0b01011111, 0b10000110, 0b10110001, 0b11010110,
            0b10111010, 0b10011111, 0b11101010, 0b00111101, 0b10101110, 0b01111100, 0b00111010,
            0b11110100, 0b01101101, 0b01101011, 0b00110110, 0b11111000, 0b11001001, 0b01111000,
            0b11110111, 0b11100001, 0b10111110, 0b10000110, 0b11010001, 0b00100001, 0b11011101,
            0b11111010, 0b11101001, 0b01011111, 0b10000110, 0b01001011, 0b11001010, 0b11111100,
            0b00010100, 0b10001110, 0b00001010, 0b11010011, 0b11010010, 0b01000111, 0b10111111,
            0b00011110, 0b10110111, 0b01000001, 0b10101111, 0b00111100, 0b01010110, 0b10111000,
            0b01101010, 0b01011111, 0b11100011, 0b00011110, 0b11101001, 0b11111000, 0b11000100,
            0b00111001, 0b00110100, 0b10100101, 0b01000001, 0b11100010, 0b00011011, 0b11110001,
            0b11111111, 0b00011010, 0b11010111, 0b00101011, 0b00101000, 0b01000011, 0b11001111,
            0b00010000, 0b11010000, 0b01000011, 0b00110101, 0b11000011, 0b10101111, 0b00000110,
            0b10110100, 0b10111101, 0b00100101, 0b11001111, 0b10000111, 0b01011110, 0b00101010,
            0b00101101, 0b10010101, 0b11011001, 0b11000110, 0b00101000, 0b11000000, 0b00110100,
            0b10011010, 0b11010110, 0b01011100, 0b01010011, 0b00100011, 0b10110100, 0b11110011,
            0b11000100, 0b10000111, 0b10101110, 0b00000110, 0b01100110, 0b11010001, 0b00100001,
            0b11011100, 0b00111010, 0b01011110, 0b11001110, 0b01100101, 0b11011101, 0b00011101,
            0b01101011, 0b10101101, 0b00101011, 0b00100100, 0b01111011, 0b11100010, 0b01001000,
            0b11000110, 0b00111100, 0b11100101, 0b10000000, 0b01101001, 0b00110101, 0b10101100,
            0b10111100, 0b11101101, 0b01100101, 0b11100111, 0b10001010, 0b01010011, 0b11101110,
            0b00101110, 0b00011011, 0b11101010, 0b11110010, 0b01011000, 0b11010101, 0b11011001,
            0b11111110, 0b00011001, 0b10110001, 0b00100110, 0b00011101, 0b01111010, 0b10011001,
            0b00001000, 0b00110011, 0b01101110, 0b01010001, 0b11110000, 0b11111101, 0b11001001,
            0b10111000, 0b10111000, 0b01001001, 0b00010010, 0b10101110, 0b10111101, 0b11011111,
            0b11010000, 0b01111000, 0b01011110, 0b00111111, 0b01011001, 0b11101000, 0b11100001,
            0b11010111, 0b10101011, 0b11111100, 0b01111101, 0b10101110, 0b01111100, 0b00111010,
            0b11110001, 0b11101110, 0b00001000, 0b00010111, 0b10001101, 0b00011110, 0b10110101,
            0b01111011, 0b10001011, 0b10000110, 0b11111010, 0b10111101, 0b01011101, 0b10011111,
            0b11100001, 0b10011011, 0b01001101, 0b01110011, 0b11011010, 0b10111001, 0b00100001,
            0b11100010, 0b01001001, 0b01110000, 0b11101001, 0b01111011, 0b00111001, 0b10010111,
            0b01110100, 0b01111001, 0b10000000, 0b00000001,
        ];
        let expected = test_cases::SAMPLE_TEST;
        let mut freq = Freq::new();
        freq.update(expected.as_bytes());
        let root = generate_tree(&freq);
        let prefix_table = generate_prefix_table(root);

        let result = decode_data(&input, prefix_table);

        assert_eq!(expected.chars().collect::<Vec<char>>(), result);
    }

    #[test]
    fn test_encode_decode_bigger_string() {
        let mut freq = Freq::new();
        let test_input = test_cases::SAMPLE_TEST;
        freq.update(test_input.as_bytes());
        let root = generate_tree(&freq);
        let prefix_table = generate_prefix_table(root);
        let test_file = Cursor::new(test_input.as_bytes());

        let (data_size, encoded_data) =
            get_encoded_data_with_header(test_file, prefix_table.clone());

        let rez = decode_data(&encoded_data, prefix_table);

        assert_eq!(rez, test_input.chars().collect::<Vec<char>>());
    }
}
