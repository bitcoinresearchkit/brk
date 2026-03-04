use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::super::lookback::ByLookbackPeriod;
use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeightStdDev, PercentFromHeight},
    market::dca::ByDcaCagr,
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v1 = Version::ONE;

        let price_return = ByLookbackPeriod::try_new(|name, _days| {
            PercentFromHeight::forced_import_bps32(
                db,
                &format!("price_return_{name}"),
                version,
                indexes,
            )
        })?;

        // CAGR (computed, 2y+ only)
        let price_cagr = ByDcaCagr::try_new(|name, _days| {
            PercentFromHeight::forced_import_bps32(
                db,
                &format!("price_cagr_{name}"),
                version,
                indexes,
            )
        })?;

        let price_return_24h_sd_1w = ComputedFromHeightStdDev::forced_import(
            db,
            "price_return_24h",
            "1w",
            7,
            version + v1,
            indexes,
        )?;
        let price_return_24h_sd_1m = ComputedFromHeightStdDev::forced_import(
            db,
            "price_return_24h",
            "1m",
            30,
            version + v1,
            indexes,
        )?;
        let price_return_24h_sd_1y = ComputedFromHeightStdDev::forced_import(
            db,
            "price_return_24h",
            "1y",
            365,
            version + v1,
            indexes,
        )?;

        let price_downside_24h = EagerVec::forced_import(db, "price_downside_24h", version)?;
        let price_downside_24h_sd_1w = ComputedFromHeightStdDev::forced_import(
            db,
            "price_downside_24h",
            "1w",
            7,
            version + v1,
            indexes,
        )?;
        let price_downside_24h_sd_1m = ComputedFromHeightStdDev::forced_import(
            db,
            "price_downside_24h",
            "1m",
            30,
            version + v1,
            indexes,
        )?;
        let price_downside_24h_sd_1y = ComputedFromHeightStdDev::forced_import(
            db,
            "price_downside_24h",
            "1y",
            365,
            version + v1,
            indexes,
        )?;

        Ok(Self {
            price_return,
            price_cagr,
            price_return_24h_sd_1w,
            price_return_24h_sd_1m,
            price_return_24h_sd_1y,
            price_downside_24h,
            price_downside_24h_sd_1w,
            price_downside_24h_sd_1m,
            price_downside_24h_sd_1y,
        })
    }
}
