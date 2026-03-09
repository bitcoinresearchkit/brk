use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlockFull, ResolutionsFull};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vbytes: ComputedPerBlockFull<StoredU64, M>,
    pub size: ResolutionsFull<StoredU64, M>,
}
