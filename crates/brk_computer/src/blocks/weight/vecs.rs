use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Weight};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedHeightDerivedFull, PercentFromHeightDistribution};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub weight: ComputedHeightDerivedFull<Weight, M>,
    pub fullness: PercentFromHeightDistribution<BasisPoints16, M>,
}
