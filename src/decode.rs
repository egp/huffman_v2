// src/decode.rs v2
use crate::huffman::{build_tree, decode_stream};

pub fn decode(input: &[u8]) -> Vec<u8> {
    if input.is_empty() {
        return Vec::new();
    }

    let mut freq = [0u32; 256];
    freq[0] = 1;

    let tree = build_tree(&freq);

    decode_stream(input, &tree, input.len())
}

// src/decode.rs v2
