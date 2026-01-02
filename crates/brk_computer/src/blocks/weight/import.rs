use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Version;
use vecdb::{Database, IterableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedVecsFromHeight, LazyVecsFromHeight, Source, VecBuilderOptions, WeightToFullness},
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v0 = Version::ZERO;
        let full_stats = || {
            VecBuilderOptions::default()
                .add_average()
                .add_minmax()
                .add_percentiles()
                .add_sum()
                .add_cumulative()
        };

        let indexes_to_block_weight = ComputedVecsFromHeight::forced_import(
            db,
            "block_weight",
            Source::Vec(indexer.vecs.block.height_to_weight.boxed_clone()),
            version + v0,
            indexes,
            full_stats(),
        )?;

        let indexes_to_block_fullness = LazyVecsFromHeight::from_computed::<WeightToFullness>(
            "block_fullness",
            version + v0,
            indexer.vecs.block.height_to_weight.boxed_clone(),
            &indexes_to_block_weight,
        );

        Ok(Self {
            indexes_to_block_weight,
            indexes_to_block_fullness,
        })
    }
}
