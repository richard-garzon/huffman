use std::fs::File;
use std::io::{BufReader, Read};

use std::collections::HashMap;

pub struct Freq {
    pub counter: HashMap<char, usize>,
    pub incomplete: Vec<u8>,
}

impl Freq {
    pub fn new() -> Freq {
        Freq {
            counter: HashMap::new(),
            incomplete: Vec::new(),
        }
    }

    pub fn update(&mut self, chunk: &str) {
        for ch in chunk.chars() {
            self.counter
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    pub fn count_chars(&mut self, file: File) {
        let mut reader = BufReader::new(file);
        let mut buffer = [0; 1024];

        while let Ok(bytes_read) = reader.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }

            let mut data = self.incomplete.clone();
            data.extend_from_slice(&buffer);

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

            self.update(valid);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let mut freq = Freq::new();
        let test_input = "Hello";

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
        let test_input = "";

        freq.update(&test_input);

        assert!(freq.counter.is_empty());
    }

    #[test]
    fn test_unicode_string() {
        let mut freq = Freq::new();
        let test_input = "привет";

        freq.update(&test_input);

        assert_eq!(freq.counter.get(&'п').unwrap(), &1);
        assert_eq!(freq.counter.get(&'р').unwrap(), &1);
        assert_eq!(freq.counter.get(&'и').unwrap(), &1);
        assert_eq!(freq.counter.get(&'в').unwrap(), &1);
        assert_eq!(freq.counter.get(&'е').unwrap(), &1);
        assert_eq!(freq.counter.get(&'т').unwrap(), &1);
    }
}
