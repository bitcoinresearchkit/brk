use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{StoredU64, TxIndex, Txid, Version};
use vecdb::{Database, EagerVec, ImportableVec, ReadableCloneableVec, LazyVecFrom1, PcoVec, Rw, StorageMode};

use brk_error::Result;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub identity: LazyVecFrom1<TxIndex, TxIndex, TxIndex, Txid>,
    pub input_count: M::Stored<EagerVec<PcoVec<TxIndex, StoredU64>>>,
    pub output_count: M::Stored<EagerVec<PcoVec<TxIndex, StoredU64>>>,
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version, indexer: &Indexer) -> Result<Self> {
        Ok(Self {
            identity: LazyVecFrom1::init(
                "txindex",
                version,
                indexer.vecs.transactions.txid.read_only_boxed_clone(),
                |index, _| index,
            ),
            input_count: EagerVec::forced_import(db, "input_count", version)?,
            output_count: EagerVec::forced_import(db, "output_count", version)?,
        })
    }
}
