/* src/bits.rs v2 */

pub struct BitReader<'a> {
    pub buffer: &'a [u8],
    pub byte_pos: usize,
    pub bit_pos: u8,
    pub total_bits: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(buffer: &'a [u8], total_bits: usize) -> Self {
        Self {
            buffer,
            byte_pos: 0,
            bit_pos: 0,
            total_bits,
        }
    }

    pub fn remaining_bits(&self) -> usize {
        let bits_read = (self.byte_pos * 8) + self.bit_pos as usize;
        // Clippy fix: Use saturating_sub instead of manual check
        self.total_bits.saturating_sub(bits_read)
    }

    pub fn read_bit(&mut self) -> Option<u8> {
        if self.remaining_bits() == 0 {
            return None;
        }

        let byte = self.buffer.get(self.byte_pos)?;
        let bit = (byte >> (7 - self.bit_pos)) & 1;

        self.bit_pos += 1;
        if self.bit_pos == 8 {
            self.bit_pos = 0;
            self.byte_pos += 1;
        }

        Some(bit)
    }
}

pub struct BitWriter {
    pub buffer: Vec<u8>,
    current_byte: u8,
    bit_pos: u8,
}

// Clippy fix: Implement Default since we have a parameterless new()
impl Default for BitWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl BitWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            current_byte: 0,
            bit_pos: 0,
        }
    }

    pub fn write_bit(&mut self, bit: u8) {
        if bit != 0 {
            self.current_byte |= 1 << (7 - self.bit_pos);
        }
        self.bit_pos += 1;
        if self.bit_pos == 8 {
            self.buffer.push(self.current_byte);
            self.current_byte = 0;
            self.bit_pos = 0;
        }
    }

    pub fn flush(&mut self) {
        if self.bit_pos > 0 {
            self.buffer.push(self.current_byte);
            self.current_byte = 0;
            self.bit_pos = 0;
        }
    }
}

/* src/bits.rs v2 */
