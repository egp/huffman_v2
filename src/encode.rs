// src/encode.rs v3
use crate::huffman::{build_codes, build_frequency_table, build_tree, encode_stream};

pub fn encode(input: &[u8]) -> Vec<u8> {
    if input.is_empty() {
        return Vec::new();
    }

    let freq = build_frequency_table(input);
    let tree = build_tree(&freq);
    let codes = build_codes(&tree);

    let encoded = encode_stream(input, &codes);

    // PREPEND frequency table (temporary transport layer)
    let freq_bytes: Vec<u8> = freq.iter().flat_map(|v| v.to_le_bytes()).collect();

    let mut out = Vec::with_capacity(freq_bytes.len() + encoded.len());
    out.extend_from_slice(&freq_bytes);
    out.extend_from_slice(&encoded);

    out
}

// src/encode.rs v3
