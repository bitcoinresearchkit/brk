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
        let last = || VecBuilderOptions::default().add_last();
        let sum_cum = || VecBuilderOptions::default().add_sum().add_cumulative();

        macro_rules! computed_h {
            ($name:expr, $opts:expr) => {
                ComputedVecsFromHeight::forced_import(
                    db,
                    $name,
                    Source::Compute,
                    version,
                    indexes,
                    $opts,
                )?
            };
        }

        Ok(Self {
            indexes_to_coinblocks_created: computed_h!("coinblocks_created", sum_cum()),
            indexes_to_coinblocks_stored: computed_h!("coinblocks_stored", sum_cum()),
            indexes_to_liveliness: computed_h!("liveliness", last()),
            indexes_to_vaultedness: computed_h!("vaultedness", last()),
            indexes_to_activity_to_vaultedness_ratio: computed_h!(
                "activity_to_vaultedness_ratio",
                last()
            ),
        })
    }
}
