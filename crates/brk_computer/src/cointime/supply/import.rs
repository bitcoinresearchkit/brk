use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::AmountPerBlock};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            vaulted: AmountPerBlock::forced_import(db, "vaulted_supply", version, indexes)?,
            active: AmountPerBlock::forced_import(db, "active_supply", version, indexes)?,
        })
    }
}
