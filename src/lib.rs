// src/lib.rs v5
pub mod checksum;
pub mod decode;
pub mod encode;
pub mod frame;
pub mod header;
pub mod huffman;

pub use decode::decode;
pub use encode::encode;

pub use checksum::checksum32;
pub use frame::{pack, unpack, FrameType};
pub use header::{parse_header, serialize_header, Header, HEADER_SIZE};
pub use huffman::build_frequency_table;
// src/lib.rs v5
