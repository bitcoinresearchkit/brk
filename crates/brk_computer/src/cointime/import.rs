use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Version;
use vecdb::{Database, PAGE_SIZE};

use super::{
    ActivityVecs, AdjustedVecs, CapVecs, DB_NAME, PricingVecs, ReserveRiskVecs, SupplyVecs,
    VERSION, ValueVecs, Vecs,
};
use crate::{indexes, prices};

impl Vecs {
    pub(crate) fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
    ) -> Result<Self> {
        let db = Database::open(&parent_path.join(DB_NAME))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;

        let version = parent_version + VERSION;
        let v1 = version + Version::ONE;
        let activity = ActivityVecs::forced_import(&db, version, indexes)?;
        let supply = SupplyVecs::forced_import(&db, v1, indexes, prices)?;
        let value = ValueVecs::forced_import(&db, v1, indexes)?;
        let cap = CapVecs::forced_import(&db, v1, indexes)?;
        let pricing = PricingVecs::forced_import(&db, version, indexes)?;
        let adjusted = AdjustedVecs::forced_import(&db, version, indexes)?;
        let reserve_risk = ReserveRiskVecs::forced_import(&db, v1, indexes)?;

        let this = Self {
            db,
            activity,
            supply,
            value,
            cap,
            pricing,
            adjusted,
            reserve_risk,
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
