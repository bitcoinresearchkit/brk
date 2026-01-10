use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::super::lookback::{self, LOOKBACK_PERIOD_NAMES};
use super::Vecs;
use crate::{
    indexes,
    internal::{
        ComputedFromDateLast, ComputedFromDateStdDev, LazyBinaryFromDateLast,
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

        let price_returns =
            LOOKBACK_PERIOD_NAMES
                .zip_ref(&lookback.price_ago)
                .map(|(name, price_ago)| {
                    LazyBinaryFromDateLast::from_computed_both_last::<PercentageDiffCloseDollars>(
                        &format!("{name}_price_returns"),
                        version,
                        &price.usd.split.close,
                        price_ago,
                    )
                });

        // CAGR (computed, 2y+ only)
        let cagr = ByDcaCagr::try_new(|name, _days| {
            ComputedFromDateLast::forced_import(db, &format!("{name}_cagr"), version, indexes)
        })?;

        let _1d_returns_1w_sd = ComputedFromDateStdDev::forced_import(
            db,
            "1d_returns_1w_sd",
            7,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;
        let _1d_returns_1m_sd = ComputedFromDateStdDev::forced_import(
            db,
            "1d_returns_1m_sd",
            30,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;
        let _1d_returns_1y_sd = ComputedFromDateStdDev::forced_import(
            db,
            "1d_returns_1y_sd",
            365,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;

        let downside_returns = EagerVec::forced_import(db, "downside_returns", version)?;
        let downside_1w_sd = ComputedFromDateStdDev::forced_import(
            db,
            "downside_1w_sd",
            7,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;
        let downside_1m_sd = ComputedFromDateStdDev::forced_import(
            db,
            "downside_1m_sd",
            30,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;
        let downside_1y_sd = ComputedFromDateStdDev::forced_import(
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
            _1d_returns_1w_sd,
            _1d_returns_1m_sd,
            _1d_returns_1y_sd,
            downside_returns,
            downside_1w_sd,
            downside_1m_sd,
            downside_1y_sd,
        })
    }
}
