use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedPerBlock, DaysToYears, DerivedResolutions, PercentPerBlock, Price},
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

        let max_days_between_price_ath =
            ComputedPerBlock::forced_import(db, "max_days_between_price_ath", v, indexes)?;

        let max_years_between_price_ath = DerivedResolutions::from_computed::<DaysToYears>(
            "max_years_between_price_ath",
            v,
            &max_days_between_price_ath,
        );

        let days_since_price_ath =
            ComputedPerBlock::forced_import(db, "days_since_price_ath", v, indexes)?;

        let years_since_price_ath = DerivedResolutions::from_computed::<DaysToYears>(
            "years_since_price_ath",
            v,
            &days_since_price_ath,
        );

        let price_drawdown = PercentPerBlock::forced_import(db, "price_drawdown", v, indexes)?;

        Ok(Self {
            price_ath,
            price_drawdown,
            days_since_price_ath,
            years_since_price_ath,
            max_days_between_price_ath,
            max_years_between_price_ath,
        })
    }
}
