use brk_traversable::Traversable;
use brk_types::{Height, StoredBool, StoredU64, TxIndex};
use vecdb::LazyVecFrom2;

use crate::internal::ComputedBlockFull;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_tx_count: ComputedBlockFull<StoredU64>,
    pub txindex_to_is_coinbase: LazyVecFrom2<TxIndex, StoredBool, TxIndex, Height, Height, TxIndex>,
}
