use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, ImportableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{BlockCountTarget, ComputedFromHeightSumCum, ConstantVecs, RollingWindows},
};

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            block_count_target: ConstantVecs::new::<BlockCountTarget>(
                "block_count_target",
                version,
                indexes,
            ),
            block_count: ComputedFromHeightSumCum::forced_import(
                db,
                "block_count",
                version,
                indexes,
            )?,
            height_24h_ago: ImportableVec::forced_import(db, "height_24h_ago", version)?,
            height_3d_ago: ImportableVec::forced_import(db, "height_3d_ago", version)?,
            height_1w_ago: ImportableVec::forced_import(db, "height_1w_ago", version)?,
            height_8d_ago: ImportableVec::forced_import(db, "height_8d_ago", version)?,
            height_9d_ago: ImportableVec::forced_import(db, "height_9d_ago", version)?,
            height_12d_ago: ImportableVec::forced_import(db, "height_12d_ago", version)?,
            height_13d_ago: ImportableVec::forced_import(db, "height_13d_ago", version)?,
            height_2w_ago: ImportableVec::forced_import(db, "height_2w_ago", version)?,
            height_21d_ago: ImportableVec::forced_import(db, "height_21d_ago", version)?,
            height_26d_ago: ImportableVec::forced_import(db, "height_26d_ago", version)?,
            height_1m_ago: ImportableVec::forced_import(db, "height_1m_ago", version)?,
            height_34d_ago: ImportableVec::forced_import(db, "height_34d_ago", version)?,
            height_55d_ago: ImportableVec::forced_import(db, "height_55d_ago", version)?,
            height_2m_ago: ImportableVec::forced_import(db, "height_2m_ago", version)?,
            height_89d_ago: ImportableVec::forced_import(db, "height_89d_ago", version)?,
            height_111d_ago: ImportableVec::forced_import(db, "height_111d_ago", version)?,
            height_144d_ago: ImportableVec::forced_import(db, "height_144d_ago", version)?,
            height_3m_ago: ImportableVec::forced_import(db, "height_3m_ago", version)?,
            height_6m_ago: ImportableVec::forced_import(db, "height_6m_ago", version)?,
            height_200d_ago: ImportableVec::forced_import(db, "height_200d_ago", version)?,
            height_350d_ago: ImportableVec::forced_import(db, "height_350d_ago", version)?,
            height_1y_ago: ImportableVec::forced_import(db, "height_1y_ago", version)?,
            height_2y_ago: ImportableVec::forced_import(db, "height_2y_ago", version)?,
            height_200w_ago: ImportableVec::forced_import(db, "height_200w_ago", version)?,
            height_3y_ago: ImportableVec::forced_import(db, "height_3y_ago", version)?,
            height_4y_ago: ImportableVec::forced_import(db, "height_4y_ago", version)?,
            height_5y_ago: ImportableVec::forced_import(db, "height_5y_ago", version)?,
            height_6y_ago: ImportableVec::forced_import(db, "height_6y_ago", version)?,
            height_8y_ago: ImportableVec::forced_import(db, "height_8y_ago", version)?,
            height_10y_ago: ImportableVec::forced_import(db, "height_10y_ago", version)?,
            block_count_sum: RollingWindows::forced_import(
                db,
                "block_count_sum",
                version,
                indexes,
            )?,
        })
    }
}
