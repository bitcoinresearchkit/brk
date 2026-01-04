use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, IterableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedVecsFromHeight, ComputedVecsFromTxindex, Source, VecBuilderOptions},
};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let full_stats = || {
            VecBuilderOptions::default()
                .add_average()
                .add_minmax()
                .add_percentiles()
                .add_sum()
                .add_cumulative()
        };

        Ok(Self {
            indexes_to_count: ComputedVecsFromTxindex::forced_import(
                db,
                "output_count",
                Source::Vec(indexes.transaction.txindex_to_output_count.boxed_clone()),
                version,
                indexes,
                full_stats(),
            )?,
            indexes_to_utxo_count: ComputedVecsFromHeight::forced_import(
                db,
                "exact_utxo_count",
                Source::Compute,
                version,
                indexes,
                full_stats(),
            )?,
        })
    }
}
