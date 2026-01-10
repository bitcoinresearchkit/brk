use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedFromDateAverage};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self(ComputedFromDateAverage::forced_import(
            db,
            "inflation_rate",
            version,
            indexes,
        )?))
    }
}
