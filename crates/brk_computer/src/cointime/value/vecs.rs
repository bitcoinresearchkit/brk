use brk_traversable::Traversable;
use brk_types::StoredF64;
use vecdb::{Rw, StorageMode};

use crate::internal::PerBlockCumulativeWithSums;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub destroyed: PerBlockCumulativeWithSums<StoredF64, StoredF64, M>,
    pub created: PerBlockCumulativeWithSums<StoredF64, StoredF64, M>,
    pub stored: PerBlockCumulativeWithSums<StoredF64, StoredF64, M>,
    pub vocdd: PerBlockCumulativeWithSums<StoredF64, StoredF64, M>,
}
