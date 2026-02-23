use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ValueFromHeightSumCum, prices};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            opreturn: ValueFromHeightSumCum::forced_import(
                db,
                "opreturn_supply",
                version,
                indexes,
                prices,
            )?,
            unspendable: ValueFromHeightSumCum::forced_import(
                db,
                "unspendable_supply",
                version,
                indexes,
                prices,
            )?,
        })
    }
}
