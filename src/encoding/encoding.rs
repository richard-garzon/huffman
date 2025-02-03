use super::{
    bitstuff::BitWriter,
    frequency::{self, Freq},
    tree::{generate_tree, HuffNode},
};
use std::io::Result;
use std::{collections::HashMap, fs::File};

pub fn get_prefixes(node: &Option<Box<HuffNode>>, state: &u8, prefix: &mut HashMap<char, u8>) {
    if node.is_none() {
        return;
    }

    let curr_node = node.as_ref().unwrap();

    if let Some(character) = curr_node.character {
        prefix.insert(character, state.clone());
    } else {
        let mut left_state = state.clone() << 1;
        left_state |= 0;
        get_prefixes(&curr_node.left, &left_state, prefix);

        let mut right_state = state.clone() << 1;
        right_state |= 1;
        get_prefixes(&curr_node.right, &right_state, prefix);
    }
}

pub fn generate_prefix_table(node: Option<Box<HuffNode>>) -> HashMap<char, u8> {
    let mut prefix_table = HashMap::new();
    let mut state: u8 = 0;

    get_prefixes(&node, &state, &mut prefix_table);

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

pub fn encode_data(compressed_file: File) -> Result<()> {
    Ok(())
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

        assert_eq!(prefix_table.get(&'a').unwrap(), &0);
        for (key, value) in prefix_table.into_iter() {
            println!("{} -> {}", key, value);
        }
    }

    #[test]
    fn test_two_letter_prefix_table() {
        let mut freq = Freq::new();
        let test_input = "aaab".as_bytes();
        freq.update(test_input);
        let root = generate_tree(&freq);

        let prefix_table = generate_prefix_table(root);

        assert_eq!(prefix_table.get(&'a').unwrap(), &1);
        assert_eq!(prefix_table.get(&'b').unwrap(), &0);
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
        assert_eq!(prefix_table.get(&'a').unwrap(), &3);
        assert_eq!(prefix_table.get(&'b').unwrap(), &2);
        assert_eq!(prefix_table.get(&'c').unwrap(), &0);
    }

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
}
