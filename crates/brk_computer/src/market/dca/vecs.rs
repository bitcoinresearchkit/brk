use brk_traversable::Traversable;
use brk_types::{Close, Dollars, StoredF32};

use super::{ByDcaCagr, ByDcaClass, ByDcaPeriod};
use crate::internal::{ComputedDateLast, LazyBinaryDateLast, ValueDateLast};

/// Dollar-cost averaging metrics by time period and year class
#[derive(Clone, Traversable)]
pub struct Vecs {
    // DCA by period - KISS types
    pub period_stack: ByDcaPeriod<ValueDateLast>,
    pub period_average_price: ByDcaPeriod<ComputedDateLast<Dollars>>,
    pub period_returns: ByDcaPeriod<LazyBinaryDateLast<StoredF32, Close<Dollars>, Dollars>>,
    pub period_cagr: ByDcaCagr<ComputedDateLast<StoredF32>>,

    // Lump sum by period (for comparison with DCA) - KISS types
    pub period_lump_sum_stack: ByDcaPeriod<ValueDateLast>,

    // DCA by year class - KISS types
    pub class_stack: ByDcaClass<ValueDateLast>,
    pub class_average_price: ByDcaClass<ComputedDateLast<Dollars>>,
    pub class_returns: ByDcaClass<LazyBinaryDateLast<StoredF32, Close<Dollars>, Dollars>>,
}
