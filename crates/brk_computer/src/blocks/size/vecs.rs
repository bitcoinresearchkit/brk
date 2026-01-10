use brk_traversable::Traversable;
use brk_types::{StoredU64, Weight};

use crate::internal::{ComputedDerivedBlockFull, LazyBlockFullHeight};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub vbytes: LazyBlockFullHeight<StoredU64, Weight>,
    pub size: ComputedDerivedBlockFull<StoredU64>,
}
