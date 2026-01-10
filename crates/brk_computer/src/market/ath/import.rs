use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec, IterableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{
        ComputedFromDateLast, ComputedFromHeightAndDateLast, LazyBinaryFromHeightAndDateLast, LazyFromDateLast,
        PercentageDiffCloseDollars, StoredU16ToYears,
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
        let price_ath = ComputedFromHeightAndDateLast::forced_import(db, "price_ath", version, indexes)?;

        let max_days_between_price_aths =
            ComputedFromDateLast::forced_import(db, "max_days_between_price_aths", version, indexes)?;

        let max_years_between_price_aths = LazyFromDateLast::from_computed::<StoredU16ToYears>(
            "max_years_between_price_aths",
            version,
            max_days_between_price_aths.dateindex.boxed_clone(),
            &max_days_between_price_aths,
        );

        let days_since_price_ath =
            ComputedFromDateLast::forced_import(db, "days_since_price_ath", version, indexes)?;

        let years_since_price_ath = LazyFromDateLast::from_computed::<StoredU16ToYears>(
            "years_since_price_ath",
            version,
            days_since_price_ath.dateindex.boxed_clone(),
            &days_since_price_ath,
        );

        let price_drawdown =
            LazyBinaryFromHeightAndDateLast::from_computed_both_last::<PercentageDiffCloseDollars>(
                "price_drawdown",
                version,
                EagerVec::forced_import(db, "price_drawdown", version)?,
                &price.usd.split.close,
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
