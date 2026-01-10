use brk_traversable::Traversable;
use brk_types::{StoredF32, StoredF64};

use crate::internal::ComputedFromDateLast;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub cointime_adj_inflation_rate: ComputedFromDateLast<StoredF32>,
    pub cointime_adj_tx_btc_velocity: ComputedFromDateLast<StoredF64>,
    pub cointime_adj_tx_usd_velocity: ComputedFromDateLast<StoredF64>,
}
