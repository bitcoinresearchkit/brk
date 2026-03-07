use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ValueFromHeightCumulative};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            opreturn: ValueFromHeightCumulative::forced_import(
                db,
                "opreturn_value",
                version,
                indexes,
            )?,
        })
    }
}
