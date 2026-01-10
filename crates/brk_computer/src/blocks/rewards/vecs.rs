use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, StoredF32};
use vecdb::{EagerVec, PcoVec};

use crate::internal::{ComputedFromDateLast, ValueFromHeightFull, ValueHeight, ValueFromHeightSumCum};

/// Coinbase/subsidy/rewards metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub _24h_coinbase_sum: ValueHeight,
    pub coinbase: ValueFromHeightFull,
    pub subsidy: ValueFromHeightFull,
    pub unclaimed_rewards: ValueFromHeightSumCum,
    pub fee_dominance: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub subsidy_dominance: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub subsidy_usd_1y_sma: Option<ComputedFromDateLast<Dollars>>,
}
