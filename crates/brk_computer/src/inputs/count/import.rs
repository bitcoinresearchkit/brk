use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::DerivedTxFull};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self(DerivedTxFull::forced_import(
            db,
            "input_count",
            version,
            indexes,
        )?))
    }
}
