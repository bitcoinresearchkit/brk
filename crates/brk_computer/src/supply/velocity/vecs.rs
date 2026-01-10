use brk_traversable::Traversable;
use brk_types::StoredF64;

use crate::internal::ComputedFromDateAverage;

/// Velocity metrics (annualized volume / circulating supply)
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub btc: ComputedFromDateAverage<StoredF64>,
    pub usd: Option<ComputedFromDateAverage<StoredF64>>,
}
