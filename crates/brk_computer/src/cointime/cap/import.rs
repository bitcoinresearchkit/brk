use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedFromHeightLast};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            thermo_cap: ComputedFromHeightLast::forced_import(db, "thermo_cap", version, indexes)?,
            investor_cap: ComputedFromHeightLast::forced_import(db, "investor_cap", version, indexes)?,
            vaulted_cap: ComputedFromHeightLast::forced_import(db, "vaulted_cap", version, indexes)?,
            active_cap: ComputedFromHeightLast::forced_import(db, "active_cap", version, indexes)?,
            cointime_cap: ComputedFromHeightLast::forced_import(db, "cointime_cap", version, indexes)?,
        })
    }
}
