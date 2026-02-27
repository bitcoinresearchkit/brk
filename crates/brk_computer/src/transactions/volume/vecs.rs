use brk_traversable::Traversable;
use brk_types::StoredF32;
use vecdb::{Rw, StorageMode};

use crate::internal::{
    ComputedFromHeightLast, ValueFromHeightLast, ValueFromHeightLastRolling,
};

/// Volume metrics
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub sent_sum: ValueFromHeightLastRolling<M>,
    pub received_sum: ValueFromHeightLastRolling<M>,
    pub annualized_volume: ValueFromHeightLast<M>,
    pub tx_per_sec: ComputedFromHeightLast<StoredF32, M>,
    pub outputs_per_sec: ComputedFromHeightLast<StoredF32, M>,
    pub inputs_per_sec: ComputedFromHeightLast<StoredF32, M>,
}
