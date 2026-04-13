use brk_cohort::ByAddrType;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, StoredU64};
use vecdb::{Rw, StorageMode};

use crate::internal::{PerBlockCumulativeRolling, PercentCumulativeRolling};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub by_type: ByAddrType<PerBlockCumulativeRolling<StoredU64, StoredU64, M>>,
    pub percent: ByAddrType<PercentCumulativeRolling<BasisPoints16, M>>,
}
