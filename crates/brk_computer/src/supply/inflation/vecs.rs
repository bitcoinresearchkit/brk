use brk_traversable::Traversable;
use brk_types::StoredF32;

use crate::internal::ComputedVecsDateAverage;

/// Inflation rate metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes: ComputedVecsDateAverage<StoredF32>,
}
