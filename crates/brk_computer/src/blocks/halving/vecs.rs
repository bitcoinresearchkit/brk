use brk_traversable::Traversable;
use brk_types::{HalvingEpoch, StoredF32, StoredU32};

use crate::internal::{ComputedFromHeightLast, ComputedFromDateLast};

/// Halving epoch metrics and countdown
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub epoch: ComputedFromDateLast<HalvingEpoch>,
    pub blocks_before_next_halving: ComputedFromHeightLast<StoredU32>,
    pub days_before_next_halving: ComputedFromHeightLast<StoredF32>,
}
