use brk_traversable::Traversable;
use brk_types::StoredF32;
use vecdb::{Rw, StorageMode};

use crate::internal::{AmountPerBlockCumulativeRolling, PerBlock, Windows};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub transfer_volume: AmountPerBlockCumulativeRolling<M>,
    pub tx_per_sec: Windows<PerBlock<StoredF32, M>>,
}
