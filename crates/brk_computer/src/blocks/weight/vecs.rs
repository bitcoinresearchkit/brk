use brk_traversable::Traversable;
use brk_types::{StoredF32, Weight};
use vecdb::{Rw, StorageMode};

use crate::internal::{
    ComputedFromHeightLast, ComputedHeightDerivedCumulativeFull, RollingDistribution,
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub weight: ComputedHeightDerivedCumulativeFull<Weight, M>,
    pub fullness: ComputedFromHeightLast<StoredF32, M>,
    pub fullness_rolling: RollingDistribution<StoredF32, M>,
}
