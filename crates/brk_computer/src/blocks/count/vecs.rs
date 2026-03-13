use brk_traversable::Traversable;
use brk_types::{StoredU32, StoredU64};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlockCumulativeWithSums, ConstantVecs};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub target: ConstantVecs<StoredU64>,
    pub total: ComputedPerBlockCumulativeWithSums<StoredU32, StoredU64, M>,
}
