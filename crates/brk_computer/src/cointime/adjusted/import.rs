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
        let last = || VecBuilderOptions::default().add_last();

        macro_rules! computed_di {
            ($name:expr) => {
                ComputedVecsFromDateIndex::forced_import(
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
            indexes_to_cointime_adj_inflation_rate: computed_di!("cointime_adj_inflation_rate"),
            indexes_to_cointime_adj_tx_btc_velocity: computed_di!("cointime_adj_tx_btc_velocity"),
            indexes_to_cointime_adj_tx_usd_velocity: computed_di!("cointime_adj_tx_usd_velocity"),
        })
    }
}
