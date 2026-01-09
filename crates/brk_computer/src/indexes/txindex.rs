use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{StoredU64, TxIndex, Txid, Version};
use vecdb::{Database, EagerVec, ImportableVec, IterableCloneableVec, LazyVecFrom1, PcoVec};

use brk_error::Result;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub identity: LazyVecFrom1<TxIndex, TxIndex, TxIndex, Txid>,
    pub input_count: EagerVec<PcoVec<TxIndex, StoredU64>>,
    pub output_count: EagerVec<PcoVec<TxIndex, StoredU64>>,
}

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexer: &Indexer) -> Result<Self> {
        Ok(Self {
            identity: LazyVecFrom1::init(
                "txindex",
                version,
                indexer.vecs.transactions.txid.boxed_clone(),
                |index, _| Some(index),
            ),
            input_count: EagerVec::forced_import(db, "txindex_input_count", version)?,
            output_count: EagerVec::forced_import(db, "txindex_output_count", version)?,
        })
    }
}
