use std::collections::HashMap;

pub struct Freq {
    pub counter: HashMap<char, usize>,
}

impl Freq {
    pub fn new() -> Freq {
        Freq {
            counter: HashMap::new(),
        }
    }

    pub fn update(&mut self, chunk: &[u8]) {
        for &byte in chunk {
            if let Some(ch) = char::from_u32(byte as u32) {
                self.counter
                    .entry(ch)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let mut freq = Freq::new();
        let test_input = "Hello".as_bytes();

        freq.update(test_input);

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

        freq.update(test_input);

        assert!(freq.counter.is_empty());
    }
}
