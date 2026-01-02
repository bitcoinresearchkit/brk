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

        macro_rules! computed_h {
            ($name:expr) => {
                ComputedVecsFromHeight::forced_import(
                    db,
                    $name,
                    Source::Compute,
                    version,
                    indexes,
                    last(),
                )?
            };
        }

        Ok(Self {
            indexes_to_thermo_cap: computed_h!("thermo_cap"),
            indexes_to_investor_cap: computed_h!("investor_cap"),
            indexes_to_vaulted_cap: computed_h!("vaulted_cap"),
            indexes_to_active_cap: computed_h!("active_cap"),
            indexes_to_cointime_cap: computed_h!("cointime_cap"),
        })
    }
}
