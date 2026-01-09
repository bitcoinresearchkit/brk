use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::{indexes, internal::{ComputedTxDistribution, ValueTxFull}, price};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        Ok(Self {
            input_value: EagerVec::forced_import(db, "input_value", version)?,
            output_value: EagerVec::forced_import(db, "output_value", version)?,
            fee: ValueTxFull::forced_import(db, "fee", version, indexes, indexer, price)?,
            fee_rate: ComputedTxDistribution::forced_import(db, "fee_rate", version, indexes)?,
        })
    }
}
