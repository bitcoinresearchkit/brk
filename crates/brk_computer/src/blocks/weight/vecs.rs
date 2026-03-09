use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Weight};
use vecdb::{Rw, StorageMode};

use crate::internal::{ResolutionsFull, PercentPerBlockRollingAverage};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub weight: ResolutionsFull<Weight, M>,
    pub fullness: PercentPerBlockRollingAverage<BasisPoints16, M>,
}
