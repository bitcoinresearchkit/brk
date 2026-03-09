use brk_traversable::Traversable;
use brk_types::{StoredU32, StoredU64};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlockCumulativeSum, ConstantVecs};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub block_count_target: ConstantVecs<StoredU64>,
    pub block_count: ComputedPerBlockCumulativeSum<StoredU32, M>,
}
