use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::super::lookback::{self, LOOKBACK_PERIOD_NAMES};
use super::Vecs;
use crate::{
    indexes,
    internal::{
        BinaryDateLast, ComputedDateLast, ComputedStandardDeviationVecsDate,
        PercentageDiffCloseDollars, StandardDeviationVecsOptions,
    },
    market::dca::ByDcaCagr,
    price,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: &price::Vecs,
        lookback: &lookback::Vecs,
    ) -> Result<Self> {
        let v1 = Version::ONE;

        // KISS: Price returns (lazy, from price.close and lookback.price_ago)
        let price_returns =
            LOOKBACK_PERIOD_NAMES
                .zip_ref(&lookback.price_ago)
                .map(|(name, price_ago)| {
                    BinaryDateLast::from_computed_both_last::<PercentageDiffCloseDollars>(
                        &format!("{name}_price_returns"),
                        version,
                        &price.usd.timeindexes_to_price_close,
                        price_ago,
                    )
                });

        // CAGR (computed, 2y+ only)
        let cagr = ByDcaCagr::try_new(|name, _days| {
            ComputedDateLast::forced_import(db, &format!("{name}_cagr"), version, indexes)
        })?;

        let indexes_to_1d_returns_1w_sd = ComputedStandardDeviationVecsDate::forced_import(
            db,
            "1d_returns_1w_sd",
            7,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;
        let indexes_to_1d_returns_1m_sd = ComputedStandardDeviationVecsDate::forced_import(
            db,
            "1d_returns_1m_sd",
            30,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;
        let indexes_to_1d_returns_1y_sd = ComputedStandardDeviationVecsDate::forced_import(
            db,
            "1d_returns_1y_sd",
            365,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;

        let dateindex_to_downside_returns =
            EagerVec::forced_import(db, "downside_returns", version)?;
        let indexes_to_downside_1w_sd = ComputedStandardDeviationVecsDate::forced_import(
            db,
            "downside_1w_sd",
            7,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;
        let indexes_to_downside_1m_sd = ComputedStandardDeviationVecsDate::forced_import(
            db,
            "downside_1m_sd",
            30,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;
        let indexes_to_downside_1y_sd = ComputedStandardDeviationVecsDate::forced_import(
            db,
            "downside_1y_sd",
            365,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;

        Ok(Self {
            price_returns,
            cagr,

            indexes_to_1d_returns_1w_sd,
            indexes_to_1d_returns_1m_sd,
            indexes_to_1d_returns_1y_sd,

            dateindex_to_downside_returns,
            indexes_to_downside_1w_sd,
            indexes_to_downside_1m_sd,
            indexes_to_downside_1y_sd,
        })
    }
}
