use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, StoredF32};
use vecdb::{EagerVec, LazyVecFrom2, PcoVec};

use crate::internal::{ComputedDateLast, LazyBinaryDateLast};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub puell_multiple: Option<ComputedDateLast<StoredF32>>,
    pub nvt: Option<LazyBinaryDateLast<StoredF32, Dollars, Dollars>>,

    pub rsi_gains: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub rsi_losses: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub rsi_average_gain_14d: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub rsi_average_loss_14d: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub rsi_14d: LazyVecFrom2<DateIndex, StoredF32, DateIndex, StoredF32, DateIndex, StoredF32>,
    pub rsi_14d_min: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub rsi_14d_max: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub stoch_rsi: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub stoch_rsi_k: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub stoch_rsi_d: EagerVec<PcoVec<DateIndex, StoredF32>>,

    pub stoch_k: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub stoch_d: EagerVec<PcoVec<DateIndex, StoredF32>>,

    pub pi_cycle:
        Option<LazyVecFrom2<DateIndex, StoredF32, DateIndex, Dollars, DateIndex, Dollars>>,

    pub macd_line: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub macd_signal: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub macd_histogram:
        LazyVecFrom2<DateIndex, StoredF32, DateIndex, StoredF32, DateIndex, StoredF32>,

    pub gini: EagerVec<PcoVec<DateIndex, StoredF32>>,
}
