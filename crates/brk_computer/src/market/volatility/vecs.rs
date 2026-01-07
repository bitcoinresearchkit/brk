use brk_traversable::Traversable;
use brk_types::{DateIndex, StoredF32};
use vecdb::LazyVecFrom2;

use crate::internal::LazyDateLast;

/// Price volatility metrics (derived from returns standard deviation)
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_price_1w_volatility: LazyDateLast<StoredF32>,
    pub indexes_to_price_1m_volatility: LazyDateLast<StoredF32>,
    pub indexes_to_price_1y_volatility: LazyDateLast<StoredF32>,

    // KISS: now concrete since source is KISS
    pub dateindex_to_sharpe_1w: LazyVecFrom2<DateIndex, StoredF32, DateIndex, StoredF32, DateIndex, StoredF32>,
    pub dateindex_to_sharpe_1m: LazyVecFrom2<DateIndex, StoredF32, DateIndex, StoredF32, DateIndex, StoredF32>,
    pub dateindex_to_sharpe_1y: LazyVecFrom2<DateIndex, StoredF32, DateIndex, StoredF32, DateIndex, StoredF32>,

    pub dateindex_to_sortino_1w: LazyVecFrom2<DateIndex, StoredF32, DateIndex, StoredF32, DateIndex, StoredF32>,
    pub dateindex_to_sortino_1m: LazyVecFrom2<DateIndex, StoredF32, DateIndex, StoredF32, DateIndex, StoredF32>,
    pub dateindex_to_sortino_1y: LazyVecFrom2<DateIndex, StoredF32, DateIndex, StoredF32, DateIndex, StoredF32>,
}
