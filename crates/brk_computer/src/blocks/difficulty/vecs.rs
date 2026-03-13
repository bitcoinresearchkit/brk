use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, Epoch, StoredF32, StoredF64, StoredU32};
use vecdb::{Rw, StorageMode};

use crate::internal::{LazyPerBlock, PerBlock, Resolutions, PercentPerBlock};
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub base: Resolutions<StoredF64>,
    pub as_hash: LazyPerBlock<StoredF64>,
    pub adjustment: PercentPerBlock<BasisPointsSigned32, M>,
    pub epoch: PerBlock<Epoch, M>,
    pub blocks_before_next: PerBlock<StoredU32, M>,
    pub days_before_next: LazyPerBlock<StoredF32, StoredU32>,
}
