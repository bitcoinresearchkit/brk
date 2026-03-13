use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::{PerBlock, PerBlockAggregated};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub total: PerBlockAggregated<StoredU64, M>,
    pub unspent: PerBlock<StoredU64, M>,
}
