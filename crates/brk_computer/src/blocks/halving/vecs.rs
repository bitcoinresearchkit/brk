use brk_traversable::Traversable;
use brk_types::{HalvingEpoch, StoredF32, StoredU32};
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedFromHeight;
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub epoch: ComputedFromHeight<HalvingEpoch, M>,
    pub blocks_before_next_halving: ComputedFromHeight<StoredU32, M>,
    pub days_before_next_halving: ComputedFromHeight<StoredF32, M>,
}
