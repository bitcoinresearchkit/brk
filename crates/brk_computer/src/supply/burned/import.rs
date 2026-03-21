use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::AmountPerBlockCumulative};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            total: AmountPerBlockCumulative::forced_import(
                db,
                "unspendable_supply",
                version,
                indexes,
            )?,
        })
    }
}
