use brk_traversable::Traversable;
use brk_types::{BasisPoints16, StoredF32};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeight, PercentFromHeight, Windows};

#[derive(Traversable)]
pub struct RsiChain<M: StorageMode = Rw> {
    pub gains: ComputedFromHeight<StoredF32, M>,
    pub losses: ComputedFromHeight<StoredF32, M>,
    pub average_gain: ComputedFromHeight<StoredF32, M>,
    pub average_loss: ComputedFromHeight<StoredF32, M>,
    pub rsi: ComputedFromHeight<StoredF32, M>,
    pub rsi_min: ComputedFromHeight<StoredF32, M>,
    pub rsi_max: ComputedFromHeight<StoredF32, M>,
    pub stoch_rsi: ComputedFromHeight<StoredF32, M>,
    pub stoch_rsi_k: ComputedFromHeight<StoredF32, M>,
    pub stoch_rsi_d: ComputedFromHeight<StoredF32, M>,
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
    pub puell_multiple: ComputedFromHeight<StoredF32, M>,
    pub nvt: ComputedFromHeight<StoredF32, M>,

    pub rsi: Windows<RsiChain<M>>,

    pub stoch_k: ComputedFromHeight<StoredF32, M>,
    pub stoch_d: ComputedFromHeight<StoredF32, M>,

    pub pi_cycle: ComputedFromHeight<StoredF32, M>,

    pub macd: Windows<MacdChain<M>>,

    pub gini: PercentFromHeight<BasisPoints16, M>,
}
