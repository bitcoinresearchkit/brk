use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, Height, Sats, StoredF32};
use vecdb::{EagerVec, PcoVec};

use crate::grouped::{ComputedValueVecsFromHeight, ComputedVecsFromDateIndex};

/// Coinbase/subsidy/rewards metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub height_to_24h_coinbase_sum: EagerVec<PcoVec<Height, Sats>>,
    pub height_to_24h_coinbase_usd_sum: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_coinbase: ComputedValueVecsFromHeight,
    pub indexes_to_subsidy: ComputedValueVecsFromHeight,
    pub indexes_to_unclaimed_rewards: ComputedValueVecsFromHeight,
    pub dateindex_to_fee_dominance: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_subsidy_dominance: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub indexes_to_subsidy_usd_1y_sma: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub indexes_to_puell_multiple: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_inflation_rate: ComputedVecsFromDateIndex<StoredF32>,
}
