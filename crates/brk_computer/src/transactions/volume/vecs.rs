use brk_traversable::Traversable;
use brk_types::StoredF32;
use vecdb::{Rw, StorageMode};

use crate::internal::{AmountPerBlockCumulativeWithSums, ComputedPerBlock};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub sent_sum: AmountPerBlockCumulativeWithSums<M>,
    pub received_sum: AmountPerBlockCumulativeWithSums<M>,
    pub tx_per_sec: ComputedPerBlock<StoredF32, M>,
    pub outputs_per_sec: ComputedPerBlock<StoredF32, M>,
    pub inputs_per_sec: ComputedPerBlock<StoredF32, M>,
}
