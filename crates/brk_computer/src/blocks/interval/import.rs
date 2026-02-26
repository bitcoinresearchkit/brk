use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedFromHeightDistribution};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let interval =
            ComputedFromHeightDistribution::forced_import(db, "block_interval", version, indexes)?;

        Ok(Self(interval))
    }
}
