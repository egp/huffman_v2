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

pub fn parse_header(data: &[u8]) -> Result<Header, String> {
    if data.len() != HEADER_SIZE {
        return Err("invalid header size".into());
    }

    if &data[0..4] != b"HUF1" {
        return Err("bad magic".into());
    }

    let format = data[4];
    let version = data[5];

    if format != 1 {
        return Err("unsupported format".into());
    }

    if version != 1 {
        return Err("unsupported version".into());
    }

    let expected_checksum = {
        let mut tmp = [0u8; 4];
        tmp.copy_from_slice(&data[24..28]);
        u32::from_le_bytes(tmp)
    };

    let computed_checksum = checksum32(&data[0..24]);

    if expected_checksum != computed_checksum {
        return Err("checksum mismatch".into());
    }

    let flags = data[6];

    let mut orig = [0u8; 8];
    orig.copy_from_slice(&data[8..16]);

    let mut payload = [0u8; 8];
    payload.copy_from_slice(&data[16..24]);

    Ok(Header {
        flags,
        original_size: u64::from_le_bytes(orig),
        payload_size: u64::from_le_bytes(payload),
    })
}

pub fn serialize_header(h: &Header) -> Vec<u8> {
    let mut buf = vec![0u8; HEADER_SIZE];

    buf[0..4].copy_from_slice(b"HUF1");
    buf[4] = 1;
    buf[5] = 1;
    buf[6] = h.flags;

    buf[8..16].copy_from_slice(&h.original_size.to_le_bytes());
    buf[16..24].copy_from_slice(&h.payload_size.to_le_bytes());

    // checksum computed over header WITHOUT checksum field
    let checksum = checksum32(&buf[0..24]);

    buf[24..28].copy_from_slice(&checksum.to_le_bytes());

    buf
}

// src/lib.rs v5
