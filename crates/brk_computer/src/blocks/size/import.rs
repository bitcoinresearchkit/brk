use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeightFull, ComputedHeightDerivedFull},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            vbytes: ComputedFromHeightFull::forced_import(
                db,
                "block_vbytes",
                version,
                indexes,
            )?,
            size: ComputedHeightDerivedFull::forced_import(
                db,
                "block_size",
                version,
                indexes,
            )?,
        })
    }
}
