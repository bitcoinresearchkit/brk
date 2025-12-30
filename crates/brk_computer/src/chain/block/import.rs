use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{StoredU64, Version};
use vecdb::{Database, EagerVec, ImportableVec, IterableCloneableVec, LazyVecFrom1};

use super::Vecs;
use crate::{
    chain::{
        TARGET_BLOCKS_PER_DAY, TARGET_BLOCKS_PER_DECADE, TARGET_BLOCKS_PER_MONTH,
        TARGET_BLOCKS_PER_QUARTER, TARGET_BLOCKS_PER_SEMESTER, TARGET_BLOCKS_PER_WEEK,
        TARGET_BLOCKS_PER_YEAR,
    },
    grouped::{ComputedVecsFromDateIndex, ComputedVecsFromHeight, Source, VecBuilderOptions},
    indexes,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v0 = Version::ZERO;

        let last = || VecBuilderOptions::default().add_last();
        let sum_cum = || VecBuilderOptions::default().add_sum().add_cumulative();
        let stats = || {
            VecBuilderOptions::default()
                .add_average()
                .add_minmax()
                .add_percentiles()
        };
        let full_stats = || {
            VecBuilderOptions::default()
                .add_average()
                .add_minmax()
                .add_percentiles()
                .add_sum()
                .add_cumulative()
        };

        let dateindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + v0,
            indexes.time.dateindex_to_dateindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_DAY)),
        );
        let weekindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + v0,
            indexes.time.weekindex_to_weekindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_WEEK)),
        );
        let monthindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + v0,
            indexes.time.monthindex_to_monthindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_MONTH)),
        );
        let quarterindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + v0,
            indexes.time.quarterindex_to_quarterindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_QUARTER)),
        );
        let semesterindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + v0,
            indexes.time.semesterindex_to_semesterindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_SEMESTER)),
        );
        let yearindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + v0,
            indexes.time.yearindex_to_yearindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_YEAR)),
        );
        let decadeindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + v0,
            indexes.time.decadeindex_to_decadeindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_DECADE)),
        );

        let height_to_interval = EagerVec::forced_import(db, "interval", version + v0)?;
        let height_to_vbytes = EagerVec::forced_import(db, "vbytes", version + v0)?;

        Ok(Self {
            dateindex_to_block_count_target,
            weekindex_to_block_count_target,
            monthindex_to_block_count_target,
            quarterindex_to_block_count_target,
            semesterindex_to_block_count_target,
            yearindex_to_block_count_target,
            decadeindex_to_block_count_target,
            height_to_interval: height_to_interval.clone(),
            height_to_24h_block_count: EagerVec::forced_import(
                db,
                "24h_block_count",
                version + v0,
            )?,
            height_to_vbytes: height_to_vbytes.clone(),
            indexes_to_block_count: ComputedVecsFromHeight::forced_import(
                db,
                "block_count",
                Source::Compute,
                version + v0,
                indexes,
                sum_cum(),
            )?,
            indexes_to_1w_block_count: ComputedVecsFromDateIndex::forced_import(
                db,
                "1w_block_count",
                Source::Compute,
                version + v0,
                indexes,
                last(),
            )?,
            indexes_to_1m_block_count: ComputedVecsFromDateIndex::forced_import(
                db,
                "1m_block_count",
                Source::Compute,
                version + v0,
                indexes,
                last(),
            )?,
            indexes_to_1y_block_count: ComputedVecsFromDateIndex::forced_import(
                db,
                "1y_block_count",
                Source::Compute,
                version + v0,
                indexes,
                last(),
            )?,
            indexes_to_block_interval: ComputedVecsFromHeight::forced_import(
                db,
                "block_interval",
                Source::Vec(height_to_interval.boxed_clone()),
                version + v0,
                indexes,
                stats(),
            )?,
            indexes_to_block_size: ComputedVecsFromHeight::forced_import(
                db,
                "block_size",
                Source::Vec(indexer.vecs.block.height_to_total_size.boxed_clone()),
                version + v0,
                indexes,
                full_stats(),
            )?,
            indexes_to_block_vbytes: ComputedVecsFromHeight::forced_import(
                db,
                "block_vbytes",
                Source::Vec(height_to_vbytes.boxed_clone()),
                version + v0,
                indexes,
                full_stats(),
            )?,
            indexes_to_block_weight: ComputedVecsFromHeight::forced_import(
                db,
                "block_weight",
                Source::Vec(indexer.vecs.block.height_to_weight.boxed_clone()),
                version + v0,
                indexes,
                full_stats(),
            )?,
            // Timestamp metrics (moved from epoch)
            difficultyepoch_to_timestamp: EagerVec::forced_import(db, "timestamp", version + v0)?,
            timeindexes_to_timestamp: ComputedVecsFromDateIndex::forced_import(
                db,
                "timestamp",
                Source::Compute,
                version + v0,
                indexes,
                VecBuilderOptions::default().add_first(),
            )?,
        })
    }
}
