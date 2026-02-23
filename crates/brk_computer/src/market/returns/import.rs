use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec, ReadableCloneableVec};

use super::super::lookback::{self, LOOKBACK_PERIOD_NAMES};
use super::Vecs;
use crate::{
    indexes,
    internal::{
        ComputedFromHeightLast, ComputedFromHeightStdDev, LazyBinaryFromHeightLast,
        PercentageDiffDollars, StandardDeviationVecsOptions,
    },
    market::dca::ByDcaCagr,
    prices,
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
        lookback: &lookback::Vecs,
    ) -> Result<Self> {
        let v1 = Version::ONE;

        let price_returns =
            LOOKBACK_PERIOD_NAMES
                .zip_ref(&lookback.price_ago)
                .map(|(name, price_ago)| {
                    LazyBinaryFromHeightLast::from_height_and_derived_last::<
                        PercentageDiffDollars,
                    >(
                        &format!("{name}_price_returns"),
                        version,
                        prices.usd.price.read_only_boxed_clone(),
                        price_ago.height.read_only_boxed_clone(),
                        &prices.usd.split.close,
                        &price_ago.rest,
                    )
                });

        // CAGR (computed, 2y+ only)
        let cagr = ByDcaCagr::try_new(|name, _days| {
            ComputedFromHeightLast::forced_import(db, &format!("{name}_cagr"), version, indexes)
        })?;

        let _1d_returns_1w_sd = ComputedFromHeightStdDev::forced_import(
            db,
            "1d_returns_1w_sd",
            7,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;
        let _1d_returns_1m_sd = ComputedFromHeightStdDev::forced_import(
            db,
            "1d_returns_1m_sd",
            30,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;
        let _1d_returns_1y_sd = ComputedFromHeightStdDev::forced_import(
            db,
            "1d_returns_1y_sd",
            365,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;

        let downside_returns = EagerVec::forced_import(db, "downside_returns", version)?;
        let downside_1w_sd = ComputedFromHeightStdDev::forced_import(
            db,
            "downside_1w_sd",
            7,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;
        let downside_1m_sd = ComputedFromHeightStdDev::forced_import(
            db,
            "downside_1m_sd",
            30,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;
        let downside_1y_sd = ComputedFromHeightStdDev::forced_import(
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
