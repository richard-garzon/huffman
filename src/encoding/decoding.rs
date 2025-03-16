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

pub fn invert_prefix_table(prefix_table: HashMap<char, (u8, u8)>) -> HashMap<u8, char> {
    let mut inverted_prefix_table: HashMap<u8, char> = HashMap::new();

    for (c, (prefix, d)) in prefix_table {
        inverted_prefix_table.insert(prefix, c);
    }

    inverted_prefix_table
}

pub fn decode_data(data: &Vec<u8>, prefix_table: HashMap<char, (u8, u8)>) -> Vec<char> {
    // number of bits to read from the encoded data
    let last_byte = data.last().unwrap();
    let data_length = data.len() - 2;

    let mut characters: Vec<char> = Vec::new();

    let mut br = BitReader::new(data.to_vec()); // yes i know this copies, i am lazy
    let inverted_prefix_table = invert_prefix_table(prefix_table);
    let mut curr_prefix = 0u8;

    while (br.get_current_byte() < data_length) {
        curr_prefix <<= br.next().unwrap() & 1;

        if inverted_prefix_table.contains_key(&curr_prefix) {
            characters.push(inverted_prefix_table.get(&curr_prefix).unwrap().clone());
            curr_prefix = 0u8;
        }
    }

    for _ in (0..*last_byte) {
        curr_prefix <= br.next().unwrap() & 1;
        if inverted_prefix_table.contains_key(&curr_prefix) {
            characters.push(inverted_prefix_table.get(&curr_prefix).unwrap().clone());
            curr_prefix = 0u8;
        }
    }

    characters
}

#[cfg(test)]
mod tests {
    use crate::encoding::encoding::generate_prefix_table;

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
}
