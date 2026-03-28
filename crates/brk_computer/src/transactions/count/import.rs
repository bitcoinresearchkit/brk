use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{StoredBool, TxIndex, Version};
use vecdb::{Database, LazyVecFrom2, ReadableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{CachedWindowStarts, PerBlockFull},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let tx_index_to_is_coinbase = LazyVecFrom2::init(
            "is_coinbase",
            version,
            indexer.vecs.transactions.height.read_only_boxed_clone(),
            indexer
                .vecs
                .transactions
                .first_tx_index
                .read_only_boxed_clone(),
            |index: TxIndex, _height, first_tx_index| StoredBool::from(index == first_tx_index),
        );

        Ok(Self {
            total: PerBlockFull::forced_import(db, "tx_count", version, indexes, cached_starts)?,
            is_coinbase: tx_index_to_is_coinbase,
        })
    }
}
