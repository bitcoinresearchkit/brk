use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::{indexes, internal::PerTxDistribution};

/// Bump this when fee/feerate aggregation logic changes (e.g., skip coinbase).
const VERSION: Version = Version::new(2);

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;
        Ok(Self {
            input_value: EagerVec::forced_import(db, "input_value", version)?,
            output_value: EagerVec::forced_import(db, "output_value", version)?,
            fee: PerTxDistribution::forced_import(db, "fee", v, indexes)?,
            fee_rate: PerTxDistribution::forced_import(db, "fee_rate", v, indexes)?,
            effective_fee_rate: PerTxDistribution::forced_import(
                db,
                "effective_fee_rate",
                v,
                indexes,
            )?,
        })
    }
}
