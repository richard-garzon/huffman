use std::fs::File;
use std::io::{BufReader, Read};

use std::collections::HashMap;

pub struct Freq {
    pub counter: HashMap<char, u32>,
    pub incomplete: Vec<u8>,
}

impl Freq {
    pub fn new() -> Freq {
        Freq {
            counter: HashMap::new(),
            incomplete: Vec::new(),
        }
    }

    pub fn update(&mut self, chunk: &[u8]) {
        let mut data = self.incomplete.clone();
        data.extend_from_slice(&chunk);

        let (valid, incomplete) = match std::str::from_utf8(&data) {
            Ok(valid_str) => (valid_str, &[] as &[u8]),
            Err(e) => {
                let valid_up_to = e.valid_up_to();
                let valid = &data[..valid_up_to];
                let incomplete = &data[valid_up_to..];
                (std::str::from_utf8(valid).unwrap(), incomplete)
            }
        };

        self.incomplete = incomplete.to_vec();

        for ch in valid.chars() {
            self.counter
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    pub fn count_chars(&mut self, file: &File) {
        let mut reader = BufReader::new(file);
        let mut buffer = [0; 1024];

        while let Ok(bytes_read) = reader.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }

            self.update(&buffer[..bytes_read]);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::encoding::test_cases;

    use super::*;

    #[test]
    fn test_string() {
        let mut freq = Freq::new();
        let test_input = "Hello".as_bytes();

        freq.update(&test_input);

        assert_eq!(freq.counter.get(&'H').unwrap(), &1);
        assert_eq!(freq.counter.get(&'e').unwrap(), &1);
        assert_eq!(freq.counter.get(&'l').unwrap(), &2);
        assert_eq!(freq.counter.get(&'o').unwrap(), &1);
        assert_eq!(freq.counter.len(), 4);
    }

    #[test]
    fn test_empty_string() {
        let mut freq = Freq::new();
        let test_input = "".as_bytes();

        freq.update(&test_input);

        assert!(freq.counter.is_empty());
    }

    #[test]
    fn test_unicode_string() {
        let mut freq = Freq::new();
        let test_input = "привет".as_bytes();

        freq.update(&test_input);

        assert_eq!(freq.counter.get(&'п').unwrap(), &1);
        assert_eq!(freq.counter.get(&'р').unwrap(), &1);
        assert_eq!(freq.counter.get(&'и').unwrap(), &1);
        assert_eq!(freq.counter.get(&'в').unwrap(), &1);
        assert_eq!(freq.counter.get(&'е').unwrap(), &1);
        assert_eq!(freq.counter.get(&'т').unwrap(), &1);
    }

    #[test]
    fn test_bigger_than_chunk_size_string() {
        let mut freq = Freq::new();
        let test_input = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabb".as_bytes();

        freq.update(&test_input);

        assert_eq!(freq.counter.get(&'a').unwrap(), &4094);
        assert_eq!(freq.counter.get(&'b').unwrap(), &2);
    }

    #[test]
    fn test_bigger_than_chunk_size_unicode_string() {
        let mut freq = Freq::new();
        let test_input = "иииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииииипппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппппп".as_bytes();

        freq.update(&test_input);

        assert_eq!(freq.counter.get(&'и').unwrap(), &1024);
        assert_eq!(freq.counter.get(&'п').unwrap(), &1024);
    }

    #[test]
    fn test_freq_sample_paragraph() {
        let mut freq = Freq::new();
        let test_input = test_cases::SAMPLE_TEST;
        let expected_counter_state: HashMap<char, u32> = HashMap::from([
            (' ', 75),
            ('e', 47),
            ('t', 40),
            ('o', 37),
            ('r', 22),
            ('n', 22),
            ('i', 21),
            ('a', 20),
            ('h', 19),
            ('s', 19),
            ('u', 12),
            ('w', 12),
            ('c', 11),
            ('d', 10),
            ('y', 9),
            ('l', 9),
            ('f', 7),
            ('\n', 6),
            ('g', 6),
            ('.', 5),
            ('k', 4),
            ('m', 4),
            ('B', 3),
            ('v', 3),
            ('b', 3),
            ('U', 2),
            ('S', 2),
            ('p', 2),
            (',', 2),
            ('T', 1),
            ('Y', 1),
            ('-', 1),
            ('P', 1),
            ('j', 1),
            ('G', 1),
            ('L', 1),
            ('I', 1),
        ]);

        freq.update(&test_input.as_bytes());

        assert_eq!(expected_counter_state, freq.counter);
    }
}
