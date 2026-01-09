use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, StoredF32};
use vecdb::{EagerVec, PcoVec};

use crate::internal::{ComputedDateLast, ValueBlockFull, ValueBlockHeight, ValueBlockSumCum};

/// Coinbase/subsidy/rewards metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub _24h_coinbase_sum: ValueBlockHeight,
    pub coinbase: ValueBlockFull,
    pub subsidy: ValueBlockFull,
    pub unclaimed_rewards: ValueBlockSumCum,
    pub fee_dominance: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub subsidy_dominance: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub subsidy_usd_1y_sma: Option<ComputedDateLast<Dollars>>,
}
