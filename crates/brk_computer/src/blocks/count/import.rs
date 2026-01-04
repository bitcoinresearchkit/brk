use brk_error::Result;
use brk_types::{StoredU64, Version};
use vecdb::{Database, EagerVec, ImportableVec, IterableCloneableVec, LazyVecFrom1};

use super::Vecs;
use crate::{
    blocks::{
        TARGET_BLOCKS_PER_DAY, TARGET_BLOCKS_PER_DECADE, TARGET_BLOCKS_PER_MONTH,
        TARGET_BLOCKS_PER_QUARTER, TARGET_BLOCKS_PER_SEMESTER, TARGET_BLOCKS_PER_WEEK,
        TARGET_BLOCKS_PER_YEAR,
    },
    indexes,
    internal::{ComputedVecsFromDateIndex, ComputedVecsFromHeight, Source, VecBuilderOptions},
};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let last = || VecBuilderOptions::default().add_last();
        let sum_cum = || VecBuilderOptions::default().add_sum().add_cumulative();

        Ok(Self {
            dateindex_to_block_count_target: LazyVecFrom1::init(
                "block_count_target",
                version,
                indexes.time.dateindex_to_dateindex.boxed_clone(),
                |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_DAY)),
            ),
            weekindex_to_block_count_target: LazyVecFrom1::init(
                "block_count_target",
                version,
                indexes.time.weekindex_to_weekindex.boxed_clone(),
                |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_WEEK)),
            ),
            monthindex_to_block_count_target: LazyVecFrom1::init(
                "block_count_target",
                version,
                indexes.time.monthindex_to_monthindex.boxed_clone(),
                |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_MONTH)),
            ),
            quarterindex_to_block_count_target: LazyVecFrom1::init(
                "block_count_target",
                version,
                indexes.time.quarterindex_to_quarterindex.boxed_clone(),
                |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_QUARTER)),
            ),
            semesterindex_to_block_count_target: LazyVecFrom1::init(
                "block_count_target",
                version,
                indexes.time.semesterindex_to_semesterindex.boxed_clone(),
                |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_SEMESTER)),
            ),
            yearindex_to_block_count_target: LazyVecFrom1::init(
                "block_count_target",
                version,
                indexes.time.yearindex_to_yearindex.boxed_clone(),
                |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_YEAR)),
            ),
            decadeindex_to_block_count_target: LazyVecFrom1::init(
                "block_count_target",
                version,
                indexes.time.decadeindex_to_decadeindex.boxed_clone(),
                |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_DECADE)),
            ),
            height_to_24h_block_count: EagerVec::forced_import(db, "24h_block_count", version)?,
            indexes_to_block_count: ComputedVecsFromHeight::forced_import(
                db,
                "block_count",
                Source::Compute,
                version,
                indexes,
                sum_cum(),
            )?,
            indexes_to_1w_block_count: ComputedVecsFromDateIndex::forced_import(
                db,
                "1w_block_count",
                Source::Compute,
                version,
                indexes,
                last(),
            )?,
            indexes_to_1m_block_count: ComputedVecsFromDateIndex::forced_import(
                db,
                "1m_block_count",
                Source::Compute,
                version,
                indexes,
                last(),
            )?,
            indexes_to_1y_block_count: ComputedVecsFromDateIndex::forced_import(
                db,
                "1y_block_count",
                Source::Compute,
                version,
                indexes,
                last(),
            )?,
        })
    }
}
