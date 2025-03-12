pub struct BitReader {
    data: Vec<u8>,
    current_byte: usize,
    bit_position: u8,
}

impl BitReader {
    pub fn new(data: Vec<u8>) -> Self {
        BitReader {
            data,
            current_byte: 0,
            bit_position: 0,
        }
    }
}

impl Iterator for BitReader {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_byte >= self.data.len() {
            return None;
        }

        let byte = self.data[self.current_byte];
        let bit = byte >> (7 - self.bit_position) & 1;

        if self.bit_position == 7 {
            self.bit_position = 0;
            self.current_byte += 1;
        } else {
            self.bit_position += 1;
        }

        return Some(bit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_one_byte() {
        let input = vec![0b10000000];
        let mut br = BitReader::new(input);

        let result = br.next();

        assert_eq!(1u8, result.unwrap());
    }

    #[test]
    fn test_read_three_bytes() {
        let input = vec![0b10100000];
        let mut br = BitReader::new(input);

        assert_eq!(1u8, br.next().unwrap());
        assert_eq!(0u8, br.next().unwrap());
        assert_eq!(1u8, br.next().unwrap());
    }
}
