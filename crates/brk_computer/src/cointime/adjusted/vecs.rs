use brk_traversable::Traversable;
use brk_types::{StoredF32, StoredF64};

use crate::internal::ComputedVecsFromDateIndex;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_cointime_adj_inflation_rate: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_cointime_adj_tx_btc_velocity: ComputedVecsFromDateIndex<StoredF64>,
    pub indexes_to_cointime_adj_tx_usd_velocity: ComputedVecsFromDateIndex<StoredF64>,
}
