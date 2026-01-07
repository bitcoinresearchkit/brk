use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedVecsDateAverage};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let indexes_to_inflation_rate = ComputedVecsDateAverage::forced_import(
            db,
            "inflation_rate",
            version,
            indexes,
        )?;

        Ok(Self {
            indexes: indexes_to_inflation_rate,
        })
    }
}
