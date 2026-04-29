/* src/header.rs v5 */
use crate::checksum::checksum32;

pub const MAGIC_BYTES: &[u8; 4] = b"HUFF";

#[derive(Debug, PartialEq, Clone)]
pub struct Header {
    pub version: u8,
    pub flags: u8,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            version: 1,
            flags: 0,
        }
    }
}

impl Header {
    pub fn new(version: u8, flags: u8) -> Self {
        Self { version, flags }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut tmp = MAGIC_BYTES.to_vec();
        tmp.push(self.version);
        tmp.push(self.flags);

        let c = checksum32(&tmp);
        let mut bytes = tmp;
        bytes.extend_from_slice(&c.to_be_bytes());
        bytes
    }
}

pub fn validate_header(data: &[u8]) -> Result<Header, &'static str> {
    if data.len() < 10 {
        return Err("Header too short");
    }

    if &data[0..4] != MAGIC_BYTES {
        return Err("Invalid magic bytes");
    }

    let tmp = &data[0..6];
    let actual = checksum32(tmp);

    let mut check_bytes = [0u8; 4];
    check_bytes.copy_from_slice(&data[6..10]);
    let expected = u32::from_be_bytes(check_bytes);

    if actual != expected {
        return Err("Header checksum mismatch");
    }

    Ok(Header {
        version: data[4],
        flags: data[5],
    })
}
/* src/header.rs v5 */
