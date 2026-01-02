use brk_traversable::Traversable;
use brk_types::{TxInIndex, TxOutIndex};
use vecdb::BytesVec;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub txoutindex_to_txinindex: BytesVec<TxOutIndex, TxInIndex>,
}
