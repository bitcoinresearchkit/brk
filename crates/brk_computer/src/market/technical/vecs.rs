use brk_traversable::Traversable;
use brk_types::{BasisPoints16, BasisPoints32, StoredF32};
use vecdb::{Rw, StorageMode};

use crate::internal::{PerBlock, PercentPerBlock, RatioPerBlock, WindowsTo1m};

#[derive(Traversable)]
pub struct RsiChain<M: StorageMode = Rw> {
    #[traversable(hidden)]
    pub gains: PerBlock<StoredF32, M>,
    #[traversable(hidden)]
    pub losses: PerBlock<StoredF32, M>,
    #[traversable(hidden)]
    pub average_gain: PerBlock<StoredF32, M>,
    #[traversable(hidden)]
    pub average_loss: PerBlock<StoredF32, M>,
    pub rsi: PercentPerBlock<BasisPoints16, M>,
    #[traversable(hidden)]
    pub rsi_min: PercentPerBlock<BasisPoints16, M>,
    #[traversable(hidden)]
    pub rsi_max: PercentPerBlock<BasisPoints16, M>,
    #[traversable(hidden)]
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
    pub rsi: WindowsTo1m<RsiChain<M>>,

    pub pi_cycle: RatioPerBlock<BasisPoints32, M>,

    pub macd: WindowsTo1m<MacdChain<M>>,
}
