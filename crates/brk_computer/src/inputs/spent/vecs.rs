use brk_traversable::Traversable;
use brk_types::{Sats, TxInIndex, TxOutIndex};
use vecdb::PcoVec;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub txinindex_to_txoutindex: PcoVec<TxInIndex, TxOutIndex>,
    pub txinindex_to_value: PcoVec<TxInIndex, Sats>,
}
