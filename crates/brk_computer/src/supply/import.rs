use std::path::Path;

use brk_error::Result;
use brk_types::Version;

use crate::{
    cointime, distribution, indexes,
    internal::{
        CachedWindowStarts, LazyAmountPerBlock, LazyFiatPerBlock, LazyRollingDeltasFiatFromHeight,
        PercentPerBlock, RollingWindows,
        db_utils::{finalize_db, open_db},
    },
    supply::burned,
};

use super::Vecs;

const VERSION: Version = Version::ONE;

impl Vecs {
    pub(crate) fn forced_import(
        parent: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
        distribution: &distribution::Vecs,
        cointime: &cointime::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let db = open_db(parent, super::DB_NAME, 1_000_000)?;

        let version = parent_version + VERSION;
        let supply_metrics = &distribution.utxo_cohorts.all.metrics.supply;

        let circulating =
            LazyAmountPerBlock::identity("circulating_supply", &supply_metrics.total, version);

        let burned = burned::Vecs::forced_import(&db, version, indexes)?;

        // Inflation rate
        let inflation_rate =
            PercentPerBlock::forced_import(&db, "inflation_rate", version + Version::ONE, indexes)?;

        // Velocity
        let velocity = super::velocity::Vecs::forced_import(&db, version, indexes)?;

        // Market cap - lazy fiat (cents + usd) from distribution supply
        let market_cap =
            LazyFiatPerBlock::from_computed("market_cap", version, &supply_metrics.total.cents);

        // Market cap delta (change + rate across 4 windows)
        let market_cap_delta = LazyRollingDeltasFiatFromHeight::new(
            "market_cap_delta",
            version + Version::new(3),
            &market_cap.cents.height,
            cached_starts,
            indexes,
        );

        let market_minus_realized_cap_growth_rate = RollingWindows::forced_import(
            &db,
            "market_minus_realized_cap_growth_rate",
            version + Version::TWO,
            indexes,
        )?;

        let hodled_or_lost = LazyAmountPerBlock::identity(
            "hodled_or_lost_supply",
            &cointime.supply.vaulted,
            version,
        );

        let this = Self {
            db,
            circulating,
            burned,
            inflation_rate,
            velocity,
            market_cap,
            market_cap_delta,
            market_minus_realized_cap_growth_rate,
            hodled_or_lost,
        };
        finalize_db(&this.db, &this)?;
        Ok(this)
    }
}
