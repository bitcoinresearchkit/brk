use brk_cohort::ByAddrType;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, StoredU64};
use vecdb::{Rw, StorageMode};

use crate::internal::{PerBlockCumulativeRolling, PercentCumulativeRolling};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    /// Per-block, per-type total output count (granular).
    pub output_count: ByAddrType<PerBlockCumulativeRolling<StoredU64, StoredU64, M>>,
    /// Per-block, per-type count of TXs containing at least one output of this type.
    pub tx_count: ByAddrType<PerBlockCumulativeRolling<StoredU64, StoredU64, M>>,
    /// Per-type tx_count as a percent of total tx count.
    pub tx_percent: ByAddrType<PercentCumulativeRolling<BasisPoints16, M>>,
}
