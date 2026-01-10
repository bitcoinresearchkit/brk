use brk_traversable::Traversable;
use brk_types::{StoredU64, Weight};

use crate::internal::{ComputedHeightDerivedFull, LazyComputedFromHeightFull};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub vbytes: LazyComputedFromHeightFull<StoredU64, Weight>,
    pub size: ComputedHeightDerivedFull<StoredU64>,
}
