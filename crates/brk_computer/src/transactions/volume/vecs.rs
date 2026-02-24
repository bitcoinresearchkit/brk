use brk_traversable::Traversable;
use brk_types::StoredF32;
use vecdb::{Rw, StorageMode};

use crate::internal::{
    ComputedFromHeightLast, StoredValueRollingWindows, ValueFromHeightLast,
};

/// Volume metrics
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub sent_sum: ValueFromHeightLast<M>,
    pub sent_sum_rolling: StoredValueRollingWindows<M>,
    #[traversable(flatten)]
    pub received_sum: ValueFromHeightLast<M>,
    pub received_sum_rolling: StoredValueRollingWindows<M>,
    #[traversable(flatten)]
    pub annualized_volume: ValueFromHeightLast<M>,
    pub tx_per_sec: ComputedFromHeightLast<StoredF32, M>,
    pub outputs_per_sec: ComputedFromHeightLast<StoredF32, M>,
    pub inputs_per_sec: ComputedFromHeightLast<StoredF32, M>,
}
