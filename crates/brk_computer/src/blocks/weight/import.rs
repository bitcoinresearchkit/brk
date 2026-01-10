use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Version;
use vecdb::{Database, IterableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedHeightDerivedFull, LazyFromHeightFull, WeightToFullness},
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let weight = ComputedHeightDerivedFull::forced_import(
            db,
            "block_weight",
            indexer.vecs.blocks.weight.boxed_clone(),
            version,
            indexes,
        )?;

        let fullness = LazyFromHeightFull::from_derived::<WeightToFullness>(
            "block_fullness",
            version,
            indexer.vecs.blocks.weight.boxed_clone(),
            &weight,
        );

        Ok(Self { weight, fullness })
    }
}
