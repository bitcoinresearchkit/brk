use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ValueFromHeightSumCum, price};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        Ok(Self {
            opreturn: ValueFromHeightSumCum::forced_import(
                db,
                "opreturn_supply",
                version,
                indexes,
                price,
            )?,
            unspendable: ValueFromHeightSumCum::forced_import(
                db,
                "unspendable_supply",
                version,
                indexes,
                price,
            )?,
        })
    }
}
