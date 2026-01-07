use brk_traversable::Traversable;
use brk_types::{StoredF32, Weight};

use crate::internal::{DerivedComputedBlockFull, LazyBlockFull};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_block_weight: DerivedComputedBlockFull<Weight>,
    /// Block fullness as percentage of max block weight (0-100%)
    pub indexes_to_block_fullness: LazyBlockFull<StoredF32, Weight>,
}
