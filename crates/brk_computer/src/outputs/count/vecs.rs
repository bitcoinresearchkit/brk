use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeight, ComputedFromHeightAggregated};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub total_count: ComputedFromHeightAggregated<StoredU64, M>,
    pub utxo_count: ComputedFromHeight<StoredU64, M>,
}
