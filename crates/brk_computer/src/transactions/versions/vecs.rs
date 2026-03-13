use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::PerBlockCumulativeWithSums;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub v1: PerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub v2: PerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub v3: PerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
}
