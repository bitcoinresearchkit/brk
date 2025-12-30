use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec, IterableCloneableVec, LazyVecFrom1};

use super::Vecs;

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexer: &Indexer) -> Result<Self> {
        Ok(Self {
            txindex_to_input_count: EagerVec::forced_import(db, "input_count", version)?,
            txindex_to_output_count: EagerVec::forced_import(db, "output_count", version)?,
            txindex_to_txindex: LazyVecFrom1::init(
                "txindex",
                version,
                indexer.vecs.tx.txindex_to_txid.boxed_clone(),
                |index, _| Some(index),
            ),
            txinindex_to_txinindex: LazyVecFrom1::init(
                "txinindex",
                version,
                indexer.vecs.txin.txinindex_to_outpoint.boxed_clone(),
                |index, _| Some(index),
            ),
            txoutindex_to_txoutindex: LazyVecFrom1::init(
                "txoutindex",
                version,
                indexer.vecs.txout.txoutindex_to_value.boxed_clone(),
                |index, _| Some(index),
            ),
        })
    }
}
