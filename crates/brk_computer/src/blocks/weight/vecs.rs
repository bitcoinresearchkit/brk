use brk_traversable::Traversable;
use brk_types::{StoredF32, Weight};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightDistribution, ComputedHeightDerivedFull};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub weight: ComputedHeightDerivedFull<Weight, M>,
    pub fullness: ComputedFromHeightDistribution<StoredF32, M>,
}
