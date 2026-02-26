use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::{ComputedFromHeightLast, Price}};

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let v1 = Version::ONE;

        Ok(Self {
            price_1w_min: Price::forced_import(db, "price_1w_min", version + v1, indexes)?,
            price_1w_max: Price::forced_import(db, "price_1w_max", version + v1, indexes)?,
            price_2w_min: Price::forced_import(db, "price_2w_min", version + v1, indexes)?,
            price_2w_max: Price::forced_import(db, "price_2w_max", version + v1, indexes)?,
            price_1m_min: Price::forced_import(db, "price_1m_min", version + v1, indexes)?,
            price_1m_max: Price::forced_import(db, "price_1m_max", version + v1, indexes)?,
            price_1y_min: Price::forced_import(db, "price_1y_min", version + v1, indexes)?,
            price_1y_max: Price::forced_import(db, "price_1y_max", version + v1, indexes)?,
            price_true_range: ComputedFromHeightLast::forced_import(
                db, "price_true_range", version + v1, indexes,
            )?,
            price_true_range_2w_sum: ComputedFromHeightLast::forced_import(
                db, "price_true_range_2w_sum", version + v1, indexes,
            )?,
            price_2w_choppiness_index: ComputedFromHeightLast::forced_import(
                db, "price_2w_choppiness_index", version + v1, indexes,
            )?,
        })
    }
}
