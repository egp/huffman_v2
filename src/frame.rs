// src/frame.rs v2

pub const HEADER_SIZE: usize = 26;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    Header = 0,
    FrequencyTableInternal = 1,
    FrequencyTableExternal = 2,
    Payload = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub flags: u8,
    pub original_size: u64,
    pub payload_size: u64,
}

pub fn pack(frame_type: FrameType, payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(5 + payload.len());

    out.push(frame_type as u8);

    let len = (payload.len() as u32).to_le_bytes();
    out.extend_from_slice(&len);

    out.extend_from_slice(payload);

    out
}

pub fn unpack(frame: &[u8]) -> Result<(FrameType, Vec<u8>), String> {
    if frame.len() < 5 {
        return Err("frame too small".into());
    }

    let t = match frame[0] {
        0 => FrameType::Header,
        1 => FrameType::FrequencyTableInternal,
        2 => FrameType::FrequencyTableExternal,
        3 => FrameType::Payload,
        _ => return Err("invalid frame type".into()),
    };

    let mut len_bytes = [0u8; 4];
    len_bytes.copy_from_slice(&frame[1..5]);
    let len = u32::from_le_bytes(len_bytes) as usize;

    if frame.len() < 5 + len {
        return Err("frame length mismatch".into());
    }

    let payload = frame[5..5 + len].to_vec();

    if t == FrameType::FrequencyTableInternal && payload.len() != 1024 {
        return Err("invalid frequency table size".into());
    }

    Ok((t, payload))
}

pub fn serialize_header(h: &Header) -> Vec<u8> {
    let mut bytes = vec![0u8; HEADER_SIZE];

    // 0–3: magic
    bytes[0..4].copy_from_slice(b"HUF1");

    // 4: version
    bytes[4] = 1;

    // 5: flags
    bytes[5] = h.flags;

    // 6–9: checksum placeholder (0 for computation pass)
    bytes[6..10].fill(0);

    // 10–17: original_size
    bytes[10..18].copy_from_slice(&h.original_size.to_le_bytes());

    // 18–25: payload_size
    bytes[18..26].copy_from_slice(&h.payload_size.to_le_bytes());

    // compute checksum over zeroed checksum field
    let checksum = crate::checksum32(&bytes);

    bytes[6..10].copy_from_slice(&checksum.to_le_bytes());

    bytes
}

pub fn parse_header(bytes: &[u8]) -> Result<Header, String> {
    if bytes.len() != HEADER_SIZE {
        return Err("invalid header size".into());
    }

    if &bytes[0..4] != b"HUF1" {
        return Err("bad magic".into());
    }

    if bytes[4] != 1 {
        return Err("unsupported version".into());
    }

    let flags = bytes[5];

    let mut checksum_bytes = [0u8; 4];
    checksum_bytes.copy_from_slice(&bytes[6..10]);
    let checksum = u32::from_le_bytes(checksum_bytes);

    let mut tmp = bytes.to_vec();
    tmp[6..10].fill(0);

    if checksum != crate::checksum32(&tmp) {
        return Err("checksum mismatch".into());
    }

    let mut orig = [0u8; 8];
    orig.copy_from_slice(&bytes[10..18]);

    let mut payload = [0u8; 8];
    payload.copy_from_slice(&bytes[18..26]);

    Ok(Header {
        flags,
        original_size: u64::from_le_bytes(orig),
        payload_size: u64::from_le_bytes(payload),
    })
}

// src/frame.rs v2
