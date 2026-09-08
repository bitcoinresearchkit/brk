use bitview_traversable::Traversable;
use brk_types::{StoredU64, Weight};
use vecdb::{Pinned, Rw, StorageMode};

use bitview_compute::{PerBlockFull, PerBlockRolling};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    /// Virtual size of the block in vbytes, computed as the block weight in
    /// weight units divided by four and rounded down.
    pub vbytes: PerBlockFull<StoredU64, Weight, M>,
    /// Total serialized block size in bytes, including the header,
    /// transaction-count CompactSize, and witness data.
    pub size: PerBlockRolling<StoredU64, M, Pinned>,
}
