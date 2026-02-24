use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightCumFull, ComputedHeightDerivedCumFull};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vbytes: ComputedFromHeightCumFull<StoredU64, M>,
    pub size: ComputedHeightDerivedCumFull<StoredU64, M>,
}
