use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedFromHeightLast};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            btc: ComputedFromHeightLast::forced_import(db, "btc_velocity", version, indexes)?,
            usd: ComputedFromHeightLast::forced_import(db, "usd_velocity", version, indexes)?,
        })
    }
}
