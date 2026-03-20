use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::PerBlockCumulativeRolling;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub v1: PerBlockCumulativeRolling<StoredU64, StoredU64, M>,
    pub v2: PerBlockCumulativeRolling<StoredU64, StoredU64, M>,
    pub v3: PerBlockCumulativeRolling<StoredU64, StoredU64, M>,
}
