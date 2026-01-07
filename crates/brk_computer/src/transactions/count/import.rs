use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{StoredBool, TxIndex, Version};
use vecdb::{Database, IterableCloneableVec, LazyVecFrom2};

use super::Vecs;
use crate::{
    indexes,
    internal::ComputedBlockFull,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let txindex_to_is_coinbase = LazyVecFrom2::init(
            "is_coinbase",
            version,
            indexer.vecs.tx.txindex_to_height.boxed_clone(),
            indexer.vecs.tx.height_to_first_txindex.boxed_clone(),
            |index: TxIndex, txindex_to_height_iter, height_to_first_txindex_iter| {
                txindex_to_height_iter.get(index).map(|height| {
                    let txindex = height_to_first_txindex_iter.get_unwrap(height);
                    StoredBool::from(index == txindex)
                })
            },
        );

        Ok(Self {
            indexes_to_tx_count: ComputedBlockFull::forced_import(
                db,
                "tx_count",
                version,
                indexes,
            )?,
            txindex_to_is_coinbase,
        })
    }
}
