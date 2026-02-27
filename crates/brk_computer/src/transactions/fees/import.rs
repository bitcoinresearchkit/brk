use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::internal::ComputedFromTxDistribution;

/// Bump this when fee/feerate aggregation logic changes (e.g., skip coinbase).
const VERSION: Version = Version::new(2);

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let v = version + VERSION;
        Ok(Self {
            input_value: EagerVec::forced_import(db, "input_value", version)?,
            output_value: EagerVec::forced_import(db, "output_value", version)?,
            fee: ComputedFromTxDistribution::forced_import(db, "fee", v)?,
            fee_rate: ComputedFromTxDistribution::forced_import(db, "fee_rate", v)?,
        })
    }
}
