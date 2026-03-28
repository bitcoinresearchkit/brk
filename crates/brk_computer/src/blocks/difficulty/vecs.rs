use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, Epoch, StoredF32, StoredF64, StoredU32};
use vecdb::{Rw, StorageMode};

use crate::internal::{LazyPerBlock, PerBlock, PercentPerBlock, Resolutions};
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub value: Resolutions<StoredF64>,
    pub hashrate: LazyPerBlock<StoredF64>,
    pub adjustment: PercentPerBlock<BasisPointsSigned32, M>,
    pub epoch: PerBlock<Epoch, M>,
    pub blocks_to_retarget: PerBlock<StoredU32, M>,
    pub days_to_retarget: LazyPerBlock<StoredF32, StoredU32>,
}
