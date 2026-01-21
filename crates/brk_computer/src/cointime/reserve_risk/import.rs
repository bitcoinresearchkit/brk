use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::{indexes, internal::ComputedFromDateLast};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        compute_dollars: bool,
    ) -> Result<Self> {
        Ok(Self {
            vocdd_365d_sma: EagerVec::forced_import(db, "vocdd_365d_sma", version)?,
            hodl_bank: EagerVec::forced_import(db, "hodl_bank", version)?,
            reserve_risk: compute_dollars
                .then(|| ComputedFromDateLast::forced_import(db, "reserve_risk", version, indexes))
                .transpose()?,
        })
    }
}
