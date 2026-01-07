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
        let indexes_to_opreturn_value = ValueBlockFull::forced_import(
            db,
            "opreturn_value",
            version,
            indexes,
            compute_dollars,
        )?;

        Ok(Self {
            indexes_to_opreturn_value,
        })
    }
}
