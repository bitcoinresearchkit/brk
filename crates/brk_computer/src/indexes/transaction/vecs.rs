use brk_traversable::Traversable;
use brk_types::{OutPoint, Sats, StoredU64, TxInIndex, TxIndex, TxOutIndex, Txid};
use vecdb::{EagerVec, LazyVecFrom1, PcoVec};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub txindex_to_input_count: EagerVec<PcoVec<TxIndex, StoredU64>>,
    pub txindex_to_output_count: EagerVec<PcoVec<TxIndex, StoredU64>>,
    pub txindex_to_txindex: LazyVecFrom1<TxIndex, TxIndex, TxIndex, Txid>,
    pub txinindex_to_txinindex: LazyVecFrom1<TxInIndex, TxInIndex, TxInIndex, OutPoint>,
    pub txoutindex_to_txoutindex: LazyVecFrom1<TxOutIndex, TxOutIndex, TxOutIndex, Sats>,
}
