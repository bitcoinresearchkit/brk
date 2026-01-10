use brk_traversable::Traversable;
use brk_types::{Close, Dollars, StoredF32};

use super::{ByDcaCagr, ByDcaClass, ByDcaPeriod};
use crate::internal::{ComputedFromDateLast, LazyBinaryFromDateLast, ValueFromDateLast};

/// Dollar-cost averaging metrics by time period and year class
#[derive(Clone, Traversable)]
pub struct Vecs {
    // DCA by period - KISS types
    pub period_stack: ByDcaPeriod<ValueFromDateLast>,
    pub period_average_price: ByDcaPeriod<ComputedFromDateLast<Dollars>>,
    pub period_returns: ByDcaPeriod<LazyBinaryFromDateLast<StoredF32, Close<Dollars>, Dollars>>,
    pub period_cagr: ByDcaCagr<ComputedFromDateLast<StoredF32>>,

    // Lump sum by period (for comparison with DCA) - KISS types
    pub period_lump_sum_stack: ByDcaPeriod<ValueFromDateLast>,

    // DCA by year class - KISS types
    pub class_stack: ByDcaClass<ValueFromDateLast>,
    pub class_average_price: ByDcaClass<ComputedFromDateLast<Dollars>>,
    pub class_returns: ByDcaClass<LazyBinaryFromDateLast<StoredF32, Close<Dollars>, Dollars>>,
}
