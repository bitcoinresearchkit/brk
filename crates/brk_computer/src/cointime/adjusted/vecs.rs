use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, StoredF64};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeight, PercentFromHeight};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub cointime_adj_inflation_rate: PercentFromHeight<BasisPointsSigned32, M>,
    pub cointime_adj_tx_velocity_btc: ComputedFromHeight<StoredF64, M>,
    pub cointime_adj_tx_velocity_usd: ComputedFromHeight<StoredF64, M>,
}
