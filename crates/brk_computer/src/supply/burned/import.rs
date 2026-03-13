use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::{AmountPerBlockCumulativeWithSums, CachedWindowStarts}};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        Ok(Self {
            opreturn: AmountPerBlockCumulativeWithSums::forced_import(
                db,
                "opreturn_supply",
                version,
                indexes,
                cached_starts,
            )?,
            unspendable: AmountPerBlockCumulativeWithSums::forced_import(
                db,
                "unspendable_supply",
                version,
                indexes,
                cached_starts,
            )?,
        })
    }
}
