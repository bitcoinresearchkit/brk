use brk_traversable::Traversable;
use brk_types::StoredF64;

use crate::internal::ComputedVecsDateAverage;

/// Velocity metrics (annualized volume / circulating supply)
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_btc: ComputedVecsDateAverage<StoredF64>,
    pub indexes_to_usd: Option<ComputedVecsDateAverage<StoredF64>>,
}
