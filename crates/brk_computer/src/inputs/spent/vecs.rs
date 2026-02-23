use brk_traversable::Traversable;
use brk_types::{Sats, TxInIndex, TxOutIndex};
use vecdb::{PcoVec, Rw, StorageMode};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub txoutindex: M::Stored<PcoVec<TxInIndex, TxOutIndex>>,
    pub value: M::Stored<PcoVec<TxInIndex, Sats>>,
}
