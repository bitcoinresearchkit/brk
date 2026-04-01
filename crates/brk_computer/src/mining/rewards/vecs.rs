use brk_traversable::Traversable;
use brk_types::{BasisPoints16, BasisPoints32, Height, Sats};
use vecdb::{EagerVec, PcoVec, Rw, StorageMode};

use crate::internal::{
    AmountPerBlockCumulative, AmountPerBlockCumulativeRolling, AmountPerBlockFull,
    LazyPercentRollingWindows, PercentPerBlock, PercentRollingWindows, RatioRollingWindows,
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub coinbase: AmountPerBlockCumulativeRolling<M>,
    pub subsidy: AmountPerBlockCumulativeRolling<M>,
    pub fees: AmountPerBlockFull<M>,
    pub output_volume: M::Stored<EagerVec<PcoVec<Height, Sats>>>,
    pub unclaimed: AmountPerBlockCumulative<M>,
    #[traversable(wrap = "fees", rename = "dominance")]
    pub fee_dominance: PercentPerBlock<BasisPoints16, M>,
    #[traversable(wrap = "fees", rename = "dominance")]
    pub fee_dominance_rolling: PercentRollingWindows<BasisPoints16, M>,
    #[traversable(wrap = "subsidy", rename = "dominance")]
    pub subsidy_dominance: PercentPerBlock<BasisPoints16, M>,
    #[traversable(wrap = "subsidy", rename = "dominance")]
    pub subsidy_dominance_rolling: LazyPercentRollingWindows<BasisPoints16>,
    #[traversable(wrap = "fees", rename = "to_subsidy_ratio")]
    pub fee_to_subsidy_ratio: RatioRollingWindows<BasisPoints32, M>,
}
