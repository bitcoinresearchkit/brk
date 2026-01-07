use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedBlockSumCum};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            indexes_to_tx_v1: ComputedBlockSumCum::forced_import(
                db,
                "tx_v1",
                version,
                indexes,
            )?,
            indexes_to_tx_v2: ComputedBlockSumCum::forced_import(
                db,
                "tx_v2",
                version,
                indexes,
            )?,
            indexes_to_tx_v3: ComputedBlockSumCum::forced_import(
                db,
                "tx_v3",
                version,
                indexes,
            )?,
        })
    }
}
