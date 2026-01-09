use brk_traversable::Traversable;
use brk_types::StoredF32;

use crate::internal::{ComputedDateLast, ValueBlockSum, ValueDateLast};

/// Volume metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub sent_sum: ValueBlockSum,
    pub annualized_volume: ValueDateLast,
    pub tx_per_sec: ComputedDateLast<StoredF32>,
    pub outputs_per_sec: ComputedDateLast<StoredF32>,
    pub inputs_per_sec: ComputedDateLast<StoredF32>,
}
