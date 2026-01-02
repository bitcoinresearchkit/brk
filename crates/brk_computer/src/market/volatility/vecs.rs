use brk_traversable::Traversable;
use brk_types::{DateIndex, StoredF32};
use vecdb::LazyVecFrom2;

use crate::internal::LazyVecsFromDateIndex;

/// Price volatility metrics (derived from returns standard deviation)
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_price_1w_volatility: LazyVecsFromDateIndex<StoredF32>,
    pub indexes_to_price_1m_volatility: LazyVecsFromDateIndex<StoredF32>,
    pub indexes_to_price_1y_volatility: LazyVecsFromDateIndex<StoredF32>,

    pub dateindex_to_sharpe_1w: Option<LazyVecFrom2<DateIndex, StoredF32, DateIndex, StoredF32, DateIndex, StoredF32>>,
    pub dateindex_to_sharpe_1m: Option<LazyVecFrom2<DateIndex, StoredF32, DateIndex, StoredF32, DateIndex, StoredF32>>,
    pub dateindex_to_sharpe_1y: Option<LazyVecFrom2<DateIndex, StoredF32, DateIndex, StoredF32, DateIndex, StoredF32>>,

    pub dateindex_to_sortino_1w: Option<LazyVecFrom2<DateIndex, StoredF32, DateIndex, StoredF32, DateIndex, StoredF32>>,
    pub dateindex_to_sortino_1m: Option<LazyVecFrom2<DateIndex, StoredF32, DateIndex, StoredF32, DateIndex, StoredF32>>,
    pub dateindex_to_sortino_1y: Option<LazyVecFrom2<DateIndex, StoredF32, DateIndex, StoredF32, DateIndex, StoredF32>>,
}
