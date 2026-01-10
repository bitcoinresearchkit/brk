use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::{indexes, internal::ComputedFromDateLast};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let v1 = Version::ONE;

        Ok(Self {
            price_1w_min: ComputedFromDateLast::forced_import(
                db,
                "price_1w_min",
                version + v1,
                indexes,
            )?,
            price_1w_max: ComputedFromDateLast::forced_import(
                db,
                "price_1w_max",
                version + v1,
                indexes,
            )?,
            price_2w_min: ComputedFromDateLast::forced_import(
                db,
                "price_2w_min",
                version + v1,
                indexes,
            )?,
            price_2w_max: ComputedFromDateLast::forced_import(
                db,
                "price_2w_max",
                version + v1,
                indexes,
            )?,
            price_1m_min: ComputedFromDateLast::forced_import(
                db,
                "price_1m_min",
                version + v1,
                indexes,
            )?,
            price_1m_max: ComputedFromDateLast::forced_import(
                db,
                "price_1m_max",
                version + v1,
                indexes,
            )?,
            price_1y_min: ComputedFromDateLast::forced_import(
                db,
                "price_1y_min",
                version + v1,
                indexes,
            )?,
            price_1y_max: ComputedFromDateLast::forced_import(
                db,
                "price_1y_max",
                version + v1,
                indexes,
            )?,
            price_true_range: EagerVec::forced_import(db, "price_true_range", version)?,
            price_true_range_2w_sum: EagerVec::forced_import(
                db,
                "price_true_range_2w_sum",
                version,
            )?,
            price_2w_choppiness_index: ComputedFromDateLast::forced_import(
                db,
                "price_2w_choppiness_index",
                version + v1,
                indexes,
            )?,
        })
    }
}
