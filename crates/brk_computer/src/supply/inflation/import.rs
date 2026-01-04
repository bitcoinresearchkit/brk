use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedVecsFromDateIndex, Source, VecBuilderOptions},
};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let indexes_to_inflation_rate = ComputedVecsFromDateIndex::forced_import(
            db,
            "inflation_rate",
            Source::Compute,
            version,
            indexes,
            VecBuilderOptions::default().add_average(),
        )?;

        Ok(Self {
            indexes: indexes_to_inflation_rate,
        })
    }
}
