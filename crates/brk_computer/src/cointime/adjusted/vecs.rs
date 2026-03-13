use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, StoredF64};
use vecdb::{Rw, StorageMode};

use crate::internal::{PerBlock, PercentPerBlock};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub inflation_rate: PercentPerBlock<BasisPointsSigned32, M>,
    pub tx_velocity_btc: PerBlock<StoredF64, M>,
    pub tx_velocity_usd: PerBlock<StoredF64, M>,
}
