use brk_traversable::Traversable;
use brk_types::{BasisPoints16, StoredF32};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeight, ComputedFromHeightRatio, PercentFromHeight, Windows};

#[derive(Traversable)]
pub struct RsiChain<M: StorageMode = Rw> {
    pub gains: ComputedFromHeight<StoredF32, M>,
    pub losses: ComputedFromHeight<StoredF32, M>,
    pub average_gain: ComputedFromHeight<StoredF32, M>,
    pub average_loss: ComputedFromHeight<StoredF32, M>,
    pub rsi: PercentFromHeight<BasisPoints16, M>,
    pub rsi_min: PercentFromHeight<BasisPoints16, M>,
    pub rsi_max: PercentFromHeight<BasisPoints16, M>,
    pub stoch_rsi: PercentFromHeight<BasisPoints16, M>,
    pub stoch_rsi_k: PercentFromHeight<BasisPoints16, M>,
    pub stoch_rsi_d: PercentFromHeight<BasisPoints16, M>,
}

#[derive(Traversable)]
pub struct MacdChain<M: StorageMode = Rw> {
    pub ema_fast: ComputedFromHeight<StoredF32, M>,
    pub ema_slow: ComputedFromHeight<StoredF32, M>,
    pub line: ComputedFromHeight<StoredF32, M>,
    pub signal: ComputedFromHeight<StoredF32, M>,
    pub histogram: ComputedFromHeight<StoredF32, M>,
}

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub puell_multiple: ComputedFromHeightRatio<M>,
    pub nvt: ComputedFromHeightRatio<M>,

    pub rsi: Windows<RsiChain<M>>,

    pub stoch_k: PercentFromHeight<BasisPoints16, M>,
    pub stoch_d: PercentFromHeight<BasisPoints16, M>,

    pub pi_cycle: ComputedFromHeightRatio<M>,

    pub macd: Windows<MacdChain<M>>,

    pub gini: PercentFromHeight<BasisPoints16, M>,
}
