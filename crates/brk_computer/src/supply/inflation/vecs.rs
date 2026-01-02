use brk_traversable::Traversable;
use brk_types::StoredF32;

use crate::internal::ComputedVecsFromDateIndex;

/// Inflation rate metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes: ComputedVecsFromDateIndex<StoredF32>,
}
