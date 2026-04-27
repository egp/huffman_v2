// tests/bb_smoke.rs v3

use huffman_v2;

#[test]
fn roundtrip_empty_input() {
    let input: &[u8] = b"";

    let encoded = huffman_v2::encode(input, None);
    let decoded = huffman_v2::decode(&encoded, None);

    assert_eq!(decoded, input);
}

#[test]
fn roundtrip_single_byte() {
    let input = b"a";

    let encoded = huffman_v2::encode(input, None);
    let decoded = huffman_v2::decode(&encoded, None);

    assert_eq!(decoded, input);
}

#[test]
fn deterministic_output_stub_level() {
    let input = b"hello hello hello";

    let e1 = huffman_v2::encode(input, None);
    let e2 = huffman_v2::encode(input, None);

    assert_eq!(e1, e2);
}

//
// -------- Phase 2: Frequency Table --------
//

#[test]
fn frequency_table_basic_counts() {
    let input = b"aab";

    let table = huffman_v2::build_frequency_table(input);

    assert_eq!(table[b'a' as usize], 2);
    assert_eq!(table[b'b' as usize], 1);

    for (i, &count) in table.iter().enumerate() {
        if i != b'a' as usize && i != b'b' as usize {
            assert_eq!(count, 0);
        }
    }
}

#[test]
fn frequency_table_all_bytes() {
    let input: Vec<u8> = (0u8..=255u8).collect();

    let table = huffman_v2::build_frequency_table(&input);

    for i in 0..256 {
        assert_eq!(table[i], 1);
    }
}

//
// -------- Phase 3: Checksum (FNV-1a) --------
//

#[test]
fn checksum_known_vector_hello() {
    let data = b"hello";

    let checksum = huffman_v2::checksum32(data);

    // FNV-1a 32-bit known value for "hello"
    assert_eq!(checksum, 0x4f9f2cab);
}

#[test]
fn checksum_known_vector_empty() {
    let data = b"";

    let checksum = huffman_v2::checksum32(data);

    // offset basis (no bytes processed)
    assert_eq!(checksum, 0x811c9dc5);
}

#[test]
fn checksum_deterministic() {
    let data = b"some data";

    let c1 = huffman_v2::checksum32(data);
    let c2 = huffman_v2::checksum32(data);

    assert_eq!(c1, c2);
}

#[test]
fn checksum_detects_change() {
    let data1 = b"some data";
    let data2 = b"some data.";

    let c1 = huffman_v2::checksum32(data1);
    let c2 = huffman_v2::checksum32(data2);

    assert_ne!(c1, c2);
}

// tests/bb_smoke.rs v3
