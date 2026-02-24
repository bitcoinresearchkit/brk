use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{StoredBool, TxIndex, Version};
use vecdb::{Database, LazyVecFrom2, ReadableCloneableVec};

use super::Vecs;
use crate::{indexes, internal::ComputedFromHeightCumFull};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let txindex_to_is_coinbase = LazyVecFrom2::init(
            "is_coinbase",
            version,
            indexer.vecs.transactions.height.read_only_boxed_clone(),
            indexer.vecs.transactions.first_txindex.read_only_boxed_clone(),
            |index: TxIndex, _height, first_txindex| {
                StoredBool::from(index == first_txindex)
            },
        );

        Ok(Self {
            tx_count: ComputedFromHeightCumFull::forced_import(
                db, "tx_count", version, indexes,
            )?,
            is_coinbase: txindex_to_is_coinbase,
        })
    }
}
