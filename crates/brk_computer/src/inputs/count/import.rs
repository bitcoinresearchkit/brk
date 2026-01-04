use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, IterableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedVecsFromTxindex, Source, VecBuilderOptions},
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
                "input_count",
                Source::Vec(indexes.transaction.txindex_to_input_count.boxed_clone()),
                version,
                indexes,
                full_stats(),
            )?,
        })
    }
}
