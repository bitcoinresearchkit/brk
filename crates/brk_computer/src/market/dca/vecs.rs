use brk_traversable::Traversable;
use brk_types::{Close, Dollars, StoredF32};

use super::{ByDcaCagr, ByDcaClass, ByDcaPeriod};
use crate::internal::{
    ComputedValueVecsFromDateIndex, ComputedVecsFromDateIndex, LazyVecsFrom2FromDateIndex,
};

/// Dollar-cost averaging metrics by time period and year class
#[derive(Clone, Traversable)]
pub struct Vecs {
    // DCA by period
    pub period_stack: ByDcaPeriod<ComputedValueVecsFromDateIndex>,
    pub period_avg_price: ByDcaPeriod<ComputedVecsFromDateIndex<Dollars>>,
    pub period_returns: ByDcaPeriod<LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>>,
    pub period_cagr: ByDcaCagr<ComputedVecsFromDateIndex<StoredF32>>,

    // Lump sum by period (for comparison with DCA)
    pub period_lump_sum_stack: ByDcaPeriod<ComputedValueVecsFromDateIndex>,

    // DCA by year class
    pub class_stack: ByDcaClass<ComputedValueVecsFromDateIndex>,
    pub class_avg_price: ByDcaClass<ComputedVecsFromDateIndex<Dollars>>,
    pub class_returns: ByDcaClass<LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>>,
}
