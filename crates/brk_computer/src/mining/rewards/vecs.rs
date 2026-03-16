use brk_traversable::Traversable;
use brk_types::{BasisPoints16, BasisPoints32, Cents};
use vecdb::{Rw, StorageMode};

use crate::internal::{
    AmountPerBlockCumulative, AmountPerBlockCumulativeWithSums, AmountPerBlockFull,
    FiatPerBlock, LazyPercentRollingWindows, PercentPerBlock, PercentRollingWindows,
    RatioRollingWindows,
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub coinbase: AmountPerBlockCumulativeWithSums<M>,
    pub subsidy: AmountPerBlockCumulative<M>,
    pub fees: AmountPerBlockFull<M>,
    pub unclaimed: AmountPerBlockCumulativeWithSums<M>,
    #[traversable(wrap = "fees", rename = "dominance")]
    pub fee_dominance: PercentPerBlock<BasisPoints16, M>,
    #[traversable(wrap = "fees", rename = "dominance")]
    pub fee_dominance_rolling: PercentRollingWindows<BasisPoints16, M>,
    #[traversable(wrap = "subsidy", rename = "dominance")]
    pub subsidy_dominance: PercentPerBlock<BasisPoints16, M>,
    #[traversable(wrap = "subsidy", rename = "dominance")]
    pub subsidy_dominance_rolling: LazyPercentRollingWindows<BasisPoints16>,
    #[traversable(wrap = "subsidy", rename = "sma_1y")]
    pub subsidy_sma_1y: FiatPerBlock<Cents, M>,
    #[traversable(wrap = "fees", rename = "to_subsidy_ratio")]
    pub fee_to_subsidy_ratio: RatioRollingWindows<BasisPoints32, M>,
}
