use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{Full, RollingFull},
};

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            height: Full::forced_import(db, "input_count", version)?,
            rolling: RollingFull::forced_import(db, "input_count", version, indexes)?,
        })
    }
}
