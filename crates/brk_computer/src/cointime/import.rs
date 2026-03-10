use std::path::Path;

use brk_error::Result;
use brk_types::Version;

use crate::{
    indexes,
    internal::{finalize_db, open_db},
};

use super::{
    ActivityVecs, AdjustedVecs, CapVecs, DB_NAME, PricesVecs, ReserveRiskVecs, SupplyVecs,
    ValueVecs, Vecs,
};

impl Vecs {
    pub(crate) fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let db = open_db(parent_path, DB_NAME, 1_000_000)?;
        let version = parent_version;
        let v1 = version + Version::ONE;
        let activity = ActivityVecs::forced_import(&db, version, indexes)?;
        let supply = SupplyVecs::forced_import(&db, v1, indexes)?;
        let value = ValueVecs::forced_import(&db, v1, indexes)?;
        let cap = CapVecs::forced_import(&db, v1, indexes)?;
        let prices = PricesVecs::forced_import(&db, version, indexes)?;
        let adjusted = AdjustedVecs::forced_import(&db, version, indexes)?;
        let reserve_risk = ReserveRiskVecs::forced_import(&db, v1, indexes)?;

        let this = Self {
            db,
            activity,
            supply,
            value,
            cap,
            prices,
            adjusted,
            reserve_risk,
        };
        finalize_db(&this.db, &this)?;
        Ok(this)
    }
}
