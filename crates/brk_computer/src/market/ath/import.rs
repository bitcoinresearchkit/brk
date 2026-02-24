use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, ReadableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{
        ComputedFromHeightLast, LazyBinaryFromHeightLast, LazyHeightDerivedLast,
        PercentageDiffDollars, PriceFromHeight, StoredU16ToYears,
    },
    prices,
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
    ) -> Result<Self> {
        let price_ath = PriceFromHeight::forced_import(db, "price_ath", version, indexes)?;

        let max_days_between_price_aths = ComputedFromHeightLast::forced_import(
            db,
            "max_days_between_price_aths",
            version,
            indexes,
        )?;

        let max_years_between_price_aths =
            LazyHeightDerivedLast::from_computed::<StoredU16ToYears>(
                "max_years_between_price_aths",
                version,
                &max_days_between_price_aths,
            );

        let days_since_price_ath =
            ComputedFromHeightLast::forced_import(db, "days_since_price_ath", version, indexes)?;

        let years_since_price_ath = LazyHeightDerivedLast::from_computed::<StoredU16ToYears>(
            "years_since_price_ath",
            version,
            &days_since_price_ath,
        );

        let price_drawdown =
            LazyBinaryFromHeightLast::from_height_and_derived_last::<PercentageDiffDollars>(
                "price_drawdown",
                version,
                prices.usd.price.read_only_boxed_clone(),
                price_ath.height.read_only_boxed_clone(),
                &prices.usd.close,
                &price_ath.rest,
            );

        Ok(Self {
            price_ath,
            price_drawdown,
            days_since_price_ath,
            years_since_price_ath,
            max_days_between_price_aths,
            max_years_between_price_aths,
        })
    }
}
