// src/header.rs v7

pub const MAGIC: &[u8; 4] = b"HUF1";
pub const VERSION: u8 = 1;
pub const HEADER_SIZE: usize = 26;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Header {
    pub flags: u8,
    pub original_size: u64,
    pub payload_size: u64,
}

/// Wire format:
/// 0..4   magic "HUF1"
/// 4      version
/// 5      flags
/// 6..10  checksum (u32 LE)
/// 10..18 original_size (u64 LE)
/// 18..26 payload_size (u64 LE)
pub fn serialize_header(h: &Header) -> Vec<u8> {
    eprintln!(">>> USING HEADER VERSION A");
    let mut buf = vec![0u8; HEADER_SIZE];

    buf[0..4].copy_from_slice(b"HUF1");
    buf[4] = 1; // version
    buf[5] = h.flags;

    buf[6..10].fill(0); // checksum placeholder

    buf[10..18].copy_from_slice(&h.original_size.to_le_bytes());
    buf[18..26].copy_from_slice(&h.payload_size.to_le_bytes());

    // IMPORTANT: checksum over canonical pre-image
    let mut tmp = buf.clone();
    tmp[6..10].fill(0);

    let c = crate::checksum32(&tmp);
    buf[6..10].copy_from_slice(&c.to_le_bytes());

    buf
}

pub fn parse_header(buf: &[u8]) -> Result<Header, String> {
    if buf.len() != HEADER_SIZE {
        return Err("invalid header size".into());
    }

    if &buf[0..4] != b"HUF1" {
        return Err("bad magic".into());
    }

    if buf[4] != 1 {
        return Err("unsupported version".into());
    }

    let mut tmp = buf.to_vec();
    tmp[6..10].fill(0);

    let expected = u32::from_le_bytes(buf[6..10].try_into().unwrap());
    let actual = crate::checksum32(&tmp);

    if expected != actual {
        return Err("checksum mismatch".into());
    }

    Ok(Header {
        flags: buf[5],
        original_size: u64::from_le_bytes(buf[10..18].try_into().unwrap()),
        payload_size: u64::from_le_bytes(buf[18..26].try_into().unwrap()),
    })
}

// src/header.rs v7
