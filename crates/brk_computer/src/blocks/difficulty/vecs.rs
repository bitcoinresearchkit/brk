use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, Epoch, StoredF32, StoredF64, StoredU32};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlock, Resolutions, PercentPerBlock};
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub raw: Resolutions<StoredF64>,
    pub as_hash: ComputedPerBlock<StoredF64, M>,
    pub adjustment: PercentPerBlock<BasisPointsSigned32, M>,
    pub epoch: ComputedPerBlock<Epoch, M>,
    pub blocks_before_next: ComputedPerBlock<StoredU32, M>,
    pub days_before_next: ComputedPerBlock<StoredF32, M>,
}
