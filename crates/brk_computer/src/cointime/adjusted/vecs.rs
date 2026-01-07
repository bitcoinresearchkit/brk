use brk_traversable::Traversable;
use brk_types::{StoredF32, StoredF64};

use crate::internal::ComputedDateLast;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_cointime_adj_inflation_rate: ComputedDateLast<StoredF32>,
    pub indexes_to_cointime_adj_tx_btc_velocity: ComputedDateLast<StoredF64>,
    pub indexes_to_cointime_adj_tx_usd_velocity: ComputedDateLast<StoredF64>,
}
