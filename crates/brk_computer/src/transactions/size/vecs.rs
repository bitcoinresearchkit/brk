use brk_traversable::Traversable;
use brk_types::{StoredU32, TxIndex, VSize, Weight};
use vecdb::LazyVecFrom2;

use crate::internal::ComputedTxDistribution;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_tx_vsize: ComputedTxDistribution<VSize>,
    pub indexes_to_tx_weight: ComputedTxDistribution<Weight>,
    // Both derive directly from eager sources (base_size, total_size) to avoid Lazy <- Lazy
    pub txindex_to_vsize: LazyVecFrom2<TxIndex, VSize, TxIndex, StoredU32, TxIndex, StoredU32>,
    pub txindex_to_weight: LazyVecFrom2<TxIndex, Weight, TxIndex, StoredU32, TxIndex, StoredU32>,
}
