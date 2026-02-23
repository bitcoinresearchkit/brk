use brk_traversable::Traversable;
use brk_types::{Height, StoredBool, StoredU64, TxIndex};
use vecdb::{LazyVecFrom2, Rw, StorageMode};

use crate::internal::ComputedFromHeightFull;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub tx_count: ComputedFromHeightFull<StoredU64, M>,
    pub is_coinbase: LazyVecFrom2<TxIndex, StoredBool, TxIndex, Height, Height, TxIndex>,
}
