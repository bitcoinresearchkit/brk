#![doc = include_str!("../README.md")]
#![doc = "\n## Examples\n"]
#![doc = "\n### File\n\n```rust"]
#![doc = include_str!("../examples/file.rs")]
#![doc = "```\n"]
#![doc = "\n### Raw\n\n```rust"]
#![doc = include_str!("../examples/raw.rs")]
#![doc = "```"]

mod error;
mod exit;
mod file;
mod stamp;
mod traits;
mod variants;
mod version;

pub use brk_vecs_macros::StoredCompressed;
pub use pco::data_types::LatentType;

use variants::*;

pub use error::*;
pub use exit::*;
pub use file::{File, PAGE_SIZE, Reader};
pub use stamp::*;
pub use traits::*;
pub use variants::{
    CompressedVec, Computation, ComputedVec, ComputedVecFrom1, ComputedVecFrom2, ComputedVecFrom3,
    EagerVec, Format, LazyVecFrom1, LazyVecFrom2, LazyVecFrom3, RawVec, StoredVec,
};
pub use version::*;
