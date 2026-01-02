use brk_traversable::Traversable;
use brk_types::StoredU64;

use crate::internal::ComputedVecsFromHeight;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_tx_v1: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_tx_v2: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_tx_v3: ComputedVecsFromHeight<StoredU64>,
}
