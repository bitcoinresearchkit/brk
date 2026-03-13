use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{CachedWindowStarts, PerBlock, PerBlockAggregated},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        Ok(Self {
            total: PerBlockAggregated::forced_import(
                db,
                "output_count",
                version,
                indexes,
                cached_starts,
            )?,
            unspent: PerBlock::forced_import(
                db,
                "exact_utxo_count",
                version,
                indexes,
            )?,
        })
    }
}
