use brk_traversable::Traversable;
use brk_types::{Height, StoredBool, StoredU64, TxIndex};
use vecdb::LazyVecFrom2;

use crate::internal::ComputedFromHeightFull;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub tx_count: ComputedFromHeightFull<StoredU64>,
    pub is_coinbase: LazyVecFrom2<TxIndex, StoredBool, TxIndex, Height, Height, TxIndex>,
}
