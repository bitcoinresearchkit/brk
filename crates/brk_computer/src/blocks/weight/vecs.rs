use brk_traversable::Traversable;
use brk_types::{StoredF32, Weight};

use crate::internal::{ComputedHeightDerivedFull, LazyFromHeightFull};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub weight: ComputedHeightDerivedFull<Weight>,
    pub fullness: LazyFromHeightFull<StoredF32, Weight>,
}
