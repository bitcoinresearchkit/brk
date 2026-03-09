use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::AmountFromHeightCumulative};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            opreturn: AmountFromHeightCumulative::forced_import(
                db,
                "opreturn_value",
                version,
                indexes,
            )?,
        })
    }
}
