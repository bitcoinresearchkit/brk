use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::{vecs::PriceMinMaxVecs, Vecs};
use crate::{
    indexes,
    internal::{ComputedPerBlock, PercentPerBlock, Price},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v1 = Version::ONE;

        Ok(Self {
            min: PriceMinMaxVecs {
                _1w: Price::forced_import(db, "price_min_1w", version + v1, indexes)?,
                _2w: Price::forced_import(db, "price_min_2w", version + v1, indexes)?,
                _1m: Price::forced_import(db, "price_min_1m", version + v1, indexes)?,
                _1y: Price::forced_import(db, "price_min_1y", version + v1, indexes)?,
            },
            max: PriceMinMaxVecs {
                _1w: Price::forced_import(db, "price_max_1w", version + v1, indexes)?,
                _2w: Price::forced_import(db, "price_max_2w", version + v1, indexes)?,
                _1m: Price::forced_import(db, "price_max_1m", version + v1, indexes)?,
                _1y: Price::forced_import(db, "price_max_1y", version + v1, indexes)?,
            },
            true_range: ComputedPerBlock::forced_import(
                db,
                "price_true_range",
                version + v1,
                indexes,
            )?,
            true_range_sum_2w: ComputedPerBlock::forced_import(
                db,
                "price_true_range_sum_2w",
                version + v1,
                indexes,
            )?,
            choppiness_index_2w: PercentPerBlock::forced_import(
                db,
                "price_choppiness_index_2w",
                version + v1,
                indexes,
            )?,
        })
    }
}
