// src/decode.rs v2
use crate::huffman::{build_tree, decode_stream};

pub fn decode(input: &[u8]) -> Vec<u8> {
    if input.is_empty() {
        return Vec::new();
    }

    if input.len() < 1024 {
        return Vec::new();
    }

    let mut freq = [0u32; 256];

    for (i, chunk) in input.chunks(4).enumerate().take(256) {
        let bytes: [u8; 4] = chunk.try_into().unwrap();
        freq[i] = u32::from_le_bytes(bytes);
    }

    let tree = build_tree(&freq);

    let total_symbols: usize = freq.iter().map(|&f| f as usize).sum();

    // ✔ EDGE CASE: single-symbol Huffman tree
    if tree.len() == 1 {
        let symbol = tree[0].symbol.unwrap();
        return vec![symbol; total_symbols];
    }

    let payload = &input[1024..];

    decode_stream(payload, &tree, total_symbols)
}

// src/decode.rs v2
