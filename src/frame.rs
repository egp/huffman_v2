// src/frame.rs v2

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    Header = 0,
    FrequencyTableInternal = 1,
    FrequencyTableExternal = 2,
    Payload = 3,
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

    Ok((t, payload))
}

// src/frame.rs v2
