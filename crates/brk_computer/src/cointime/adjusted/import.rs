use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{PerBlock, PercentPerBlock},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            inflation_rate: PercentPerBlock::forced_import(
                db,
                "cointime_adj_inflation_rate",
                version + Version::ONE,
                indexes,
            )?,
            tx_velocity_native: PerBlock::forced_import(
                db,
                "cointime_adj_tx_velocity_btc",
                version,
                indexes,
            )?,
            tx_velocity_fiat: PerBlock::forced_import(
                db,
                "cointime_adj_tx_velocity_usd",
                version,
                indexes,
            )?,
        })
    }
}
