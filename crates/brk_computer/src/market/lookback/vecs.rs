use brk_traversable::Traversable;

use super::ByLookbackPeriod;
use crate::internal::Price;

/// Price lookback metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    #[traversable(flatten)]
    pub price_ago: ByLookbackPeriod<Price>,
}
