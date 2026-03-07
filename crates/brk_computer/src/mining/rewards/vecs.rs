use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Cents};
use vecdb::{Rw, StorageMode};

use crate::internal::{
    FiatFromHeight, PercentFromHeight, PercentRollingWindows, ValueFromHeightCumulative,
    ValueFromHeightCumulativeSum, ValueFromHeightFull,
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub coinbase: ValueFromHeightCumulativeSum<M>,
    pub subsidy: ValueFromHeightCumulative<M>,
    pub fees: ValueFromHeightFull<M>,
    pub unclaimed_rewards: ValueFromHeightCumulativeSum<M>,
    pub fee_dominance: PercentFromHeight<BasisPoints16, M>,
    pub fee_dominance_rolling: PercentRollingWindows<BasisPoints16, M>,
    pub subsidy_dominance: PercentFromHeight<BasisPoints16, M>,
    pub subsidy_dominance_rolling: PercentRollingWindows<BasisPoints16, M>,
    pub subsidy_sma_1y: FiatFromHeight<Cents, M>,
}
