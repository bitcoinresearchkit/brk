use brk_traversable::Traversable;
use brk_types::{StoredF32, Weight};

use crate::internal::{DerivedComputedBlockFull, LazyBlockFull};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub weight: DerivedComputedBlockFull<Weight>,
    pub fullness: LazyBlockFull<StoredF32, Weight>,
}
