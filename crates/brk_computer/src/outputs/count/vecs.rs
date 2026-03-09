use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlock, ComputedPerBlockAggregated};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub total_count: ComputedPerBlockAggregated<StoredU64, M>,
    pub utxo_count: ComputedPerBlock<StoredU64, M>,
}
