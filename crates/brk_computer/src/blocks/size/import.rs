use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Version;
use vecdb::{Database, ReadableCloneableVec};

use super::Vecs;
use crate::{indexes, internal::{ComputedHeightDerivedFull, LazyComputedFromHeightFull, WeightToVbytes}};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            vbytes: LazyComputedFromHeightFull::forced_import::<WeightToVbytes>(
                db,
                "block_vbytes",
                version,
                &indexer.vecs.blocks.weight,
                indexes,
            )?,
            size: ComputedHeightDerivedFull::forced_import(
                db,
                "block_size",
                indexer.vecs.blocks.total_size.read_only_boxed_clone(),
                version,
                indexes,
            )?,
        })
    }
}
