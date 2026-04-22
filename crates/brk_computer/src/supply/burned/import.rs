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
            total: ValuePerBlockCumulative::forced_import(
                db,
                "unspendable_supply",
                version,
                indexes,
            )?,
        })
    }
}
