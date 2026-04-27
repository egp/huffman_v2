// src/lib.rs v5

pub const HEADER_SIZE: usize = 32;

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub flags: u8,
    pub original_size: u64,
    pub payload_size: u64,
}

pub fn encode(input: &[u8], _passphrase: Option<&[u8]>) -> Vec<u8> {
    input.to_vec()
}

pub fn decode(input: &[u8], _passphrase: Option<&[u8]>) -> Vec<u8> {
    input.to_vec()
}

pub fn build_frequency_table(input: &[u8]) -> [u32; 256] {
    let mut table = [0u32; 256];
    for &byte in input {
        table[byte as usize] += 1;
    }
    table
}

pub fn checksum32(data: &[u8]) -> u32 {
    const OFFSET_BASIS: u32 = 0x811c9dc5;
    const FNV_PRIME: u32 = 0x01000193;

    let mut hash = OFFSET_BASIS;

    for &byte in data {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    hash
}

// -------- Frame (STUBS) --------

pub fn serialize_header(_h: &Header) -> Vec<u8> {
    // STUB
    vec![0u8; HEADER_SIZE]
}

pub fn parse_header(_data: &[u8]) -> Result<Header, String> {
    // STUB
    Err("not implemented".into())
}

// src/lib.rs v5
