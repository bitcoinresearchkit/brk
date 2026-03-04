use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, Cents, Height, Sats};
use vecdb::{EagerVec, PcoVec, Rw, StorageMode};

use super::{ByDcaCagr, ByDcaClass, ByDcaPeriod};
use crate::internal::{ComputedFromHeight, PercentFromHeight, Price, ValueFromHeight};
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    /// Per-height DCA sats contribution: sats_from_dca(close) on day boundaries, 0 otherwise.
    /// Computed once, reused by all period rolling sums.
    pub dca_sats_per_day: M::Stored<EagerVec<PcoVec<Height, Sats>>>,

    // DCA by period
    pub period_stack: ByDcaPeriod<ValueFromHeight<M>>,
    pub period_cost_basis: ByDcaPeriod<Price<ComputedFromHeight<Cents, M>>>,
    pub period_return: ByDcaPeriod<PercentFromHeight<BasisPointsSigned32, M>>,
    pub period_cagr: ByDcaCagr<PercentFromHeight<BasisPointsSigned32, M>>,

    // Lump sum by period (for comparison with DCA)
    pub period_lump_sum_stack: ByDcaPeriod<ValueFromHeight<M>>,
    pub period_lump_sum_return: ByDcaPeriod<PercentFromHeight<BasisPointsSigned32, M>>,

    // DCA by year class
    pub class_stack: ByDcaClass<ValueFromHeight<M>>,
    pub class_cost_basis: ByDcaClass<Price<ComputedFromHeight<Cents, M>>>,
    pub class_return: ByDcaClass<PercentFromHeight<BasisPointsSigned32, M>>,
}
