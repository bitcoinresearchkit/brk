use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::super::lookback::ByLookbackPeriod;
use super::Vecs;
use crate::{
    indexes,
    internal::{
        ComputedFromHeightLast, ComputedFromHeightStdDev,
        StandardDeviationVecsOptions,
    },
    market::dca::ByDcaCagr,
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v1 = Version::ONE;

        let price_returns = ByLookbackPeriod::try_new(|name, _days| {
            ComputedFromHeightLast::forced_import(
                db,
                &format!("{name}_price_returns"),
                version,
                indexes,
            )
        })?;

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
        )?;
        let _1d_returns_1m_sd = ComputedFromHeightStdDev::forced_import(
            db,
            "1d_returns_1m_sd",
            30,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
        )?;
        let _1d_returns_1y_sd = ComputedFromHeightStdDev::forced_import(
            db,
            "1d_returns_1y_sd",
            365,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
        )?;

        let downside_returns = EagerVec::forced_import(db, "downside_returns", version)?;
        let downside_1w_sd = ComputedFromHeightStdDev::forced_import(
            db,
            "downside_1w_sd",
            7,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
        )?;
        let downside_1m_sd = ComputedFromHeightStdDev::forced_import(
            db,
            "downside_1m_sd",
            30,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
        )?;
        let downside_1y_sd = ComputedFromHeightStdDev::forced_import(
            db,
            "downside_1y_sd",
            365,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
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
