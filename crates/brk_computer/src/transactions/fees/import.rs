use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec, IterableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedValueVecsFromTxindex, ComputedVecsFromTxindex, Source, VecBuilderOptions},
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
        let stats = || {
            VecBuilderOptions::default()
                .add_average()
                .add_minmax()
                .add_percentiles()
        };

        let txindex_to_input_value = EagerVec::forced_import(db, "input_value", version)?;
        let txindex_to_output_value = EagerVec::forced_import(db, "output_value", version)?;
        let txindex_to_fee = EagerVec::forced_import(db, "fee", version)?;
        let txindex_to_fee_rate = EagerVec::forced_import(db, "fee_rate", version)?;

        Ok(Self {
            txindex_to_input_value,
            txindex_to_output_value,
            txindex_to_fee: txindex_to_fee.clone(),
            txindex_to_fee_rate: txindex_to_fee_rate.clone(),
            indexes_to_fee: ComputedValueVecsFromTxindex::forced_import(
                db,
                "fee",
                indexer,
                indexes,
                Source::Vec(txindex_to_fee.boxed_clone()),
                version,
                price,
                VecBuilderOptions::default()
                    .add_sum()
                    .add_cumulative()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
            )?,
            indexes_to_fee_rate: ComputedVecsFromTxindex::forced_import(
                db,
                "fee_rate",
                Source::Vec(txindex_to_fee_rate.boxed_clone()),
                version,
                indexes,
                stats(),
            )?,
        })
    }
}
