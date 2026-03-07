use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Weight};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedHeightDerivedFull, PercentFromHeightRollingAverage};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub weight: ComputedHeightDerivedFull<Weight, M>,
    pub fullness: PercentFromHeightRollingAverage<BasisPoints16, M>,
}
