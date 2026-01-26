use brk_traversable::Traversable;
use brk_types::{Close, Dollars, StoredF32, StoredU32};

use super::{ByDcaCagr, ByDcaClass, ByDcaPeriod};
use crate::internal::{ComputedFromDateLast, LazyBinaryFromDateLast, Price, ValueFromDateLast};

/// Dollar-cost averaging metrics by time period and year class
#[derive(Clone, Traversable)]
pub struct Vecs {
    // DCA by period - KISS types
    pub period_stack: ByDcaPeriod<ValueFromDateLast>,
    pub period_average_price: ByDcaPeriod<Price>,
    pub period_returns: ByDcaPeriod<LazyBinaryFromDateLast<StoredF32, Close<Dollars>, Dollars>>,
    pub period_cagr: ByDcaCagr<ComputedFromDateLast<StoredF32>>,

    // DCA by period - profitability
    pub period_days_in_profit: ByDcaPeriod<ComputedFromDateLast<StoredU32>>,
    pub period_days_in_loss: ByDcaPeriod<ComputedFromDateLast<StoredU32>>,
    pub period_max_drawdown: ByDcaPeriod<ComputedFromDateLast<StoredF32>>,
    pub period_max_return: ByDcaPeriod<ComputedFromDateLast<StoredF32>>,

    // Lump sum by period (for comparison with DCA) - KISS types
    pub period_lump_sum_stack: ByDcaPeriod<ValueFromDateLast>,
    pub period_lump_sum_returns: ByDcaPeriod<LazyBinaryFromDateLast<StoredF32, Close<Dollars>, Dollars>>,

    // Lump sum by period - profitability
    pub period_lump_sum_days_in_profit: ByDcaPeriod<ComputedFromDateLast<StoredU32>>,
    pub period_lump_sum_days_in_loss: ByDcaPeriod<ComputedFromDateLast<StoredU32>>,
    pub period_lump_sum_max_drawdown: ByDcaPeriod<ComputedFromDateLast<StoredF32>>,
    pub period_lump_sum_max_return: ByDcaPeriod<ComputedFromDateLast<StoredF32>>,

    // DCA by year class - KISS types
    pub class_stack: ByDcaClass<ValueFromDateLast>,
    pub class_average_price: ByDcaClass<Price>,
    pub class_returns: ByDcaClass<LazyBinaryFromDateLast<StoredF32, Close<Dollars>, Dollars>>,

    // DCA by year class - profitability
    pub class_days_in_profit: ByDcaClass<ComputedFromDateLast<StoredU32>>,
    pub class_days_in_loss: ByDcaClass<ComputedFromDateLast<StoredU32>>,
    pub class_max_drawdown: ByDcaClass<ComputedFromDateLast<StoredF32>>,
    pub class_max_return: ByDcaClass<ComputedFromDateLast<StoredF32>>,
}
