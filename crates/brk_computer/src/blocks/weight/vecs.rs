use brk_traversable::Traversable;
use brk_types::{StoredF32, Weight};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedHeightDerivedFull, LazyFromHeightTransformDistribution};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub weight: ComputedHeightDerivedFull<Weight, M>,
    pub fullness: LazyFromHeightTransformDistribution<StoredF32, Weight>,
}
