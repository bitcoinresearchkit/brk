use brk_traversable::Traversable;
use brk_types::{TxInIndex, TxOutIndex};
use vecdb::{BytesVec, Rw, StorageMode};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub txinindex: M::Stored<BytesVec<TxOutIndex, TxInIndex>>,
}
