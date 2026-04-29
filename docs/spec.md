# Huffman_v2 Project Specification

## 1. Frame Architecture
The stream consists of three discrete frame types, each protected by a checksum (defined in `src/checksum.rs`[cite: 1]).

*   **Header Frame:** 
    - Magic Bytes (e.g., `HUFF`).
    - Version Byte.
    - Flags (Encryption active, Sparse Table, etc.).
*   **Frequency Frame:** 
    - Deterministic Source: Either a 1024-byte table (256 * 4-byte counts) or a UTF-8 filename string for external table lookup.
    - Logic: Both encoder and decoder derive identical Huffman Trees from this data.
*   **Payload Frame:** 
    - Encoded bitstream.
    - Byte-alignment padding.
    - Optional XOR encryption using a user-provided passphrase.

## 2. Core Logic
*   **Deterministic Tree (`huffman.rs`):** Ensures that given the same frequency table, the resulting prefix codes are identical[cite: 1].
*   **I/O Operations:** `BitWriter` and `BitReader` (housed in `huffman.rs`[cite: 1]) manage bit-to-byte packing.
*   **Security:** XOR-based obfuscation applied only to the Payload Frame.

## 3. Implementation Philosophy
- **DfT/TDD:** Red/Green cycle focus.
- **Modularity:** Separate framing, checksumming, and codec logic[cite: 1].