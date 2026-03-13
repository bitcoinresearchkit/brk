use brk_traversable::Traversable;
use brk_types::{BasisPoints16, StoredU64, Weight};
use vecdb::{Rw, StorageMode};

use crate::internal::{LazyPerBlockRolling, PercentPerBlockRollingAverage};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub weight: LazyPerBlockRolling<Weight, StoredU64>,
    pub fullness: PercentPerBlockRollingAverage<BasisPoints16, M>,
}
