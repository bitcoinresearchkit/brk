use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedFromHeightLast};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v2 = Version::TWO;

        Ok(Self {
            epoch: ComputedFromHeightLast::forced_import(db, "halving_epoch", version, indexes)?,
            blocks_before_next_halving: ComputedFromHeightLast::forced_import(
                db,
                "blocks_before_next_halving",
                version + v2,
                indexes,
            )?,
            days_before_next_halving: ComputedFromHeightLast::forced_import(
                db,
                "days_before_next_halving",
                version + v2,
                indexes,
            )?,
        })
    }
}
