// tests/bb_smoke.rs v4

use huffman_v2::*;

#[test]
fn roundtrip_empty_input() {
    let input: &[u8] = b"";
    let encoded = encode(input, None);
    let decoded = decode(&encoded, None);
    assert_eq!(decoded, input);
}

#[test]
fn roundtrip_single_byte() {
    let input = b"a";
    let encoded = encode(input, None);
    let decoded = decode(&encoded, None);
    assert_eq!(decoded, input);
}

#[test]
fn deterministic_output_stub_level() {
    let input = b"hello hello hello";
    let e1 = encode(input, None);
    let e2 = encode(input, None);
    assert_eq!(e1, e2);
}

// -------- Frequency Table --------

#[test]
fn frequency_table_basic_counts() {
    let input = b"aab";
    let table = build_frequency_table(input);

    assert_eq!(table[b'a' as usize], 2);
    assert_eq!(table[b'b' as usize], 1);
}

// -------- Checksum --------

#[test]
fn checksum_known_vector_hello() {
    assert_eq!(checksum32(b"hello"), 0x4f9f2cab);
}

// -------- Frame Phase --------

#[test]
fn header_serialize_size() {
    let h = Header {
        flags: 0,
        original_size: 123,
        payload_size: 456,
    };

    let bytes = serialize_header(&h);

    assert_eq!(bytes.len(), HEADER_SIZE);
}

#[test]
#[ignore]
fn header_roundtrip() {
    let h = Header {
        flags: 0xAA,
        original_size: 123456,
        payload_size: 654321,
    };

    let bytes = serialize_header(&h);
    let parsed = parse_header(&bytes).unwrap();

    assert_eq!(parsed, h);
}

#[test]
#[ignore]
fn header_rejects_bad_magic() {
    let mut bytes = vec![0u8; HEADER_SIZE];

    bytes[0..4].copy_from_slice(b"BAD!");

    let result = parse_header(&bytes);

    assert!(result.is_err());
}

#[test]
#[ignore]
fn header_checksum_detects_corruption() {
    let h = Header {
        flags: 0,
        original_size: 1,
        payload_size: 1,
    };

    let mut bytes = serialize_header(&h);

    // flip one bit
    bytes[10] ^= 0x01;

    let result = parse_header(&bytes);

    assert!(result.is_err());
}

#[test]
fn header_serialize_writes_magic() {
    let h = Header {
        flags: 0,
        original_size: 123,
        payload_size: 456,
    };

    let bytes = serialize_header(&h);

    // Must be correct size
    assert_eq!(bytes.len(), HEADER_SIZE);

    // MUST write magic
    assert_eq!(&bytes[0..4], b"HUF1");
}

#[test]
fn header_writes_version() {
    let h = Header {
        flags: 0,
        original_size: 123,
        payload_size: 456,
    };

    let bytes = serialize_header(&h);

    assert_eq!(bytes.len(), HEADER_SIZE);

    // magic already enforced
    assert_eq!(&bytes[0..4], b"HUF1");

    // VERSION MUST BE 1
    assert_eq!(bytes[5], 1);
}

// tests/bb_smoke.rs v4
