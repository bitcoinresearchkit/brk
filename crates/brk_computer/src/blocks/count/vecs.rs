use brk_traversable::Traversable;
use brk_types::{StoredU32, StoredU64};
use vecdb::{Rw, StorageMode};

use crate::internal::{ConstantVecs, PerBlockCumulativeRolling, Windows};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub target: Windows<ConstantVecs<StoredU64>>,
    pub total: PerBlockCumulativeRolling<StoredU32, StoredU64, M>,
}
