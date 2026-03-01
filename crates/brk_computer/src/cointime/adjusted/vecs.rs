use brk_traversable::Traversable;
use brk_types::{StoredF32, StoredF64};
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedFromHeight;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub cointime_adj_inflation_rate: ComputedFromHeight<StoredF32, M>,
    pub cointime_adj_tx_btc_velocity: ComputedFromHeight<StoredF64, M>,
    pub cointime_adj_tx_usd_velocity: ComputedFromHeight<StoredF64, M>,
}
