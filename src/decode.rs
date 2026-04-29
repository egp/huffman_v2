/* src/decode.rs v4 */
use crate::bits::BitReader;
use crate::huffman::Node;

pub struct Decoder<'a> {
    reader: BitReader<'a>,
    root: Node,
}

impl<'a> Decoder<'a> {
    pub fn new(data: &'a [u8], total_bits: usize, root: Node) -> Self {
        Self {
            reader: BitReader::new(data, total_bits),
            root,
        }
    }

    pub fn decode_frame(&mut self) -> Result<Vec<u8>, &'static str> {
        let mut output = Vec::new();

        while self.reader.remaining_bits() > 0 {
            // We pass the reader as a mutable reference and the root as an
            // immutable reference to avoid borrowing conflict.
            if let Some(symbol) = Self::decode_symbol(&mut self.reader, &self.root) {
                output.push(symbol);
            } else {
                return Err("Failed to decode symbol: bitstream ended prematurely");
            }
        }

        Ok(output)
    }

    fn decode_symbol(reader: &mut BitReader, node: &Node) -> Option<u8> {
        match node {
            Node::Leaf { symbol, .. } => Some(*symbol),
            Node::Internal { left, right, .. } => {
                let bit = reader.read_bit()?;
                if bit == 0 {
                    Self::decode_symbol(reader, left)
                } else {
                    Self::decode_symbol(reader, right)
                }
            }
        }
    }
}
/* src/decode.rs v4 */
