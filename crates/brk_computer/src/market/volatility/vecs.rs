use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeight, LazyFromHeight};

use brk_types::StoredF32;

/// Price volatility metrics (derived from returns standard deviation)
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price_volatility_1w: LazyFromHeight<StoredF32>,
    pub price_volatility_1m: LazyFromHeight<StoredF32>,
    pub price_volatility_1y: LazyFromHeight<StoredF32>,

    pub price_sharpe_1w: ComputedFromHeight<StoredF32, M>,
    pub price_sharpe_1m: ComputedFromHeight<StoredF32, M>,
    pub price_sharpe_1y: ComputedFromHeight<StoredF32, M>,

    pub price_sortino_1w: ComputedFromHeight<StoredF32, M>,
    pub price_sortino_1m: ComputedFromHeight<StoredF32, M>,
    pub price_sortino_1y: ComputedFromHeight<StoredF32, M>,
}
