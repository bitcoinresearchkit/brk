use std::path::Path;

use brk_error::Result;
use brk_types::{Cents, Dollars, Sats, Version};

use crate::{
    distribution, indexes,
    internal::{
        FiatRollingDelta, Identity, LazyFiatPerBlock, LazyAmountPerBlock, PercentPerBlock,
        RollingWindows, SatsToBitcoin, finalize_db, open_db,
    },
};

use super::Vecs;

const VERSION: Version = Version::ONE;

impl Vecs {
    pub(crate) fn forced_import(
        parent: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
        distribution: &distribution::Vecs,
    ) -> Result<Self> {
        let db = open_db(parent, super::DB_NAME, 10_000_000)?;

        let version = parent_version + VERSION;
        let supply_metrics = &distribution.utxo_cohorts.all.metrics.supply;

        // Circulating supply - lazy refs to distribution
        let circulating = LazyAmountPerBlock::from_block_source::<
            Identity<Sats>,
            SatsToBitcoin,
            Identity<Cents>,
            Identity<Dollars>,
        >("circulating_supply", &supply_metrics.total, version);

        // Burned/unspendable supply - computed from scripts
        let burned = super::burned::Vecs::forced_import(&db, version, indexes)?;

        // Inflation rate
        let inflation_rate =
            PercentPerBlock::forced_import(&db, "inflation_rate", version, indexes)?;

        // Velocity
        let velocity = super::velocity::Vecs::forced_import(&db, version, indexes)?;

        // Market cap - lazy fiat (cents + usd) from distribution supply
        let market_cap =
            LazyFiatPerBlock::from_computed("market_cap", version, &supply_metrics.total.cents);

        // Market cap delta (change + rate across 4 windows)
        let market_cap_delta = FiatRollingDelta::forced_import(
            &db,
            "market_cap_delta",
            version + Version::new(3),
            indexes,
        )?;

        let market_minus_realized_cap_growth_rate = RollingWindows::forced_import(
            &db,
            "market_minus_realized_cap_growth_rate",
            version + Version::TWO,
            indexes,
        )?;

        let this = Self {
            db,
            circulating,
            burned,
            inflation_rate,
            velocity,
            market_cap,
            market_cap_delta,
            market_minus_realized_cap_growth_rate,
        };
        finalize_db(&this.db, &this)?;
        Ok(this)
    }
}
