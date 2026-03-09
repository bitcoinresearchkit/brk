use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::AmountPerBlockCumulativeSum};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            opreturn: AmountPerBlockCumulativeSum::forced_import(
                db,
                "opreturn_supply",
                version,
                indexes,
            )?,
            unspendable: AmountPerBlockCumulativeSum::forced_import(
                db,
                "unspendable_supply",
                version,
                indexes,
            )?,
        })
    }
}
