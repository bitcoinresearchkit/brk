use brk_traversable::Traversable;
use brk_types::StoredF32;
use vecdb::{Rw, StorageMode};

use crate::internal::{AmountPerBlockCumulativeWithSums, PerBlock};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub sent_sum: AmountPerBlockCumulativeWithSums<M>,
    pub received_sum: AmountPerBlockCumulativeWithSums<M>,
    pub tx_per_sec: PerBlock<StoredF32, M>,
    pub outputs_per_sec: PerBlock<StoredF32, M>,
    pub inputs_per_sec: PerBlock<StoredF32, M>,
}
