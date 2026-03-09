use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Cents};
use vecdb::{Rw, StorageMode};

use crate::internal::{
    AmountFromHeightCumulative, AmountFromHeightCumulativeSum, AmountFromHeightFull,
    FiatFromHeight, PercentFromHeight, PercentRollingWindows,
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub coinbase: AmountFromHeightCumulativeSum<M>,
    pub subsidy: AmountFromHeightCumulative<M>,
    pub fees: AmountFromHeightFull<M>,
    pub unclaimed_rewards: AmountFromHeightCumulativeSum<M>,
    pub fee_dominance: PercentFromHeight<BasisPoints16, M>,
    pub fee_dominance_rolling: PercentRollingWindows<BasisPoints16, M>,
    pub subsidy_dominance: PercentFromHeight<BasisPoints16, M>,
    pub subsidy_dominance_rolling: PercentRollingWindows<BasisPoints16, M>,
    pub subsidy_sma_1y: FiatFromHeight<Cents, M>,
}
