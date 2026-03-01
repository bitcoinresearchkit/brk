use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::FiatFromHeightLast};

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            thermo_cap: FiatFromHeightLast::forced_import(db, "thermo_cap", version, indexes)?,
            investor_cap: FiatFromHeightLast::forced_import(db, "investor_cap", version, indexes)?,
            vaulted_cap: FiatFromHeightLast::forced_import(db, "vaulted_cap", version, indexes)?,
            active_cap: FiatFromHeightLast::forced_import(db, "active_cap", version, indexes)?,
            cointime_cap: FiatFromHeightLast::forced_import(db, "cointime_cap", version, indexes)?,
        })
    }
}
