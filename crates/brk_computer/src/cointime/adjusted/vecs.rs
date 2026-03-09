use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, StoredF64};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlock, PercentPerBlock};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub adj_inflation_rate: PercentPerBlock<BasisPointsSigned32, M>,
    pub adj_tx_velocity_btc: ComputedPerBlock<StoredF64, M>,
    pub adj_tx_velocity_usd: ComputedPerBlock<StoredF64, M>,
}
