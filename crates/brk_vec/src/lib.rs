#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

mod enums;
mod structs;
mod traits;
mod variants;

pub use enums::*;
pub use memmap2::Mmap;
pub use structs::*;
pub use traits::*;
pub use variants::*;
