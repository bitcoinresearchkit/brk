use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeightRatioExtended, Price},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let vaulted_price = Price::forced_import(db, "vaulted_price", version, indexes)?;
        let vaulted_price_ratio =
            ComputedFromHeightRatioExtended::forced_import(db, "vaulted_price", version, indexes)?;

        let active_price = Price::forced_import(db, "active_price", version, indexes)?;
        let active_price_ratio =
            ComputedFromHeightRatioExtended::forced_import(db, "active_price", version, indexes)?;

        let true_market_mean = Price::forced_import(db, "true_market_mean", version, indexes)?;
        let true_market_mean_ratio = ComputedFromHeightRatioExtended::forced_import(
            db,
            "true_market_mean",
            version,
            indexes,
        )?;

        let cointime_price = Price::forced_import(db, "cointime_price", version, indexes)?;
        let cointime_price_ratio =
            ComputedFromHeightRatioExtended::forced_import(db, "cointime_price", version, indexes)?;

        Ok(Self {
            vaulted_price,
            vaulted_price_ratio,
            active_price,
            active_price_ratio,
            true_market_mean,
            true_market_mean_ratio,
            cointime_price,
            cointime_price_ratio,
        })
    }
}
