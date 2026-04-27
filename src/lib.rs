pub mod checksum;
pub mod frame;
pub mod header;

// Core exports (test-facing API)
pub use checksum::checksum32;
pub use frame::FrameType;
pub use header::{parse_header, serialize_header, Header, HEADER_SIZE};

pub fn encode(input: &[u8], _passphrase: Option<&str>) -> Vec<u8> {
    if input.is_empty() {
        return Vec::new();
    }

    // temporary deterministic behavior:
    // just echo input for now (identity transform)
    input.to_vec()
}

pub fn decode(input: &[u8], _passphrase: Option<&str>) -> Vec<u8> {
    if input.is_empty() {
        return Vec::new();
    }

    // inverse of current stub encode
    input.to_vec()
}

pub fn build_frequency_table(input: &[u8]) -> [u32; 256] {
    let mut table = [0u32; 256];

    for &b in input {
        table[b as usize] += 1;
    }

    table
}
