use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{Sats, TxOutIndex, Version};
use vecdb::{IterableCloneableVec, LazyVecFrom1};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub identity: LazyVecFrom1<TxOutIndex, TxOutIndex, TxOutIndex, Sats>,
}

impl Vecs {
    pub fn forced_import(version: Version, indexer: &Indexer) -> Self {
        Self {
            identity: LazyVecFrom1::init(
                "txoutindex",
                version,
                indexer.vecs.outputs.value.boxed_clone(),
                |index, _| Some(index),
            ),
        }
    }
}
