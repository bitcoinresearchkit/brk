use brk_traversable::Traversable;
use brk_types::{BasisPoints16, BasisPoints32, StoredF32};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlock, RatioPerBlock, PercentPerBlock, Windows};

#[derive(Traversable)]
pub struct RsiChain<M: StorageMode = Rw> {
    pub gains: ComputedPerBlock<StoredF32, M>,
    pub losses: ComputedPerBlock<StoredF32, M>,
    pub average_gain: ComputedPerBlock<StoredF32, M>,
    pub average_loss: ComputedPerBlock<StoredF32, M>,
    pub rsi: PercentPerBlock<BasisPoints16, M>,
    pub rsi_min: PercentPerBlock<BasisPoints16, M>,
    pub rsi_max: PercentPerBlock<BasisPoints16, M>,
    pub stoch_rsi: PercentPerBlock<BasisPoints16, M>,
    pub stoch_rsi_k: PercentPerBlock<BasisPoints16, M>,
    pub stoch_rsi_d: PercentPerBlock<BasisPoints16, M>,
}

#[derive(Traversable)]
pub struct MacdChain<M: StorageMode = Rw> {
    pub ema_fast: ComputedPerBlock<StoredF32, M>,
    pub ema_slow: ComputedPerBlock<StoredF32, M>,
    pub line: ComputedPerBlock<StoredF32, M>,
    pub signal: ComputedPerBlock<StoredF32, M>,
    pub histogram: ComputedPerBlock<StoredF32, M>,
}

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub puell_multiple: RatioPerBlock<BasisPoints32, M>,
    pub nvt: RatioPerBlock<BasisPoints32, M>,

    pub rsi: Windows<RsiChain<M>>,

    pub stoch_k: PercentPerBlock<BasisPoints16, M>,
    pub stoch_d: PercentPerBlock<BasisPoints16, M>,

    pub pi_cycle: RatioPerBlock<BasisPoints32, M>,

    pub macd: Windows<MacdChain<M>>,

    pub gini: PercentPerBlock<BasisPoints16, M>,
}
