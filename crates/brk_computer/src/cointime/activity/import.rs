use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeight, ComputedFromHeightCumulativeSum},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            coinblocks_created: ComputedFromHeightCumulativeSum::forced_import(
                db,
                "coinblocks_created",
                version,
                indexes,
            )?,
            coinblocks_stored: ComputedFromHeightCumulativeSum::forced_import(
                db,
                "coinblocks_stored",
                version,
                indexes,
            )?,
            liveliness: ComputedFromHeight::forced_import(db, "liveliness", version, indexes)?,
            vaultedness: ComputedFromHeight::forced_import(db, "vaultedness", version, indexes)?,
            activity_to_vaultedness_ratio: ComputedFromHeight::forced_import(
                db,
                "activity_to_vaultedness_ratio",
                version,
                indexes,
            )?,
        })
    }
}
