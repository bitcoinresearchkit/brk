use brk_traversable::Traversable;
use brk_types::StoredF32;

use crate::internal::{ComputedFromDateLast, ValueFromHeightSum, ValueFromDateLast};

/// Volume metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub sent_sum: ValueFromHeightSum,
    pub annualized_volume: ValueFromDateLast,
    pub tx_per_sec: ComputedFromDateLast<StoredF32>,
    pub outputs_per_sec: ComputedFromDateLast<StoredF32>,
    pub inputs_per_sec: ComputedFromDateLast<StoredF32>,
}
