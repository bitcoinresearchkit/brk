use brk_traversable::Traversable;
use brk_types::{Sats, TxInIndex, TxOutIndex};
use vecdb::PcoVec;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub txoutindex: PcoVec<TxInIndex, TxOutIndex>,
    pub value: PcoVec<TxInIndex, Sats>,
}
