// src/header.rs v5
use crate::checksum::checksum32;

pub const MAGIC: &[u8; 4] = b"HUF1";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Header {
    pub flags: u8,
    pub original_size: u64,
    pub payload_size: u64,
}

// src/header.rs v5