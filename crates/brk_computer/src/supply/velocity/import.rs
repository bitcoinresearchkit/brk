use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedVecsDateAverage};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        compute_dollars: bool,
    ) -> Result<Self> {
        Ok(Self {
            btc: ComputedVecsDateAverage::forced_import(db, "btc_velocity", version, indexes)?,
            usd: compute_dollars.then(|| {
                ComputedVecsDateAverage::forced_import(db, "usd_velocity", version, indexes)
                    .unwrap()
            }),
        })
    }
}
