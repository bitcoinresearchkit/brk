use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Sats, StoredF32, StoredU32};
use vecdb::{EagerVec, PcoVec, Rw, StorageMode};

use super::{ByDcaCagr, ByDcaClass, ByDcaPeriod};
use crate::internal::{
    ComputedFromHeightLast, LazyBinaryFromHeightLast, Price, ValueFromHeightLast,
};

/// Dollar-cost averaging metrics by time period and year class
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    /// Per-height DCA sats contribution: sats_from_dca(close) on day boundaries, 0 otherwise.
    /// Computed once, reused by all period rolling sums.
    pub dca_sats_per_day: M::Stored<EagerVec<PcoVec<Height, Sats>>>,

    // DCA by period - KISS types
    pub period_stack: ByDcaPeriod<ValueFromHeightLast<M>>,
    pub period_average_price: ByDcaPeriod<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub period_returns: ByDcaPeriod<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,
    pub period_cagr: ByDcaCagr<ComputedFromHeightLast<StoredF32, M>>,

    // DCA by period - profitability
    pub period_days_in_profit: ByDcaPeriod<ComputedFromHeightLast<StoredU32, M>>,
    pub period_days_in_loss: ByDcaPeriod<ComputedFromHeightLast<StoredU32, M>>,
    pub period_min_return: ByDcaPeriod<ComputedFromHeightLast<StoredF32, M>>,
    pub period_max_return: ByDcaPeriod<ComputedFromHeightLast<StoredF32, M>>,

    // Lump sum by period (for comparison with DCA) - KISS types
    pub period_lump_sum_stack: ByDcaPeriod<ValueFromHeightLast<M>>,
    pub period_lump_sum_returns: ByDcaPeriod<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,

    // Lump sum by period - profitability
    pub period_lump_sum_days_in_profit: ByDcaPeriod<ComputedFromHeightLast<StoredU32, M>>,
    pub period_lump_sum_days_in_loss: ByDcaPeriod<ComputedFromHeightLast<StoredU32, M>>,
    pub period_lump_sum_min_return: ByDcaPeriod<ComputedFromHeightLast<StoredF32, M>>,
    pub period_lump_sum_max_return: ByDcaPeriod<ComputedFromHeightLast<StoredF32, M>>,

    // DCA by year class - KISS types
    pub class_stack: ByDcaClass<ValueFromHeightLast<M>>,
    pub class_average_price: ByDcaClass<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub class_returns: ByDcaClass<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,

    // DCA by year class - profitability
    pub class_days_in_profit: ByDcaClass<ComputedFromHeightLast<StoredU32, M>>,
    pub class_days_in_loss: ByDcaClass<ComputedFromHeightLast<StoredU32, M>>,
    pub class_min_return: ByDcaClass<ComputedFromHeightLast<StoredF32, M>>,
    pub class_max_return: ByDcaClass<ComputedFromHeightLast<StoredF32, M>>,
}
