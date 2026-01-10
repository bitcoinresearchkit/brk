use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedFromDateLast};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            cointime_adj_inflation_rate: ComputedFromDateLast::forced_import(
                db,
                "cointime_adj_inflation_rate",
                version,
                indexes,
            )?,
            cointime_adj_tx_btc_velocity: ComputedFromDateLast::forced_import(
                db,
                "cointime_adj_tx_btc_velocity",
                version,
                indexes,
            )?,
            cointime_adj_tx_usd_velocity: ComputedFromDateLast::forced_import(
                db,
                "cointime_adj_tx_usd_velocity",
                version,
                indexes,
            )?,
        })
    }
}
