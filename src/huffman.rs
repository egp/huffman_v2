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

    if nodes.len() <= 1 {
        return nodes;
    }

    let mut active: Vec<usize> = (0..nodes.len()).collect();

    while active.len() > 1 {
        active.sort_by(|&a, &b| {
            nodes[a]
                .weight
                .cmp(&nodes[b].weight)
                .then(nodes[a].symbol.cmp(&nodes[b].symbol))
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

// src/huffman.rs v1
