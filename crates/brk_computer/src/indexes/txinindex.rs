use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{OutPoint, TxInIndex, Version};
use vecdb::{IterableCloneableVec, LazyVecFrom1};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub identity: LazyVecFrom1<TxInIndex, TxInIndex, TxInIndex, OutPoint>,
}

impl Vecs {
    pub fn forced_import(version: Version, indexer: &Indexer) -> Self {
        Self {
            identity: LazyVecFrom1::init(
                "txinindex",
                version,
                indexer.vecs.inputs.outpoint.boxed_clone(),
                |index, _| Some(index),
            ),
        }
    }
}
