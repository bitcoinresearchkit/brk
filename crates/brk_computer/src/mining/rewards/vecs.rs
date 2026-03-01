use brk_traversable::Traversable;
use brk_types::{Cents, StoredF32};
use vecdb::{Rw, StorageMode};

use crate::internal::{
    ComputedFromHeight, FiatFromHeight, ValueFromHeightFull,
    ValueFromHeightCumulativeSum,
};

/// Coinbase/subsidy/rewards metrics
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub coinbase: ValueFromHeightFull<M>,
    pub subsidy: ValueFromHeightFull<M>,
    pub fees: ValueFromHeightFull<M>,
    pub unclaimed_rewards: ValueFromHeightCumulativeSum<M>,
    pub fee_dominance: ComputedFromHeight<StoredF32, M>,
    pub fee_dominance_24h: ComputedFromHeight<StoredF32, M>,
    pub fee_dominance_7d: ComputedFromHeight<StoredF32, M>,
    pub fee_dominance_30d: ComputedFromHeight<StoredF32, M>,
    pub fee_dominance_1y: ComputedFromHeight<StoredF32, M>,
    pub subsidy_dominance: ComputedFromHeight<StoredF32, M>,
    pub subsidy_dominance_24h: ComputedFromHeight<StoredF32, M>,
    pub subsidy_dominance_7d: ComputedFromHeight<StoredF32, M>,
    pub subsidy_dominance_30d: ComputedFromHeight<StoredF32, M>,
    pub subsidy_dominance_1y: ComputedFromHeight<StoredF32, M>,
    pub subsidy_usd_1y_sma: FiatFromHeight<Cents, M>,
}
