use brk_traversable::Traversable;
use brk_types::{StoredU32, StoredU64};
use vecdb::{Rw, StorageMode};

use crate::internal::{PerBlockCumulativeWithSums, ConstantVecs};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub target: ConstantVecs<StoredU64>,
    pub total: PerBlockCumulativeWithSums<StoredU32, StoredU64, M>,
}
