use brk_traversable::Traversable;
use brk_types::{Height, StoredF64};
use vecdb::{EagerVec, PcoVec, Rw, StorageMode};

use crate::internal::ComputedFromHeightLast;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vocdd_365d_median: M::Stored<EagerVec<PcoVec<Height, StoredF64>>>,
    pub hodl_bank: M::Stored<EagerVec<PcoVec<Height, StoredF64>>>,
    pub reserve_risk: ComputedFromHeightLast<StoredF64, M>,
}
