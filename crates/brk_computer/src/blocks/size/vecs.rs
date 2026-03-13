use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::{PerBlockFull, PerBlockRolling};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vbytes: PerBlockFull<StoredU64, M>,
    pub size: PerBlockRolling<StoredU64, M>,
}
