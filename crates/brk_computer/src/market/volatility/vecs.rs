use brk_traversable::Traversable;
use brk_types::{Height, StoredF32};
use vecdb::LazyVecFrom2;

use crate::internal::LazyFromHeightLast;

/// Price volatility metrics (derived from returns standard deviation)
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price_1w_volatility: LazyFromHeightLast<StoredF32>,
    pub price_1m_volatility: LazyFromHeightLast<StoredF32>,
    pub price_1y_volatility: LazyFromHeightLast<StoredF32>,

    pub sharpe_1w: LazyVecFrom2<Height, StoredF32, Height, StoredF32, Height, StoredF32>,
    pub sharpe_1m: LazyVecFrom2<Height, StoredF32, Height, StoredF32, Height, StoredF32>,
    pub sharpe_1y: LazyVecFrom2<Height, StoredF32, Height, StoredF32, Height, StoredF32>,

    pub sortino_1w: LazyVecFrom2<Height, StoredF32, Height, StoredF32, Height, StoredF32>,
    pub sortino_1m: LazyVecFrom2<Height, StoredF32, Height, StoredF32, Height, StoredF32>,
    pub sortino_1y: LazyVecFrom2<Height, StoredF32, Height, StoredF32, Height, StoredF32>,
}
