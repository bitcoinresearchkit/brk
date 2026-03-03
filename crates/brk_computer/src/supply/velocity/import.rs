use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedFromHeight};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            btc: ComputedFromHeight::forced_import(db, "velocity_btc", version, indexes)?,
            usd: ComputedFromHeight::forced_import(db, "velocity_usd", version, indexes)?,
        })
    }
}
