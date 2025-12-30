use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::{
    grouped::{ComputedVecsFromDateIndex, Source, VecBuilderOptions},
    indexes,
};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let v0 = Version::ZERO;
        let v1 = Version::ONE;
        let last = VecBuilderOptions::default().add_last();

        Ok(Self {
            indexes_to_price_1w_min: ComputedVecsFromDateIndex::forced_import(
                db,
                "price_1w_min",
                Source::Compute,
                version + v1,
                indexes,
                last,
            )?,
            indexes_to_price_1w_max: ComputedVecsFromDateIndex::forced_import(
                db,
                "price_1w_max",
                Source::Compute,
                version + v1,
                indexes,
                last,
            )?,
            indexes_to_price_2w_min: ComputedVecsFromDateIndex::forced_import(
                db,
                "price_2w_min",
                Source::Compute,
                version + v1,
                indexes,
                last,
            )?,
            indexes_to_price_2w_max: ComputedVecsFromDateIndex::forced_import(
                db,
                "price_2w_max",
                Source::Compute,
                version + v1,
                indexes,
                last,
            )?,
            indexes_to_price_1m_min: ComputedVecsFromDateIndex::forced_import(
                db,
                "price_1m_min",
                Source::Compute,
                version + v1,
                indexes,
                last,
            )?,
            indexes_to_price_1m_max: ComputedVecsFromDateIndex::forced_import(
                db,
                "price_1m_max",
                Source::Compute,
                version + v1,
                indexes,
                last,
            )?,
            indexes_to_price_1y_min: ComputedVecsFromDateIndex::forced_import(
                db,
                "price_1y_min",
                Source::Compute,
                version + v1,
                indexes,
                last,
            )?,
            indexes_to_price_1y_max: ComputedVecsFromDateIndex::forced_import(
                db,
                "price_1y_max",
                Source::Compute,
                version + v1,
                indexes,
                last,
            )?,
            dateindex_to_price_true_range: EagerVec::forced_import(
                db,
                "price_true_range",
                version + v0,
            )?,
            dateindex_to_price_true_range_2w_sum: EagerVec::forced_import(
                db,
                "price_true_range_2w_sum",
                version + v0,
            )?,
            indexes_to_price_2w_choppiness_index: ComputedVecsFromDateIndex::forced_import(
                db,
                "price_2w_choppiness_index",
                Source::Compute,
                version + v1,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
        })
    }
}
