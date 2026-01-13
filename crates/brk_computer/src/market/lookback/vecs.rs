use brk_traversable::Traversable;
use brk_types::Dollars;

use super::ByLookbackPeriod;
use crate::internal::ComputedFromDateLast;

/// Price lookback metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    #[traversable(flatten)]
    pub price_ago: ByLookbackPeriod<ComputedFromDateLast<Dollars>>,
}
