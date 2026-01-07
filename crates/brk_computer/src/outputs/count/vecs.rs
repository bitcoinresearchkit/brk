use brk_traversable::Traversable;
use brk_types::StoredU64;

use crate::internal::{ComputedBlockFull, DerivedTxFull};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_count: DerivedTxFull<StoredU64>,
    pub indexes_to_utxo_count: ComputedBlockFull<StoredU64>,
}
