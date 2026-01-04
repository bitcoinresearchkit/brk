use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Height, StoredU64, Version};
use vecdb::{Database, IterableCloneableVec, LazyVecFrom1, VecIndex};

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedVecsFromHeight, Source, VecBuilderOptions},
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let full_stats = || {
            VecBuilderOptions::default()
                .add_average()
                .add_minmax()
                .add_percentiles()
                .add_sum()
                .add_cumulative()
        };

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
            indexes_to_block_size: ComputedVecsFromHeight::forced_import(
                db,
                "block_size",
                Source::Vec(indexer.vecs.block.height_to_total_size.boxed_clone()),
                version,
                indexes,
                full_stats(),
            )?,
            indexes_to_block_vbytes: ComputedVecsFromHeight::forced_import(
                db,
                "block_vbytes",
                Source::Vec(height_to_vbytes.boxed_clone()),
                version,
                indexes,
                full_stats(),
            )?,
            height_to_vbytes,
        })
    }
}
