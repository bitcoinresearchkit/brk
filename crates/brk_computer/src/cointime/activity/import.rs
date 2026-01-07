use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedBlockLast, ComputedBlockSumCum},
};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            indexes_to_coinblocks_created: ComputedBlockSumCum::forced_import(
                db,
                "coinblocks_created",
                version,
                indexes,
            )?,
            indexes_to_coinblocks_stored: ComputedBlockSumCum::forced_import(
                db,
                "coinblocks_stored",
                version,
                indexes,
            )?,
            indexes_to_liveliness: ComputedBlockLast::forced_import(
                db,
                "liveliness",
                version,
                indexes,
            )?,
            indexes_to_vaultedness: ComputedBlockLast::forced_import(
                db,
                "vaultedness",
                version,
                indexes,
            )?,
            indexes_to_activity_to_vaultedness_ratio: ComputedBlockLast::forced_import(
                db,
                "activity_to_vaultedness_ratio",
                version,
                indexes,
            )?,
        })
    }
}
