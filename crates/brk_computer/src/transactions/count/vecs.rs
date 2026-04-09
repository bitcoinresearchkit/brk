use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::PerBlockFull;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub total: PerBlockFull<StoredU64, M>,
}
