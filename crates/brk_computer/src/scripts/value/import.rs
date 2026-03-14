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
            op_return: AmountPerBlockCumulativeWithSums::forced_import(
                db,
                "op_return_value",
                version,
                indexes,
                cached_starts,
            )?,
        })
    }
}
