use brk_traversable::Traversable;
use brk_types::{Cents, Height, Sats, StoredF32};
use vecdb::{EagerVec, PcoVec, Rw, StorageMode};

use super::{ByDcaCagr, ByDcaClass, ByDcaPeriod};
use crate::internal::{ComputedFromHeight, Price, ValueFromHeight};

/// Dollar-cost averaging metrics by time period and year class
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    /// Per-height DCA sats contribution: sats_from_dca(close) on day boundaries, 0 otherwise.
    /// Computed once, reused by all period rolling sums.
    pub dca_sats_per_day: M::Stored<EagerVec<PcoVec<Height, Sats>>>,

    // DCA by period
    pub period_stack: ByDcaPeriod<ValueFromHeight<M>>,
    pub period_average_price: ByDcaPeriod<Price<ComputedFromHeight<Cents, M>>>,
    pub period_returns: ByDcaPeriod<ComputedFromHeight<StoredF32, M>>,
    pub period_cagr: ByDcaCagr<ComputedFromHeight<StoredF32, M>>,

    // Lump sum by period (for comparison with DCA)
    pub period_lump_sum_stack: ByDcaPeriod<ValueFromHeight<M>>,
    pub period_lump_sum_returns: ByDcaPeriod<ComputedFromHeight<StoredF32, M>>,

    // DCA by year class
    pub class_stack: ByDcaClass<ValueFromHeight<M>>,
    pub class_average_price: ByDcaClass<Price<ComputedFromHeight<Cents, M>>>,
    pub class_returns: ByDcaClass<ComputedFromHeight<StoredF32, M>>,
}
