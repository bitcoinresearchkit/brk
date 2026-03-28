use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::super::lookback::ByLookbackPeriod;
use super::Vecs;
use crate::{
    indexes,
    internal::{PercentPerBlock, StdDevPerBlock, Windows},
    investing::ByDcaCagr,
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v1 = Version::ONE;

        let periods = ByLookbackPeriod::try_new(|name, _days| {
            PercentPerBlock::forced_import(db, &format!("price_return_{name}"), version, indexes)
        })?;

        let cagr = ByDcaCagr::try_new(|name, _days| {
            PercentPerBlock::forced_import(db, &format!("price_cagr_{name}"), version, indexes)
        })?;

        let mut days_iter = Windows::<()>::DAYS.iter();
        let sd_24h = Windows::try_from_fn(|suffix| {
            let days = *days_iter.next().unwrap();
            StdDevPerBlock::forced_import(
                db,
                "price_return_24h",
                suffix,
                days,
                version + v1,
                indexes,
            )
        })?;

        Ok(Self {
            periods,
            cagr,
            sd_24h,
        })
    }
}
