use brk_cohort::ByType;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, StoredU64};
use vecdb::{Rw, StorageMode};

use super::WithOutputTypes;
use crate::internal::{PerBlockCumulativeRolling, PercentCumulativeRolling};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub output_count: WithOutputTypes<PerBlockCumulativeRolling<StoredU64, StoredU64, M>>,
    pub tx_count: WithOutputTypes<PerBlockCumulativeRolling<StoredU64, StoredU64, M>>,
    pub tx_percent: ByType<PercentCumulativeRolling<BasisPoints16, M>>,
}
