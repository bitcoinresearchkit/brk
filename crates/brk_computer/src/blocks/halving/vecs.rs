use brk_traversable::Traversable;
use brk_types::{HalvingEpoch, StoredF32, StoredU32};

use crate::internal::{ComputedVecsFromDateIndex, ComputedVecsFromHeight};

/// Halving epoch metrics and countdown
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_halvingepoch: ComputedVecsFromDateIndex<HalvingEpoch>,
    pub indexes_to_blocks_before_next_halving: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_days_before_next_halving: ComputedVecsFromHeight<StoredF32>,
}
