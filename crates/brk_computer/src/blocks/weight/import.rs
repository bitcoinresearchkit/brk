use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Version;
use vecdb::{Database, IterableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{
        DerivedComputedBlockFull, LazyBlockFull, WeightToFullness,
    },
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let indexes_to_block_weight = DerivedComputedBlockFull::forced_import(
            db,
            "block_weight",
            indexer.vecs.block.height_to_weight.boxed_clone(),
            version,
            indexes,
        )?;

        let indexes_to_block_fullness =
            LazyBlockFull::from_derived::<WeightToFullness>(
                "block_fullness",
                version,
                indexer.vecs.block.height_to_weight.boxed_clone(),
                &indexes_to_block_weight,
            );

        Ok(Self {
            indexes_to_block_weight,
            indexes_to_block_fullness,
        })
    }
}
