use brk_traversable::Traversable;
use brk_types::{StoredF32, Weight};

use crate::internal::{ComputedHeightDerivedFull, LazyFromHeightTransformDistribution};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub weight: ComputedHeightDerivedFull<Weight>,
    pub fullness: LazyFromHeightTransformDistribution<StoredF32, Weight>,
}
