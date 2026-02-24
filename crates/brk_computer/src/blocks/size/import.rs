use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Version;
use vecdb::{Database, ReadableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeightCumFull, ComputedHeightDerivedCumFull},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            vbytes: ComputedFromHeightCumFull::forced_import(
                db,
                "block_vbytes",
                version,
                indexes,
            )?,
            size: ComputedHeightDerivedCumFull::forced_import(
                db,
                "block_size",
                indexer.vecs.blocks.total_size.read_only_boxed_clone(),
                version,
                indexes,
            )?,
        })
    }
}
