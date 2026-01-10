use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Height, StoredU64, Version};
use vecdb::{Database, IterableCloneableVec, VecIndex};

use super::Vecs;
use crate::{indexes, internal::{ComputedHeightDerivedFull, LazyComputedFromHeightFull}};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            vbytes: LazyComputedFromHeightFull::forced_import_with_init(
                db,
                "block_vbytes",
                version,
                indexer.vecs.blocks.weight.clone(),
                indexes,
                |height: Height, weight_iter| {
                    weight_iter
                        .get_at(height.to_usize())
                        .map(|w| StoredU64::from(w.to_vbytes_floor()))
                },
            )?,
            size: ComputedHeightDerivedFull::forced_import(
                db,
                "block_size",
                indexer.vecs.blocks.total_size.boxed_clone(),
                version,
                indexes,
            )?,
        })
    }
}
