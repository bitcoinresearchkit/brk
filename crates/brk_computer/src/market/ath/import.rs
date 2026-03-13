use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{PerBlock, DaysToYears, DerivedResolutions, PercentPerBlock, Price},
};

const VERSION: Version = Version::ONE;

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let high = Price::forced_import(db, "price_ath", v, indexes)?;

        let max_days_between =
            PerBlock::forced_import(db, "max_days_between_price_ath", v, indexes)?;

        let max_years_between = DerivedResolutions::from_computed::<DaysToYears>(
            "max_years_between_price_ath",
            v,
            &max_days_between,
        );

        let days_since =
            PerBlock::forced_import(db, "days_since_price_ath", v, indexes)?;

        let years_since = DerivedResolutions::from_computed::<DaysToYears>(
            "years_since_price_ath",
            v,
            &days_since,
        );

        let drawdown = PercentPerBlock::forced_import(db, "price_drawdown", v, indexes)?;

        Ok(Self {
            high,
            drawdown,
            days_since,
            years_since,
            max_days_between,
            max_years_between,
        })
    }
}
