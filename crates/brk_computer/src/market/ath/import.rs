use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec, IterableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{
        ComputedVecsFromDateIndex, LazyVecsFrom2FromDateIndex, LazyVecsFromDateIndex,
        PercentageDiffCloseDollars, Source, StoredU16ToYears, VecBuilderOptions,
    },
    price,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: &price::Vecs,
    ) -> Result<Self> {
        let last = VecBuilderOptions::default().add_last();

        let indexes_to_price_ath = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_ath",
            Source::Compute,
            version,
            indexes,
            last,
        )?;

        let indexes_to_max_days_between_price_aths = ComputedVecsFromDateIndex::forced_import(
            db,
            "max_days_between_price_aths",
            Source::Compute,
            version,
            indexes,
            last,
        )?;

        let indexes_to_max_years_between_price_aths =
            LazyVecsFromDateIndex::from_computed::<StoredU16ToYears>(
                "max_years_between_price_aths",
                version,
                indexes_to_max_days_between_price_aths
                    .dateindex
                    .as_ref()
                    .map(|v| v.boxed_clone()),
                &indexes_to_max_days_between_price_aths,
            );

        let indexes_to_days_since_price_ath = ComputedVecsFromDateIndex::forced_import(
            db,
            "days_since_price_ath",
            Source::Compute,
            version,
            indexes,
            last,
        )?;

        let indexes_to_years_since_price_ath =
            LazyVecsFromDateIndex::from_computed::<StoredU16ToYears>(
                "years_since_price_ath",
                version,
                indexes_to_days_since_price_ath
                    .dateindex
                    .as_ref()
                    .map(|v| v.boxed_clone()),
                &indexes_to_days_since_price_ath,
            );

        let indexes_to_price_drawdown =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "price_drawdown",
                version,
                &price.usd.timeindexes_to_price_close,
                &indexes_to_price_ath,
            );

        Ok(Self {
            height_to_price_ath: EagerVec::forced_import(db, "price_ath", version)?,
            height_to_price_drawdown: EagerVec::forced_import(db, "price_drawdown", version)?,
            indexes_to_price_ath,
            indexes_to_price_drawdown,
            indexes_to_days_since_price_ath,
            indexes_to_years_since_price_ath,
            indexes_to_max_days_between_price_aths,
            indexes_to_max_years_between_price_aths,
        })
    }
}
