use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedBlockFull, DerivedTxFull},
};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            total_count: DerivedTxFull::forced_import(db, "output_count", version, indexes)?,
            utxo_count: ComputedBlockFull::forced_import(db, "exact_utxo_count", version, indexes)?,
        })
    }
}
