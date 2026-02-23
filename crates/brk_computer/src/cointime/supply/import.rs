use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ValueFromHeightLast, prices};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            vaulted_supply: ValueFromHeightLast::forced_import(
                db,
                "vaulted_supply",
                version,
                indexes,
                prices,
            )?,
            active_supply: ValueFromHeightLast::forced_import(
                db,
                "active_supply",
                version,
                indexes,
                prices,
            )?,
        })
    }
}
