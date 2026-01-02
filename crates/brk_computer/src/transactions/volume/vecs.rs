use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Sats, StoredF32};

use crate::internal::{ComputedValueVecsFromHeight, ComputedVecsFromDateIndex};

/// Volume metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_sent_sum: ComputedValueVecsFromHeight,
    pub indexes_to_annualized_volume: ComputedVecsFromDateIndex<Sats>,
    pub indexes_to_annualized_volume_btc: ComputedVecsFromDateIndex<Bitcoin>,
    pub indexes_to_annualized_volume_usd: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_tx_per_sec: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_outputs_per_sec: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_inputs_per_sec: ComputedVecsFromDateIndex<StoredF32>,
}
