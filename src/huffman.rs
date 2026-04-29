/* src/huffman.rs v7 */
use crate::bits::BitReader;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Node {
    Leaf {
        symbol: u8,
        freq: u64,
    },
    Internal {
        left: Box<Node>,
        right: Box<Node>,
        freq: u64,
    },
}

impl Node {
    pub fn freq(&self) -> u64 {
        match self {
            Node::Leaf { freq, .. } => *freq,
            Node::Internal { freq, .. } => *freq,
        }
    }
}

pub fn build_frequency_table(_data: &[u8]) -> HashMap<u8, u64> {
    HashMap::new()
}

pub fn build_tree(_freqs: &HashMap<u8, u64>) -> (Node, Node) {
    let dummy = Node::Leaf { symbol: 0, freq: 0 };
    (dummy.clone(), dummy)
}

pub fn build_codes(_root: &Node) -> HashMap<u8, String> {
    HashMap::new()
}

pub fn encode_stream(_data: &[u8], _codes: &HashMap<u8, String>) -> Vec<u8> {
    Vec::new()
}

pub fn decode_stream(_reader: &mut BitReader, _root: &Node) -> Vec<u8> {
    Vec::new()
}
/* src/huffman.rs v7 */
