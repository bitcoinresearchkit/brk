use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::PerBlock};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            native: PerBlock::forced_import(db, "velocity", version, indexes)?,
            fiat: PerBlock::forced_import(db, "velocity_fiat", version, indexes)?,
        })
    }
}
