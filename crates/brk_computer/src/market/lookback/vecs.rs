use brk_traversable::Traversable;
use brk_types::Dollars;

use super::ByLookbackPeriod;
use crate::internal::ComputedVecsFromDateIndex;

/// Price lookback metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price_ago: ByLookbackPeriod<ComputedVecsFromDateIndex<Dollars>>,
}
