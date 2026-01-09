use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ValueBlockFull};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        compute_dollars: bool,
    ) -> Result<Self> {
        Ok(Self {
            opreturn: ValueBlockFull::forced_import(
                db,
                "opreturn_value",
                version,
                indexes,
                compute_dollars,
            )?,
        })
    }
}
