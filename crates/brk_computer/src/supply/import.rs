use std::path::Path;

use brk_error::Result;
use brk_types::{Cents, Dollars, Sats, Version};

use crate::{
    distribution, indexes,
    internal::{
        Identity, LazyFromHeight, LazyValueFromHeight, PercentFromHeight, PercentRollingWindows,
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
        let circulating = LazyValueFromHeight::from_block_source::<
            Identity<Sats>,
            SatsToBitcoin,
            Identity<Cents>,
            Identity<Dollars>,
        >("circulating_supply", &supply_metrics.total, version);

        // Burned/unspendable supply - computed from scripts
        let burned = super::burned::Vecs::forced_import(&db, version, indexes)?;

        // Inflation rate
        let inflation_rate =
            PercentFromHeight::forced_import(&db, "inflation_rate", version, indexes)?;

        // Velocity
        let velocity = super::velocity::Vecs::forced_import(&db, version, indexes)?;

        // Market cap - lazy identity from distribution supply in USD
        let market_cap = LazyFromHeight::from_lazy::<Identity<Dollars>, Cents>(
            "market_cap",
            version,
            &supply_metrics.total.usd,
        );

        // Growth rates (4 windows: 24h, 1w, 1m, 1y)
        let market_cap_growth_rate = PercentRollingWindows::forced_import(
            &db,
            "market_cap_growth_rate",
            version + Version::TWO,
            indexes,
        )?;
        let realized_cap_growth_rate = PercentRollingWindows::forced_import(
            &db,
            "realized_cap_growth_rate",
            version + Version::TWO,
            indexes,
        )?;
        let market_minus_realized_cap_growth_rate = RollingWindows::forced_import(
            &db,
            "market_minus_realized_cap_growth_rate",
            version + Version::ONE,
            indexes,
        )?;

        let this = Self {
            db,
            circulating,
            burned,
            inflation_rate,
            velocity,
            market_cap,
            market_cap_growth_rate,
            realized_cap_growth_rate,
            market_minus_realized_cap_growth_rate,
        };
        finalize_db(&this.db, &this)?;
        Ok(this)
    }
}
