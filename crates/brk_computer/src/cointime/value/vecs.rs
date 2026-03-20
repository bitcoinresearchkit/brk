use brk_traversable::Traversable;
use brk_types::StoredF64;
use vecdb::{Rw, StorageMode};

use crate::internal::PerBlockCumulativeRolling;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub destroyed: PerBlockCumulativeRolling<StoredF64, StoredF64, M>,
    pub created: PerBlockCumulativeRolling<StoredF64, StoredF64, M>,
    pub stored: PerBlockCumulativeRolling<StoredF64, StoredF64, M>,
    pub vocdd: PerBlockCumulativeRolling<StoredF64, StoredF64, M>,
}
