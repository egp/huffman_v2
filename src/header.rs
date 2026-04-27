use crate::checksum::checksum32;

pub const MAGIC: &[u8; 4] = b"HUF1";
pub const HEADER_SIZE: usize = 32;

// Layout:
// 0..4   magic
// 4      format
// 5      version
// 6      flags
// 7      reserved
// 8..16  original_size
// 16..24 payload_size
// 24..28 checksum
// 28..32 reserved/future

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Header {
    pub flags: u8,
    pub original_size: u64,
    pub payload_size: u64,
}

pub fn serialize_header(h: &Header) -> Vec<u8> {
    let mut buf = vec![0u8; HEADER_SIZE];

    buf[0..4].copy_from_slice(MAGIC);
    buf[4] = 1; // format
    buf[5] = 1; // version
    buf[6] = h.flags;

    buf[8..16].copy_from_slice(&h.original_size.to_le_bytes());
    buf[16..24].copy_from_slice(&h.payload_size.to_le_bytes());

    let checksum = checksum32(&buf[0..24]);
    buf[24..28].copy_from_slice(&checksum.to_le_bytes());

    buf
}

pub fn parse_header(data: &[u8]) -> Result<Header, String> {
    if data.len() != HEADER_SIZE {
        return Err("invalid header size".into());
    }

    if &data[0..4] != MAGIC {
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

    let mut chk_bytes = [0u8; 4];
    chk_bytes.copy_from_slice(&data[24..28]);

    let expected = u32::from_le_bytes(chk_bytes);
    let computed = checksum32(&data[0..24]);

    if expected != computed {
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
