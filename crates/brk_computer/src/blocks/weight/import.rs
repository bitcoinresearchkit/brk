use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Version;
use vecdb::{Database, ReadableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedHeightDerivedFull, LazyFromHeightTransformDistribution, WeightToFullness},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let weight = ComputedHeightDerivedFull::forced_import(
            db,
            "block_weight",
            indexer.vecs.blocks.weight.read_only_boxed_clone(),
            version,
            indexes,
        )?;

        let fullness = LazyFromHeightTransformDistribution::from_derived::<WeightToFullness>(
            "block_fullness",
            version,
            indexer.vecs.blocks.weight.read_only_boxed_clone(),
            &weight,
        );

        Ok(Self { weight, fullness })
    }
}
