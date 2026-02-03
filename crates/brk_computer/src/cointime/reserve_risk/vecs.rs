use brk_traversable::Traversable;
use brk_types::{DateIndex, StoredF64};
use vecdb::{EagerVec, PcoVec};

use crate::internal::ComputedFromDateLast;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub vocdd_365d_median: EagerVec<PcoVec<DateIndex, StoredF64>>,
    pub hodl_bank: EagerVec<PcoVec<DateIndex, StoredF64>>,
    pub reserve_risk: Option<ComputedFromDateLast<StoredF64>>,
}
