use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedValueVecsFromHeight, Source, VecBuilderOptions},
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        compute_dollars: bool,
    ) -> Result<Self> {
        let last = || VecBuilderOptions::default().add_last();

        macro_rules! value_h {
            ($name:expr) => {
                ComputedValueVecsFromHeight::forced_import(
                    db,
                    $name,
                    Source::Compute,
                    version,
                    last(),
                    compute_dollars,
                    indexes,
                )?
            };
        }

        Ok(Self {
            indexes_to_vaulted_supply: value_h!("vaulted_supply"),
            indexes_to_active_supply: value_h!("active_supply"),
        })
    }
}
