// src/encode.rs v3
use crate::huffman::{build_codes, build_frequency_table, build_tree, encode_stream};

pub fn encode(input: &[u8]) -> Vec<u8> {
    if input.is_empty() {
        return Vec::new();
    }

    let freq = build_frequency_table(input);
    let tree = build_tree(&freq);
    let codes = build_codes(&tree);

    encode_stream(input, &codes)
}
// src/encode.rs v3
