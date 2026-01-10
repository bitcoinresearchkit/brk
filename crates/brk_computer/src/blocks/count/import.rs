use brk_error::Result;
use brk_types::{StoredU64, Version};
use vecdb::{Database, ImportableVec};

use super::Vecs;
use crate::{
    blocks::{
        TARGET_BLOCKS_PER_DAY, TARGET_BLOCKS_PER_DECADE, TARGET_BLOCKS_PER_MONTH,
        TARGET_BLOCKS_PER_QUARTER, TARGET_BLOCKS_PER_SEMESTER, TARGET_BLOCKS_PER_WEEK,
        TARGET_BLOCKS_PER_YEAR,
    },
    indexes,
    internal::{ComputedFromHeightLast, ComputedFromHeightSumCum, LazyFromDate},
};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            block_count_target: LazyFromDate::new(
                "block_count_target",
                version,
                indexes,
                |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_DAY)),
                |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_WEEK)),
                |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_MONTH)),
                |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_QUARTER)),
                |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_SEMESTER)),
                |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_YEAR)),
                |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_DECADE)),
            ),
            block_count: ComputedFromHeightSumCum::forced_import(db, "block_count", version, indexes)?,
            _24h_start: ImportableVec::forced_import(db, "24h_start", version)?,
            _1w_start: ImportableVec::forced_import(db, "1w_start", version)?,
            _1m_start: ImportableVec::forced_import(db, "1m_start", version)?,
            _1y_start: ImportableVec::forced_import(db, "1y_start", version)?,
            _24h_block_count: ComputedFromHeightLast::forced_import(
                db,
                "24h_block_count",
                version,
                indexes,
            )?,
            _1w_block_count: ComputedFromHeightLast::forced_import(
                db,
                "1w_block_count",
                version,
                indexes,
            )?,
            _1m_block_count: ComputedFromHeightLast::forced_import(
                db,
                "1m_block_count",
                version,
                indexes,
            )?,
            _1y_block_count: ComputedFromHeightLast::forced_import(
                db,
                "1y_block_count",
                version,
                indexes,
            )?,
        })
    }
}
