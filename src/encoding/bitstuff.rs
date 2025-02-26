use std::io::{self, Write};

pub struct BitWriter {
    buf: Vec<u8>,
    current_byte: u8,
    bit_position: u8,
}

impl BitWriter {
    pub fn new() -> Self {
        BitWriter {
            buf: Vec::new(),
            current_byte: 0,
            bit_position: 0,
        }
    }

    pub fn write_bit(&mut self, bit: u8) {
        if bit > 1 {
            panic!("Invalid bit val");
        }

        self.current_byte = (self.current_byte << 1) | bit;
        self.bit_position += 1;

        if self.bit_position == 8 {
            self.buf.push(self.current_byte);
            self.current_byte = 0;
            self.bit_position = 0;
        }
    }

    pub fn write_bits(&mut self, bits: u32, num_bits: u8) {
        for i in (0..num_bits).rev() {
            let bit = ((bits >> i) & 1) as u8;
            self.write_bit(bit);
        }
    }

    pub fn flush(&mut self) {
        if self.bit_position > 0 {
            self.current_byte <<= 8 - self.bit_position;
            self.buf.push(self.current_byte);
            self.current_byte = 0;
            self.bit_position = 0;
        }
    }

    pub fn get_vec(mut self) -> io::Result<Vec<u8>> {
        self.flush();
        Ok(self.buf)
    }

    pub fn get_current_pos(&self) -> u8 {
        self.bit_position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_a_bit() {
        let mut bw = BitWriter::new();

        bw.write_bit(1);

        assert_eq!(bw.get_vec().unwrap(), vec![0b10000000]);
    }

    #[test]
    fn test_write_bits() {
        let mut bw = BitWriter::new();

        bw.write_bits(0xDEADBEEF, 32);

        assert_eq!(bw.get_vec().unwrap(), vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn test_flush() {
        let mut bw = BitWriter::new();

        bw.write_bit(0);
        bw.flush();
        bw.write_bit(1);

        assert_eq!(bw.get_vec().unwrap(), vec![0, 128]);
    }
}
