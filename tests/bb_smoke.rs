// tests/bb_smoke.rs v1

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
// tests/bb_smoke.rs v1
