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

        Ok(Self {
            indexes_to_tx_v1: ComputedVecsFromHeight::forced_import(
                db,
                "tx_v1",
                Source::Compute,
                version,
                indexes,
                sum_cum(),
            )?,
            indexes_to_tx_v2: ComputedVecsFromHeight::forced_import(
                db,
                "tx_v2",
                Source::Compute,
                version,
                indexes,
                sum_cum(),
            )?,
            indexes_to_tx_v3: ComputedVecsFromHeight::forced_import(
                db,
                "tx_v3",
                Source::Compute,
                version,
                indexes,
                sum_cum(),
            )?,
        })
    }
}
