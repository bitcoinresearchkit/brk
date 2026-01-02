use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Version;
use vecdb::{Database, PAGE_SIZE};

use super::Vecs;
use crate::{distribution, indexes, price};

const VERSION: Version = Version::ZERO;

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

        // Circulating supply - lazy refs to distribution
        let circulating = super::circulating::Vecs::import(version, distribution);

        // Burned/unspendable supply - computed from scripts
        let burned = super::burned::Vecs::forced_import(&db, version, indexes, compute_dollars)?;

        // Inflation rate
        let inflation = super::inflation::Vecs::forced_import(&db, version, indexes)?;

        // Velocity
        let velocity =
            super::velocity::Vecs::forced_import(&db, version, indexes, compute_dollars)?;

        // Market cap - lazy refs to supply in USD
        let market_cap = super::market_cap::Vecs::import(version, distribution);

        let this = Self {
            db,
            circulating,
            burned,
            inflation,
            velocity,
            market_cap,
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
