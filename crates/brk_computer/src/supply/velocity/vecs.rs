use brk_traversable::Traversable;
use brk_types::StoredF64;

use crate::internal::ComputedVecsFromDateIndex;

/// Velocity metrics (annualized volume / circulating supply)
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_btc: ComputedVecsFromDateIndex<StoredF64>,
    pub indexes_to_usd: Option<ComputedVecsFromDateIndex<StoredF64>>,
}
