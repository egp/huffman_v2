// src/huffman.rs v1

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub weight: u32,
    pub symbol: Option<u8>,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub is_leaf: bool,
}

pub fn build_tree(freq: &[u32; 256]) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();

    // 1. Create leaf nodes
    for (i, &f) in freq.iter().enumerate() {
        if f > 0 {
            nodes.push(Node {
                weight: f,
                symbol: Some(i as u8),
                left: None,
                right: None,
                is_leaf: true,
            });
        }
    }

    // Edge case: empty or single symbol
    if nodes.len() <= 1 {
        return nodes;
    }

    // 2. Active set of indices
    let mut active: Vec<usize> = (0..nodes.len()).collect();

    // 3. Deterministic Huffman construction
    while active.len() > 1 {
        active.sort_by(|&a, &b| {
            let na = &nodes[a];
            let nb = &nodes[b];

            na.weight
                .cmp(&nb.weight)
                .then_with(|| na.symbol.cmp(&nb.symbol))
                .then_with(|| a.cmp(&b)) // final deterministic tie-breaker
        });

        let a = active.remove(0);
        let b = active.remove(0);

        let parent = Node {
            weight: nodes[a].weight + nodes[b].weight,
            symbol: None,
            left: Some(a),
            right: Some(b),
            is_leaf: false,
        };

        nodes.push(parent);
        let parent_idx = nodes.len() - 1;

        active.push(parent_idx);
    }

    nodes
}

pub fn build_codes(tree: &Vec<Node>) -> HashMap<u8, Vec<u8>> {
    let mut codes = HashMap::new();

    if tree.is_empty() {
        return codes;
    }

    let root_index = tree.len() - 1;

    fn walk(nodes: &Vec<Node>, idx: usize, path: &mut Vec<u8>, out: &mut HashMap<u8, Vec<u8>>) {
        let node = &nodes[idx];

        if node.is_leaf {
            if let Some(sym) = node.symbol {
                out.insert(sym, path.clone());
            }
            return;
        }

        if let Some(left) = node.left {
            path.push(0);
            walk(nodes, left, path, out);
            path.pop();
        }

        if let Some(right) = node.right {
            path.push(1);
            walk(nodes, right, path, out);
            path.pop();
        }
    }

    walk(tree, root_index, &mut Vec::new(), &mut codes);

    codes
}

#[derive(Default)]
pub struct BitWriter {
    pub buffer: Vec<u8>,
    current: u8,
    pos: u8,
}

impl BitWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            current: 0,
            pos: 0,
        }
    }

    pub fn write_bit(&mut self, bit: u8) {
        if bit != 0 {
            self.current |= 1 << (7 - self.pos);
        }

        self.pos += 1;

        if self.pos == 8 {
            self.flush();
        }
    }

    fn flush(&mut self) {
        self.buffer.push(self.current);
        self.current = 0;
        self.pos = 0;
    }

    pub fn finish(mut self) -> Vec<u8> {
        let mut out = Vec::new();

        let bit_len = (self.buffer.len() as u32 * 8) + self.pos as u32;

        out.extend_from_slice(&bit_len.to_le_bytes());

        if self.pos > 0 {
            self.flush();
        }

        out.extend_from_slice(&self.buffer);

        out
    }
}

pub struct BitReader<'a> {
    data: &'a [u8],
    byte_pos: usize,
    bit_pos: u8,
    remaining_bits: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let mut len_bytes = [0u8; 4];
        len_bytes.copy_from_slice(&data[0..4]);
        let bit_len = u32::from_le_bytes(len_bytes) as usize;

        Self {
            data: &data[4..],
            byte_pos: 0,
            bit_pos: 0,
            remaining_bits: bit_len,
        }
    }

    pub fn next_bit(&mut self) -> Option<u8> {
        if self.remaining_bits == 0 {
            return None;
        }

        if self.byte_pos >= self.data.len() {
            return None;
        }

        let byte = self.data[self.byte_pos];
        let bit = (byte >> (7 - self.bit_pos)) & 1;

        self.bit_pos += 1;
        self.remaining_bits -= 1;

        if self.bit_pos == 8 {
            self.bit_pos = 0;
            self.byte_pos += 1;
        }

        Some(bit)
    }
}

pub fn encode_stream(input: &[u8], codes: &std::collections::HashMap<u8, Vec<u8>>) -> Vec<u8> {
    let mut writer = BitWriter::new();

    for &symbol in input {
        if let Some(code) = codes.get(&symbol) {
            for &bit in code {
                writer.write_bit(bit);
            }
        }
    }

    writer.finish()
}

pub fn decode_stream(encoded: &[u8], tree: &[Node], expected_len: usize) -> Vec<u8> {
    let mut output = Vec::with_capacity(expected_len);

    if tree.is_empty() {
        return output;
    }

    let mut reader = BitReader::new(encoded);
    let mut current = tree.len() - 1;

    while output.len() < expected_len {
        let bit = match reader.next_bit() {
            Some(b) => b,
            None => break,
        };

        let node = &tree[current];

        let next = if bit == 0 { node.left } else { node.right };

        let next = match next {
            Some(n) => n,
            None => {
                // IMPORTANT: we are in invalid traversal state
                break;
            }
        };

        current = next;

        let node = &tree[current];

        if node.is_leaf {
            if let Some(sym) = node.symbol {
                output.push(sym);
            }
            current = tree.len() - 1;
        }
    }

    output
}

pub fn build_frequency_table(input: &[u8]) -> [u32; 256] {
    let mut freq = [0u32; 256];
    for &b in input {
        freq[b as usize] += 1;
    }
    freq
}

// src/huffman.rs v1
