use brk_traversable::Traversable;
use brk_types::StoredF64;

use crate::internal::ComputedVecsFromHeight;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_cointime_value_destroyed: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_cointime_value_created: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_cointime_value_stored: ComputedVecsFromHeight<StoredF64>,
}
