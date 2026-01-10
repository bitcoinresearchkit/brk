use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::{indexes, internal::{ComputedFromTxDistribution, ValueFromTxFull}, price};

/// Bump this when fee/feerate aggregation logic changes (e.g., skip coinbase).
const VERSION: Version = Version::ONE;

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let v = version + VERSION;
        Ok(Self {
            input_value: EagerVec::forced_import(db, "input_value", version)?,
            output_value: EagerVec::forced_import(db, "output_value", version)?,
            fee: ValueFromTxFull::forced_import(db, "fee", v, indexes, indexer, price)?,
            fee_rate: ComputedFromTxDistribution::forced_import(db, "fee_rate", v, indexes)?,
        })
    }
}
