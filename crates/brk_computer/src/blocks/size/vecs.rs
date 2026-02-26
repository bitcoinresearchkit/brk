use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightCumulativeFull, ComputedHeightDerivedCumulativeFull};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vbytes: ComputedFromHeightCumulativeFull<StoredU64, M>,
    pub size: ComputedHeightDerivedCumulativeFull<StoredU64, M>,
}
