use brk_traversable::Traversable;
use brk_types::StoredU64;

use crate::internal::{ComputedVecsFromHeight, ComputedVecsFromTxindex};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_count: ComputedVecsFromTxindex<StoredU64>,
    pub indexes_to_utxo_count: ComputedVecsFromHeight<StoredU64>,
}
