use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, StoredF32};
use vecdb::{EagerVec, LazyVecFrom2, PcoVec};

use crate::internal::{BinaryDateLast, ComputedDateLast};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_puell_multiple: Option<ComputedDateLast<StoredF32>>,
    pub indexes_to_nvt: Option<BinaryDateLast<StoredF32, Dollars, Dollars>>,

    pub dateindex_to_rsi_gains: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_rsi_losses: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_rsi_average_gain_14d: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_rsi_average_loss_14d: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_rsi_14d:
        LazyVecFrom2<DateIndex, StoredF32, DateIndex, StoredF32, DateIndex, StoredF32>,

    pub dateindex_to_rsi_14d_min: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_rsi_14d_max: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_stoch_rsi: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_stoch_rsi_k: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_stoch_rsi_d: EagerVec<PcoVec<DateIndex, StoredF32>>,

    pub dateindex_to_stoch_k: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_stoch_d: EagerVec<PcoVec<DateIndex, StoredF32>>,

    pub dateindex_to_pi_cycle:
        Option<LazyVecFrom2<DateIndex, StoredF32, DateIndex, Dollars, DateIndex, Dollars>>,

    pub dateindex_to_macd_line: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_macd_signal: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_macd_histogram:
        LazyVecFrom2<DateIndex, StoredF32, DateIndex, StoredF32, DateIndex, StoredF32>,

    pub dateindex_to_gini: EagerVec<PcoVec<DateIndex, StoredF32>>,
}
