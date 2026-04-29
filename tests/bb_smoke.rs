/* tests/bb_smoke.rs v2 */
use huffman_v2::header::{validate_header, Header};

#[test]
fn test_header_smoke_roundtrip() {
    // Cycle 1: Testing the new checksummed header format
    let h = Header {
        version: 1,
        flags: 0,
    };

    let bytes = h.serialize();

    // The header is now 10 bytes: 4 (Magic) + 1 (Ver) + 1 (Flags) + 4 (Checksum)
    assert_eq!(bytes.len(), 10);

    let validated = validate_header(&bytes).expect("Header validation failed");
    assert_eq!(validated.version, 1);
    assert_eq!(validated.flags, 0);
}

#[test]
fn test_header_corruption_detection() {
    let h = Header {
        version: 1,
        flags: 0,
    };
    let mut bytes = h.serialize();

    // Corrupt the version byte
    bytes[4] = 99;

    // Validation should fail because the checksum won't match the new data
    assert!(validate_header(&bytes).is_err());
}
/* tests/bb_smoke.rs v2 */
