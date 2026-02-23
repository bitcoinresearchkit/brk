use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::{indexes, internal::ComputedFromHeightLast};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v1 = version + Version::ONE;
        Ok(Self {
            vocdd_365d_median: EagerVec::forced_import(db, "vocdd_365d_median", v1)?,
            hodl_bank: EagerVec::forced_import(db, "hodl_bank", v1)?,
            reserve_risk: ComputedFromHeightLast::forced_import(db, "reserve_risk", v1, indexes)?,
        })
    }
}
