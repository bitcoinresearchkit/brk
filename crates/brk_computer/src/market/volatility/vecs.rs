use brk_traversable::Traversable;
use brk_types::StoredF32;

use crate::grouped::{ComputedStandardDeviationVecsFromDateIndex, LazyVecsFromDateIndex};

/// Volatility and standard deviation metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_1d_returns_1w_sd: ComputedStandardDeviationVecsFromDateIndex,
    pub indexes_to_1d_returns_1m_sd: ComputedStandardDeviationVecsFromDateIndex,
    pub indexes_to_1d_returns_1y_sd: ComputedStandardDeviationVecsFromDateIndex,
    pub indexes_to_price_1w_volatility: LazyVecsFromDateIndex<StoredF32>,
    pub indexes_to_price_1m_volatility: LazyVecsFromDateIndex<StoredF32>,
    pub indexes_to_price_1y_volatility: LazyVecsFromDateIndex<StoredF32>,
}
