use brk_traversable::Traversable;
use brk_types::{StoredU64, Weight};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedHeightDerivedFull, LazyComputedFromHeightFull};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vbytes: LazyComputedFromHeightFull<StoredU64, Weight, M>,
    pub size: ComputedHeightDerivedFull<StoredU64, M>,
}
