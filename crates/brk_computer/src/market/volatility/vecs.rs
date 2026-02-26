use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightLast, LazyFromHeightLast};

use brk_types::StoredF32;

/// Price volatility metrics (derived from returns standard deviation)
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price_1w_volatility: LazyFromHeightLast<StoredF32>,
    pub price_1m_volatility: LazyFromHeightLast<StoredF32>,
    pub price_1y_volatility: LazyFromHeightLast<StoredF32>,

    pub sharpe_1w: ComputedFromHeightLast<StoredF32, M>,
    pub sharpe_1m: ComputedFromHeightLast<StoredF32, M>,
    pub sharpe_1y: ComputedFromHeightLast<StoredF32, M>,

    pub sortino_1w: ComputedFromHeightLast<StoredF32, M>,
    pub sortino_1m: ComputedFromHeightLast<StoredF32, M>,
    pub sortino_1y: ComputedFromHeightLast<StoredF32, M>,
}
