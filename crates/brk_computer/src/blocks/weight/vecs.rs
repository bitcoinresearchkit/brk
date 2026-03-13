use brk_traversable::Traversable;
use brk_types::{BasisPoints16, StoredU64, Weight};
use vecdb::{Rw, StorageMode};

use crate::internal::{LazyResolutionsFull, PercentPerBlockRollingAverage};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub weight: LazyResolutionsFull<Weight, StoredU64>,
    pub fullness: PercentPerBlockRollingAverage<BasisPoints16, M>,
}
