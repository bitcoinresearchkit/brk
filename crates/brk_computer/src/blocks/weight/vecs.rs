use brk_traversable::Traversable;
use brk_types::{StoredF32, Weight};

use crate::internal::{ComputedDerivedBlockFull, LazyBlockFull};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub weight: ComputedDerivedBlockFull<Weight>,
    pub fullness: LazyBlockFull<StoredF32, Weight>,
}
