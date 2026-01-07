use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Sats, StoredF32};

use crate::internal::{ComputedDateLast, ValueBlockSum};

/// Volume metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_sent_sum: ValueBlockSum,
    pub indexes_to_annualized_volume: ComputedDateLast<Sats>,
    pub indexes_to_annualized_volume_btc: ComputedDateLast<Bitcoin>,
    pub indexes_to_annualized_volume_usd: ComputedDateLast<Dollars>,
    pub indexes_to_tx_per_sec: ComputedDateLast<StoredF32>,
    pub indexes_to_outputs_per_sec: ComputedDateLast<StoredF32>,
    pub indexes_to_inputs_per_sec: ComputedDateLast<StoredF32>,
}
