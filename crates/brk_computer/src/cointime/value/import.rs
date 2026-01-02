use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedVecsFromHeight, Source, VecBuilderOptions},
};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let sum_cum = || VecBuilderOptions::default().add_sum().add_cumulative();

        macro_rules! computed_h {
            ($name:expr) => {
                ComputedVecsFromHeight::forced_import(
                    db,
                    $name,
                    Source::Compute,
                    version,
                    indexes,
                    sum_cum(),
                )?
            };
        }

        Ok(Self {
            indexes_to_cointime_value_destroyed: computed_h!("cointime_value_destroyed"),
            indexes_to_cointime_value_created: computed_h!("cointime_value_created"),
            indexes_to_cointime_value_stored: computed_h!("cointime_value_stored"),
        })
    }
}
