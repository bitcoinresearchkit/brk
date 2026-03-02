use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeight, DaysToYears, LazyHeightDerived, Price},
};

const VERSION: Version = Version::ONE;

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let price_ath = Price::forced_import(db, "price_ath", v, indexes)?;

        let max_days_between_price_aths =
            ComputedFromHeight::forced_import(db, "max_days_between_price_aths", v, indexes)?;

        let max_years_between_price_aths =
            LazyHeightDerived::from_computed::<DaysToYears>(
                "max_years_between_price_aths",
                v,
                &max_days_between_price_aths,
            );

        let days_since_price_ath =
            ComputedFromHeight::forced_import(db, "days_since_price_ath", v, indexes)?;

        let years_since_price_ath = LazyHeightDerived::from_computed::<DaysToYears>(
            "years_since_price_ath",
            v,
            &days_since_price_ath,
        );

        let price_drawdown =
            ComputedFromHeight::forced_import(db, "price_drawdown", v, indexes)?;

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
