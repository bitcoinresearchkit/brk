use brk_traversable::Traversable;
use brk_types::{TxInIndex, TxOutIndex};
use vecdb::{BytesVec, Rw, StorageMode};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub txin_index: M::Stored<BytesVec<TxOutIndex, TxInIndex>>,
}
