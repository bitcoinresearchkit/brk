use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedDateLast};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            indexes_to_cointime_adj_inflation_rate: ComputedDateLast::forced_import(
                db,
                "cointime_adj_inflation_rate",
                version,
                indexes,
            )?,
            indexes_to_cointime_adj_tx_btc_velocity: ComputedDateLast::forced_import(
                db,
                "cointime_adj_tx_btc_velocity",
                version,
                indexes,
            )?,
            indexes_to_cointime_adj_tx_usd_velocity: ComputedDateLast::forced_import(
                db,
                "cointime_adj_tx_usd_velocity",
                version,
                indexes,
            )?,
        })
    }
}
