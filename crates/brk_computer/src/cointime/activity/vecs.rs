use brk_traversable::Traversable;
use brk_types::StoredF64;

use crate::internal::ComputedVecsFromHeight;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_coinblocks_created: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_coinblocks_stored: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_liveliness: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_vaultedness: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_activity_to_vaultedness_ratio: ComputedVecsFromHeight<StoredF64>,
}
