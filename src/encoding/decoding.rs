use crate::encoding::frequency::Freq;
use crate::encoding::tree::generate_tree;
use std::collections::HashMap;

use super::tree::HuffNode;

/// how to start decoding...
/// we should first read the header from the file and rebuild the huffman tree to decode
/// and then build the prefix table
/// we should go from <string of bits> -> prefix table
///
/// next we just decode the file and restore the data to its original state

//pub fn decode_tree_header_with_size(tree_data: Vec<u8>) -> Option<Box<HuffNode>> {
// before implementing this, implement a bit iterator that makes it easy to
// move through these vecs of u8 without have to handle indices or nested loops
//}

#[cfg(test)]
mod tests {
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
    }
}
