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
fn header_roundtrip_debug() {
    let h = Header {
        flags: 0xAA,
        original_size: 0x1122334455667788,
        payload_size: 0x0102030405060708,
    };

    let bytes = serialize_header(&h);
    let parsed = parse_header(&bytes).unwrap();

    println!("ORIG: {:x?}", h);
    println!("PARSED: {:x?}", parsed);

    assert_eq!(parsed.flags, h.flags);
    assert_eq!(parsed.original_size, h.original_size);
    assert_eq!(parsed.payload_size, h.payload_size);
}

#[test]
fn header_rejects_bad_magic() {
    let mut bytes = vec![0u8; HEADER_SIZE];

    bytes[0..4].copy_from_slice(b"BAD!");

    let result = parse_header(&bytes);

    assert!(result.is_err());
}

#[test]
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

#[test]
fn header_writes_flags_bitfield() {
    let h = Header {
        flags: 0b0000_0001, // XOR flag ON (future use, but must serialize correctly)
        original_size: 123,
        payload_size: 456,
    };

    let bytes = serialize_header(&h);

    assert_eq!(bytes.len(), HEADER_SIZE);

    // magic
    assert_eq!(&bytes[0..4], b"HUF1");

    // version
    assert_eq!(bytes[5], 1);

    // flags MUST be preserved exactly
    assert_eq!(bytes[6], 0b0000_0001);
}

#[test]
fn header_writes_original_size_le() {
    let h = Header {
        flags: 0,
        original_size: 0x1122334455667788,
        payload_size: 0,
    };

    let bytes = serialize_header(&h);

    assert_eq!(bytes.len(), HEADER_SIZE);

    // HARD CHECK: exact magic + version + flags context
    assert_eq!(&bytes[0..4], b"HUF1");
    assert_eq!(bytes[4], 1);
    assert_eq!(bytes[5], 1);
    assert_eq!(bytes[6], 0);

    // HARD CHECK: must NOT be zero-filled
    let all_zero = bytes.iter().all(|&b| b == 0);
    assert!(!all_zero, "header is still uninitialized stub");

    // STRICT CHECK: original_size exact byte match
    let expected = 0x1122334455667788u64.to_le_bytes();

    assert_eq!(&bytes[8..16], &expected);
}

#[test]
fn debug_dump_header_bytes() {
    let h = Header {
        flags: 0xAA,
        original_size: 0x1122334455667788,
        payload_size: 0x0102030405060708,
    };

    let bytes = serialize_header(&h);

    println!("{:02x?}", bytes);

    // force visibility in CI logs
    assert_eq!(bytes.len(), HEADER_SIZE);
}

#[test]
fn header_writes_checksum_field() {
    let h = Header {
        flags: 0xAA,
        original_size: 0x1122334455667788,
        payload_size: 0x0102030405060708,
    };

    let bytes = serialize_header(&h);

    assert_eq!(bytes.len(), HEADER_SIZE);

    // sanity checks
    assert_eq!(&bytes[0..4], b"HUF1");
    assert_eq!(bytes[5], 1);

    // checksum field must NOT be zero (once implemented)
    let mut chk = [0u8; 4];
    chk.copy_from_slice(&bytes[24..28]);

    let checksum = u32::from_le_bytes(chk);

    assert_ne!(checksum, 0, "checksum not written");
}

#[test]
fn frame_header_type_exists() {
    let frame_type = FrameType::Header;

    assert_eq!(frame_type as u8, 0);
}

#[test]
fn frame_basic_pack_and_unpack() {
    let payload = vec![1u8, 2, 3, 4];

    let frame = frame::pack(FrameType::Header, &payload);
    let (t, out) = frame::unpack(&frame).unwrap();

    assert_eq!(t, FrameType::Header);
    assert_eq!(out, payload);
}

// tests/bb_smoke.rs v4
