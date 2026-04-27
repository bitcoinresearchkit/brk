use brk_traversable::Traversable;
use brk_types::{BasisPoints16, BasisPoints32, Height, Sats};
use vecdb::{EagerVec, PcoVec, Rw, StorageMode};

use crate::internal::{
    LazyPercentCumulativeRolling, PercentCumulativeRolling, RatioRollingWindows,
    ValuePerBlockCumulative, ValuePerBlockCumulativeRolling, ValuePerBlockFull,
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub coinbase: ValuePerBlockCumulativeRolling<M>,
    pub subsidy: ValuePerBlockCumulativeRolling<M>,
    pub fees: ValuePerBlockFull<M>,
    pub output_volume: M::Stored<EagerVec<PcoVec<Height, Sats>>>,
    pub unclaimed: ValuePerBlockCumulative<M>,
    #[traversable(wrap = "fees", rename = "dominance")]
    pub fee_dominance: PercentCumulativeRolling<BasisPoints16, M>,
    #[traversable(wrap = "subsidy", rename = "dominance")]
    pub subsidy_dominance: LazyPercentCumulativeRolling<BasisPoints16>,
    #[traversable(wrap = "fees", rename = "to_subsidy_ratio")]
    pub fee_to_subsidy_ratio: RatioRollingWindows<BasisPoints32, M>,
}
