use brk_traversable::Traversable;
use brk_types::{StoredU64, Weight};

use crate::internal::{ComputedDerivedBlockFull, LazyComputedBlockFull};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub vbytes: LazyComputedBlockFull<StoredU64, Weight>,
    pub size: ComputedDerivedBlockFull<StoredU64>,
}
