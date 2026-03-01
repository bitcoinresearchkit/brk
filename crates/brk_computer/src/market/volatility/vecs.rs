use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeight, LazyFromHeight};

use brk_types::StoredF32;

/// Price volatility metrics (derived from returns standard deviation)
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price_1w_volatility: LazyFromHeight<StoredF32>,
    pub price_1m_volatility: LazyFromHeight<StoredF32>,
    pub price_1y_volatility: LazyFromHeight<StoredF32>,

    pub sharpe_1w: ComputedFromHeight<StoredF32, M>,
    pub sharpe_1m: ComputedFromHeight<StoredF32, M>,
    pub sharpe_1y: ComputedFromHeight<StoredF32, M>,

    pub sortino_1w: ComputedFromHeight<StoredF32, M>,
    pub sortino_1m: ComputedFromHeight<StoredF32, M>,
    pub sortino_1y: ComputedFromHeight<StoredF32, M>,
}
