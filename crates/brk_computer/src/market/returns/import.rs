use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::super::lookback::ByLookbackPeriod;
use super::{vecs::PriceReturn24hSdVecs, Vecs};
use crate::{
    indexes,
    internal::{StdDevPerBlock, PercentPerBlock},
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
            PercentPerBlock::forced_import(db, &format!("price_return_{name}"), version, indexes)
        })?;

        // CAGR (computed, 2y+ only)
        let price_cagr = ByDcaCagr::try_new(|name, _days| {
            PercentPerBlock::forced_import(db, &format!("price_cagr_{name}"), version, indexes)
        })?;

        let price_return_24h_sd = PriceReturn24hSdVecs {
            _1w: StdDevPerBlock::forced_import(
                db,
                "price_return_24h",
                "1w",
                7,
                version + v1,
                indexes,
            )?,
            _1m: StdDevPerBlock::forced_import(
                db,
                "price_return_24h",
                "1m",
                30,
                version + v1,
                indexes,
            )?,
            _1y: StdDevPerBlock::forced_import(
                db,
                "price_return_24h",
                "1y",
                365,
                version + v1,
                indexes,
            )?,
        };

        Ok(Self {
            periods: price_return,
            cagr: price_cagr,
            sd_24h: price_return_24h_sd,
        })
    }
}
