use brk_traversable::Traversable;
use brk_types::{HalvingEpoch, StoredF32, StoredU32};

use crate::internal::{ComputedBlockLast, ComputedDateLast};

/// Halving epoch metrics and countdown
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub halvingepoch: ComputedDateLast<HalvingEpoch>,
    pub blocks_before_next_halving: ComputedBlockLast<StoredU32>,
    pub days_before_next_halving: ComputedBlockLast<StoredF32>,
}
