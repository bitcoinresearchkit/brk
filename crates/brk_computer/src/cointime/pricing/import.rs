use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedPerBlock, RatioPerBlockExtended, Price},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let vaulted_price = Price::forced_import(db, "vaulted_price", version, indexes)?;
        let vaulted_price_ratio =
            RatioPerBlockExtended::forced_import(db, "vaulted_price", version, indexes)?;

        let active_price = Price::forced_import(db, "active_price", version, indexes)?;
        let active_price_ratio =
            RatioPerBlockExtended::forced_import(db, "active_price", version, indexes)?;

        let true_market_mean = Price::forced_import(db, "true_market_mean", version, indexes)?;
        let true_market_mean_ratio = RatioPerBlockExtended::forced_import(
            db,
            "true_market_mean",
            version,
            indexes,
        )?;

        let cointime_price = Price::forced_import(db, "cointime_price", version, indexes)?;
        let cointime_price_ratio =
            RatioPerBlockExtended::forced_import(db, "cointime_price", version, indexes)?;

        let transfer_price = Price::forced_import(db, "transfer_price", version, indexes)?;
        let transfer_price_ratio =
            RatioPerBlockExtended::forced_import(db, "transfer_price", version, indexes)?;

        let balanced_price = Price::forced_import(db, "balanced_price", version, indexes)?;
        let balanced_price_ratio =
            RatioPerBlockExtended::forced_import(db, "balanced_price", version, indexes)?;

        let terminal_price = Price::forced_import(db, "terminal_price", version, indexes)?;
        let terminal_price_ratio =
            RatioPerBlockExtended::forced_import(db, "terminal_price", version, indexes)?;

        let delta_price = Price::forced_import(db, "delta_price", version, indexes)?;
        let delta_price_ratio =
            RatioPerBlockExtended::forced_import(db, "delta_price", version, indexes)?;

        let cumulative_market_cap =
            ComputedPerBlock::forced_import(db, "cumulative_market_cap", version, indexes)?;

        Ok(Self {
            vaulted_price,
            vaulted_price_ratio,
            active_price,
            active_price_ratio,
            true_market_mean,
            true_market_mean_ratio,
            cointime_price,
            cointime_price_ratio,
            transfer_price,
            transfer_price_ratio,
            balanced_price,
            balanced_price_ratio,
            terminal_price,
            terminal_price_ratio,
            delta_price,
            delta_price_ratio,
            cumulative_market_cap,
        })
    }
}
