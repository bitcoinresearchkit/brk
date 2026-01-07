use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedTxDistribution, ValueDerivedTxFull},
    price,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let txindex_to_input_value = EagerVec::forced_import(db, "input_value", version)?;
        let txindex_to_output_value = EagerVec::forced_import(db, "output_value", version)?;
        let txindex_to_fee = EagerVec::forced_import(db, "fee", version)?;
        let txindex_to_fee_rate = EagerVec::forced_import(db, "fee_rate", version)?;

        Ok(Self {
            txindex_to_input_value,
            txindex_to_output_value,
            txindex_to_fee: txindex_to_fee.clone(),
            txindex_to_fee_rate: txindex_to_fee_rate.clone(),
            indexes_to_fee: ValueDerivedTxFull::forced_import(
                db,
                "fee",
                version,
                indexes,
                indexer,
                price,
                &txindex_to_fee,
            )?,
            indexes_to_fee_rate: ComputedTxDistribution::forced_import(
                db,
                "fee_rate",
                version,
                indexes,
            )?,
        })
    }
}
