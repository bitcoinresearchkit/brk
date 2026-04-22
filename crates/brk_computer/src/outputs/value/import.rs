use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ValuePerBlockCumulative};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            op_return: ValuePerBlockCumulative::forced_import(
                db,
                "op_return_value",
                version,
                indexes,
            )?,
        })
    }
}
