use brk_traversable::Traversable;
use brk_types::{Dollars, StoredF32};
use vecdb::{Rw, StorageMode};

use crate::internal::{
    ComputedFromHeightLast, ValueFromHeightFull,
    ValueFromHeightSumCumulative,
};

/// Coinbase/subsidy/rewards metrics
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub coinbase: ValueFromHeightFull<M>,
    pub subsidy: ValueFromHeightFull<M>,
    pub fees: ValueFromHeightFull<M>,
    pub unclaimed_rewards: ValueFromHeightSumCumulative<M>,
    pub fee_dominance: ComputedFromHeightLast<StoredF32, M>,
    pub fee_dominance_24h: ComputedFromHeightLast<StoredF32, M>,
    pub fee_dominance_7d: ComputedFromHeightLast<StoredF32, M>,
    pub fee_dominance_30d: ComputedFromHeightLast<StoredF32, M>,
    pub fee_dominance_1y: ComputedFromHeightLast<StoredF32, M>,
    pub subsidy_dominance: ComputedFromHeightLast<StoredF32, M>,
    pub subsidy_dominance_24h: ComputedFromHeightLast<StoredF32, M>,
    pub subsidy_dominance_7d: ComputedFromHeightLast<StoredF32, M>,
    pub subsidy_dominance_30d: ComputedFromHeightLast<StoredF32, M>,
    pub subsidy_dominance_1y: ComputedFromHeightLast<StoredF32, M>,
    pub subsidy_usd_1y_sma: ComputedFromHeightLast<Dollars, M>,
}
