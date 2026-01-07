use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Height, StoredU64, Version};
use vecdb::{Database, IterableCloneableVec, LazyVecFrom1, VecIndex};

use super::Vecs;
use crate::{
    indexes,
    internal::DerivedComputedBlockFull,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let height_to_vbytes = LazyVecFrom1::init(
            "vbytes",
            version,
            indexer.vecs.block.height_to_weight.boxed_clone(),
            |height: Height, weight_iter| {
                weight_iter
                    .get_at(height.to_usize())
                    .map(|w| StoredU64::from(w.to_vbytes_floor()))
            },
        );

        Ok(Self {
            indexes_to_block_size: DerivedComputedBlockFull::forced_import(
                db,
                "block_size",
                indexer.vecs.block.height_to_total_size.boxed_clone(),
                version,
                indexes,
            )?,
            indexes_to_block_vbytes: DerivedComputedBlockFull::forced_import(
                db,
                "block_vbytes",
                height_to_vbytes.boxed_clone(),
                version,
                indexes,
            )?,
            height_to_vbytes,
        })
    }
}
