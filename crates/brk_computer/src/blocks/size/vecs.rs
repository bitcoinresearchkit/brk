use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightFull, ComputedHeightDerivedFull};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vbytes: ComputedFromHeightFull<StoredU64, M>,
    pub size: ComputedHeightDerivedFull<StoredU64, M>,
}
