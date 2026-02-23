use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Version;
use vecdb::{Database, ReadableCloneableVec, PAGE_SIZE};

use super::Vecs;
use crate::{
    distribution, indexes,
    internal::{
        ComputedFromHeightLast, DifferenceF32, DollarsIdentity,
        LazyBinaryComputedFromHeightLast, LazyFromHeightLast, LazyValueFromHeightLast,
        SatsIdentity,
    },
    prices,
};

const VERSION: Version = Version::ONE;

impl Vecs {
    pub(crate) fn forced_import(
        parent: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
        distribution: &distribution::Vecs,
    ) -> Result<Self> {
        let db = Database::open(&parent.join(super::DB_NAME))?;
        db.set_min_len(PAGE_SIZE * 10_000_000)?;

        let version = parent_version + VERSION;
        let supply_metrics = &distribution.utxo_cohorts.all.metrics.supply;

        // Circulating supply - lazy refs to distribution
        let circulating = LazyValueFromHeightLast::from_block_source::<SatsIdentity, DollarsIdentity>(
            "circulating_supply",
            &supply_metrics.total,
            version,
        );

        // Burned/unspendable supply - computed from scripts
        let burned = super::burned::Vecs::forced_import(&db, version, indexes, prices)?;

        // Inflation rate
        let inflation =
            ComputedFromHeightLast::forced_import(&db, "inflation_rate", version, indexes)?;

        // Velocity
        let velocity =
            super::velocity::Vecs::forced_import(&db, version, indexes)?;

        // Market cap - lazy identity from distribution supply in USD
        let market_cap = LazyFromHeightLast::from_lazy_binary_computed::<DollarsIdentity, _, _>(
            "market_cap",
            version,
            supply_metrics.total.usd.height.read_only_boxed_clone(),
            &supply_metrics.total.usd,
        );

        // Growth rates
        let market_cap_growth_rate = ComputedFromHeightLast::forced_import(
            &db,
            "market_cap_growth_rate",
            version + Version::ONE,
            indexes,
        )?;
        let realized_cap_growth_rate = ComputedFromHeightLast::forced_import(
            &db,
            "realized_cap_growth_rate",
            version + Version::ONE,
            indexes,
        )?;
        let cap_growth_rate_diff =
            LazyBinaryComputedFromHeightLast::forced_import::<DifferenceF32>(
                "cap_growth_rate_diff",
                version,
                market_cap_growth_rate.height.read_only_boxed_clone(),
                realized_cap_growth_rate.height.read_only_boxed_clone(),
                indexes,
            );

        let this = Self {
            db,
            circulating,
            burned,
            inflation,
            velocity,
            market_cap,
            market_cap_growth_rate,
            realized_cap_growth_rate,
            cap_growth_rate_diff,
        };

        this.db.retain_regions(
            this.iter_any_exportable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;
        this.db.compact()?;

        Ok(this)
    }
}
