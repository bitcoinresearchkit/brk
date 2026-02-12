use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Version;
use vecdb::{Database, IterableCloneableVec, LazyVecFrom2, PAGE_SIZE};

use super::Vecs;
use crate::{
    distribution, indexes, price,
    internal::{
        ComputedFromDateAverage, ComputedFromDateLast, DifferenceF32, DollarsIdentity,
        LazyFromHeightLast, LazyValueFromHeightLast, SatsIdentity,
    },
};

const VERSION: Version = Version::ONE;

impl Vecs {
    pub fn forced_import(
        parent: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        distribution: &distribution::Vecs,
    ) -> Result<Self> {
        let db = Database::open(&parent.join(super::DB_NAME))?;
        db.set_min_len(PAGE_SIZE * 10_000_000)?;

        let version = parent_version + VERSION;
        let compute_dollars = price.is_some();

        let supply_metrics = &distribution.utxo_cohorts.all.metrics.supply;

        // Circulating supply - lazy refs to distribution
        let circulating = LazyValueFromHeightLast::from_block_source::<SatsIdentity, DollarsIdentity>(
            "circulating_supply",
            &supply_metrics.total,
            version,
        );

        // Burned/unspendable supply - computed from scripts
        let burned = super::burned::Vecs::forced_import(&db, version, indexes, price)?;

        // Inflation rate
        let inflation =
            ComputedFromDateAverage::forced_import(&db, "inflation_rate", version, indexes)?;

        // Velocity
        let velocity =
            super::velocity::Vecs::forced_import(&db, version, indexes, compute_dollars)?;

        // Market cap - lazy identity from distribution supply in USD
        let market_cap = supply_metrics.total.dollars.as_ref().map(|d| {
            LazyFromHeightLast::from_lazy_binary_computed::<DollarsIdentity, _, _>(
                "market_cap",
                version,
                d.height.boxed_clone(),
                d,
            )
        });

        // Growth rates
        let market_cap_growth_rate =
            ComputedFromDateLast::forced_import(&db, "market_cap_growth_rate", version, indexes)?;
        let realized_cap_growth_rate =
            ComputedFromDateLast::forced_import(&db, "realized_cap_growth_rate", version, indexes)?;
        let cap_growth_rate_diff = LazyVecFrom2::transformed::<DifferenceF32>(
            "cap_growth_rate_diff",
            version,
            market_cap_growth_rate.dateindex.boxed_clone(),
            realized_cap_growth_rate.dateindex.boxed_clone(),
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
