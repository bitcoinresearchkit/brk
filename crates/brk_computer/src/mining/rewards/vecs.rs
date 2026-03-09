use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Cents};
use vecdb::{Rw, StorageMode};

use crate::internal::{
    AmountPerBlockCumulative, AmountPerBlockCumulativeSum, AmountPerBlockFull,
    FiatPerBlock, PercentPerBlock, PercentRollingWindows,
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub coinbase: AmountPerBlockCumulativeSum<M>,
    pub subsidy: AmountPerBlockCumulative<M>,
    pub fees: AmountPerBlockFull<M>,
    pub unclaimed_rewards: AmountPerBlockCumulativeSum<M>,
    pub fee_dominance: PercentPerBlock<BasisPoints16, M>,
    pub fee_dominance_rolling: PercentRollingWindows<BasisPoints16, M>,
    pub subsidy_dominance: PercentPerBlock<BasisPoints16, M>,
    pub subsidy_dominance_rolling: PercentRollingWindows<BasisPoints16, M>,
    pub subsidy_sma_1y: FiatPerBlock<Cents, M>,
}
