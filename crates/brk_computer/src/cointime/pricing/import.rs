use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromDateRatio, PriceFromHeight},
    price,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let vaulted_price = PriceFromHeight::forced_import(db, "vaulted_price", version, indexes)?;
        let vaulted_price_ratio = ComputedFromDateRatio::forced_import(
            db,
            "vaulted_price",
            Some(&vaulted_price),
            version,
            indexes,
            true,
            price,
        )?;

        let active_price = PriceFromHeight::forced_import(db, "active_price", version, indexes)?;
        let active_price_ratio = ComputedFromDateRatio::forced_import(
            db,
            "active_price",
            Some(&active_price),
            version,
            indexes,
            true,
            price,
        )?;

        let true_market_mean =
            PriceFromHeight::forced_import(db, "true_market_mean", version, indexes)?;
        let true_market_mean_ratio = ComputedFromDateRatio::forced_import(
            db,
            "true_market_mean",
            Some(&true_market_mean),
            version,
            indexes,
            true,
            price,
        )?;

        let cointime_price =
            PriceFromHeight::forced_import(db, "cointime_price", version, indexes)?;
        let cointime_price_ratio = ComputedFromDateRatio::forced_import(
            db,
            "cointime_price",
            Some(&cointime_price),
            version,
            indexes,
            true,
            price,
        )?;

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
