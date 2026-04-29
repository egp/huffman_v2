/* tests/bb_header_tests.rs v2 */
use huffman_v2::header::{validate_header, Header};

#[test]
fn test_header_serialization_roundtrip() {
    let original = Header {
        version: 1,
        flags: 0b0000_0001,
    };

    let serialized = original.serialize();
    let validated = validate_header(&serialized).expect("Header should be valid");

    assert_eq!(validated.version, original.version);
    assert_eq!(validated.flags, original.flags);
}

#[test]
fn test_invalid_magic_bytes() {
    let corrupt_header = vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0];
    assert!(validate_header(&corrupt_header).is_err());
}

#[test]
fn test_corrupt_checksum() {
    let mut bytes = Header {
        version: 1,
        flags: 0,
    }
    .serialize();
    // Tamper with the flags byte to invalidate checksum
    bytes[5] ^= 0xFF;
    assert!(validate_header(&bytes).is_err());
}
/* tests/bb_header_tests.rs v2 */
