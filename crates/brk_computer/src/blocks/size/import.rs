use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeightCumulativeFull, ComputedHeightDerivedCumulativeFull},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            vbytes: ComputedFromHeightCumulativeFull::forced_import(
                db,
                "block_vbytes",
                version,
                indexes,
            )?,
            size: ComputedHeightDerivedCumulativeFull::forced_import(
                db,
                "block_size",
                version,
                indexes,
            )?,
        })
    }
}
