use brk_traversable::Traversable;
use brk_types::StoredF32;
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedFromHeight;

pub const TIMEFRAME_NAMES: [&str; 4] = ["1d", "1w", "1m", "1y"];

#[derive(Clone, Traversable)]
pub struct ByIndicatorTimeframe<T> {
    pub _1d: T,
    pub _1w: T,
    pub _1m: T,
    pub _1y: T,
}

impl<T> ByIndicatorTimeframe<T> {
    pub fn try_new<E>(mut create: impl FnMut(&str) -> Result<T, E>) -> Result<Self, E> {
        Ok(Self {
            _1d: create(TIMEFRAME_NAMES[0])?,
            _1w: create(TIMEFRAME_NAMES[1])?,
            _1m: create(TIMEFRAME_NAMES[2])?,
            _1y: create(TIMEFRAME_NAMES[3])?,
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&str, &mut T)> {
        [
            (TIMEFRAME_NAMES[0], &mut self._1d),
            (TIMEFRAME_NAMES[1], &mut self._1w),
            (TIMEFRAME_NAMES[2], &mut self._1m),
            (TIMEFRAME_NAMES[3], &mut self._1y),
        ]
        .into_iter()
    }
}

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
    pub line: ComputedFromHeight<StoredF32, M>,
    pub signal: ComputedFromHeight<StoredF32, M>,
    pub histogram: ComputedFromHeight<StoredF32, M>,
}

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub puell_multiple: ComputedFromHeight<StoredF32, M>,
    pub nvt: ComputedFromHeight<StoredF32, M>,

    pub rsi: ByIndicatorTimeframe<RsiChain<M>>,

    pub stoch_k: ComputedFromHeight<StoredF32, M>,
    pub stoch_d: ComputedFromHeight<StoredF32, M>,

    pub pi_cycle: ComputedFromHeight<StoredF32, M>,

    pub macd: ByIndicatorTimeframe<MacdChain<M>>,

    pub gini: ComputedFromHeight<StoredF32, M>,
}
