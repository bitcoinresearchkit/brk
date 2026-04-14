use brk_cohort::SpendableType;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, StoredU64};
use vecdb::{Rw, StorageMode};

use super::WithInputTypes;
use crate::internal::{PerBlockCumulativeRolling, PercentCumulativeRolling};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub input_count: WithInputTypes<PerBlockCumulativeRolling<StoredU64, StoredU64, M>>,
    pub input_share: SpendableType<PercentCumulativeRolling<BasisPoints16, M>>,
    pub tx_count: WithInputTypes<PerBlockCumulativeRolling<StoredU64, StoredU64, M>>,
    pub tx_share: SpendableType<PercentCumulativeRolling<BasisPoints16, M>>,
}
