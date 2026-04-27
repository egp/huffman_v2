// src/checksum.rs V1
pub fn checksum32(data: &[u8]) -> u32 {
    let mut hash: u32 = 2166136261;

    for b in data {
        hash ^= *b as u32;
        hash = hash.wrapping_mul(16777619);
    }

    hash
}
// src/checksum.rs V1
