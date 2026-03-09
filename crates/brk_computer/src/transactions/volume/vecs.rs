use brk_traversable::Traversable;
use brk_types::StoredF32;
use vecdb::{Rw, StorageMode};

use crate::internal::{AmountFromHeight, AmountFromHeightRolling, ComputedFromHeight};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub sent_sum: AmountFromHeightRolling<M>,
    pub received_sum: AmountFromHeightRolling<M>,
    pub annualized_volume: AmountFromHeight<M>,
    pub tx_per_sec: ComputedFromHeight<StoredF32, M>,
    pub outputs_per_sec: ComputedFromHeight<StoredF32, M>,
    pub inputs_per_sec: ComputedFromHeight<StoredF32, M>,
}
