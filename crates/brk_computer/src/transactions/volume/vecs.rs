use brk_traversable::Traversable;
use brk_types::StoredF32;
use vecdb::{Rw, StorageMode};

use crate::internal::{
    ComputedFromHeight, ValueFromHeight, ValueFromHeightRolling,
};

/// Volume metrics
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub sent_sum: ValueFromHeightRolling<M>,
    pub received_sum: ValueFromHeightRolling<M>,
    pub annualized_volume: ValueFromHeight<M>,
    pub tx_per_sec: ComputedFromHeight<StoredF32, M>,
    pub outputs_per_sec: ComputedFromHeight<StoredF32, M>,
    pub inputs_per_sec: ComputedFromHeight<StoredF32, M>,
}
