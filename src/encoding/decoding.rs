use crate::encoding::frequency::Freq;
use crate::encoding::tree::generate_tree;
use std::collections::HashMap;

/// how to start decoding...
/// we should first read the header from the file and rebuild the huffman tree to decode
/// and then build the prefix table
/// we should go from <string of bits> -> prefix table
///
/// next we just decode the file and restore the data to its original state

pub fn decode_tree_header_with_size(tree_data: Vec<u8>) -> HashMap<char, u8> {
    let prefix_table: HashMap<char, u8> = HashMap::new();

    prefix_table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_tree_header_size() {
        let input: Vec<u8> = vec![0b00000000, 0b00000011];
        let expected = "aaa";
        let mut freq = Freq::new();
        freq.update(expected.as_bytes());
        let root = generate_tree(&freq);
    }
}
