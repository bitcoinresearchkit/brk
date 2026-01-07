use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedBlockLast};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            indexes_to_thermo_cap: ComputedBlockLast::forced_import(
                db,
                "thermo_cap",
                version,
                indexes,
            )?,
            indexes_to_investor_cap: ComputedBlockLast::forced_import(
                db,
                "investor_cap",
                version,
                indexes,
            )?,
            indexes_to_vaulted_cap: ComputedBlockLast::forced_import(
                db,
                "vaulted_cap",
                version,
                indexes,
            )?,
            indexes_to_active_cap: ComputedBlockLast::forced_import(
                db,
                "active_cap",
                version,
                indexes,
            )?,
            indexes_to_cointime_cap: ComputedBlockLast::forced_import(
                db,
                "cointime_cap",
                version,
                indexes,
            )?,
        })
    }
}
