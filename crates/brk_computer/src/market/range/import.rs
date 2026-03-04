use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::{ComputedFromHeight, PercentFromHeight, Price}};

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let v1 = Version::ONE;

        Ok(Self {
            price_min_1w: Price::forced_import(db, "price_min_1w", version + v1, indexes)?,
            price_max_1w: Price::forced_import(db, "price_max_1w", version + v1, indexes)?,
            price_min_2w: Price::forced_import(db, "price_min_2w", version + v1, indexes)?,
            price_max_2w: Price::forced_import(db, "price_max_2w", version + v1, indexes)?,
            price_min_1m: Price::forced_import(db, "price_min_1m", version + v1, indexes)?,
            price_max_1m: Price::forced_import(db, "price_max_1m", version + v1, indexes)?,
            price_min_1y: Price::forced_import(db, "price_min_1y", version + v1, indexes)?,
            price_max_1y: Price::forced_import(db, "price_max_1y", version + v1, indexes)?,
            price_true_range: ComputedFromHeight::forced_import(
                db, "price_true_range", version + v1, indexes,
            )?,
            price_true_range_sum_2w: ComputedFromHeight::forced_import(
                db, "price_true_range_sum_2w", version + v1, indexes,
            )?,
            price_choppiness_index_2w: PercentFromHeight::forced_import_bp16(
                db, "price_choppiness_index_2w", version + v1, indexes,
            )?,
        })
    }
}
