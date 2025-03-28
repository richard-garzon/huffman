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

    pub fn get_current_byte(&self) -> usize {
        return self.current_byte;
    }

    pub fn read_bits(&mut self, num_bits: u32) -> Vec<u8> {
        if (self.data.len() * 8) < num_bits as usize {
            panic!("You tried to read more b its than exist in this BitReader")
        }

        let mut bits = Vec::new();
        let mut current_byte = 0u8;
        let mut bits_collected = 0;

        for _ in 0..num_bits {
            if let Some(bit) = self.next() {
                current_byte = (current_byte << 1) | bit;
                bits_collected += 1;

                if bits_collected == 8 {
                    bits.push(current_byte);
                    current_byte = 0;
                    bits_collected = 0;
                }
            } else {
                break;
            }
        }

        if bits_collected > 0 {
            current_byte <<= 8 - bits_collected;
            bits.push(current_byte);
        }
        bits
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
    fn test_read_one_bit() {
        let input = vec![0b10000000];
        let mut br = BitReader::new(input);

        let result = br.next();

        assert_eq!(1u8, result.unwrap());
    }

    #[test]
    fn test_read_three_bits() {
        let input = vec![0b10100000];
        let mut br = BitReader::new(input);

        assert_eq!(1u8, br.next().unwrap());
        assert_eq!(0u8, br.next().unwrap());
        assert_eq!(1u8, br.next().unwrap());
    }

    #[test]
    fn test_read_two_bytes() {
        let input = vec![0b10101010, 0b10001000];
        let mut br = BitReader::new(input.clone());

        let char_bits = br.read_bits(16);

        assert_eq!(input, char_bits);
    }
}
