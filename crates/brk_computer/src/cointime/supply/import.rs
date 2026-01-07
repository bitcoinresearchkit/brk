use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::ValueBlockLast,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        compute_dollars: bool,
    ) -> Result<Self> {
        Ok(Self {
            indexes_to_vaulted_supply: ValueBlockLast::forced_import(
                db,
                "vaulted_supply",
                version,
                indexes,
                compute_dollars,
            )?,
            indexes_to_active_supply: ValueBlockLast::forced_import(
                db,
                "active_supply",
                version,
                indexes,
                compute_dollars,
            )?,
        })
    }
}
