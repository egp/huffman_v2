/* src/lib.rs v5 */
pub mod bits;
pub mod checksum;
pub mod decode;
pub mod encode;
pub mod frame;
pub mod header;
pub mod huffman;

pub use decode::Decoder;
pub use huffman::build_frequency_table;
pub use huffman::build_tree;
/* src/lib.rs v5 */
