use brk_traversable::Traversable;
use brk_types::{BasisPoints16, BasisPoints32, StoredF32};
use vecdb::{Rw, StorageMode};

use crate::internal::{PerBlock, PercentPerBlock, RatioPerBlock, Windows};

#[derive(Traversable)]
pub struct RsiChain<M: StorageMode = Rw> {
    pub gains: PerBlock<StoredF32, M>,
    pub losses: PerBlock<StoredF32, M>,
    pub average_gain: PerBlock<StoredF32, M>,
    pub average_loss: PerBlock<StoredF32, M>,
    pub rsi: PercentPerBlock<BasisPoints16, M>,
    pub rsi_min: PercentPerBlock<BasisPoints16, M>,
    pub rsi_max: PercentPerBlock<BasisPoints16, M>,
    pub stoch_rsi: PercentPerBlock<BasisPoints16, M>,
    pub stoch_rsi_k: PercentPerBlock<BasisPoints16, M>,
    pub stoch_rsi_d: PercentPerBlock<BasisPoints16, M>,
}

#[derive(Traversable)]
pub struct MacdChain<M: StorageMode = Rw> {
    pub ema_fast: PerBlock<StoredF32, M>,
    pub ema_slow: PerBlock<StoredF32, M>,
    pub line: PerBlock<StoredF32, M>,
    pub signal: PerBlock<StoredF32, M>,
    pub histogram: PerBlock<StoredF32, M>,
}

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub rsi: Windows<RsiChain<M>>,

    pub stoch_k: PercentPerBlock<BasisPoints16, M>,
    pub stoch_d: PercentPerBlock<BasisPoints16, M>,

    pub pi_cycle: RatioPerBlock<BasisPoints32, M>,

    pub macd: Windows<MacdChain<M>>,
}
