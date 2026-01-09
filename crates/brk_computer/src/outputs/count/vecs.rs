use brk_traversable::Traversable;
use brk_types::StoredU64;

use crate::internal::{ComputedBlockFull, DerivedTxFull};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub total_count: DerivedTxFull<StoredU64>,
    pub utxo_count: ComputedBlockFull<StoredU64>,
}
