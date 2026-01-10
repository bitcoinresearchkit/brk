use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedFromDateAverage};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        compute_dollars: bool,
    ) -> Result<Self> {
        Ok(Self {
            btc: ComputedFromDateAverage::forced_import(db, "btc_velocity", version, indexes)?,
            usd: compute_dollars.then(|| {
                ComputedFromDateAverage::forced_import(db, "usd_velocity", version, indexes)
                    .unwrap()
            }),
        })
    }
}
