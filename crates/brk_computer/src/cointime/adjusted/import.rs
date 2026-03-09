use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedPerBlock, PercentPerBlock},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            adj_inflation_rate: PercentPerBlock::forced_import(
                db,
                "cointime_adj_inflation_rate",
                version,
                indexes,
            )?,
            adj_tx_velocity_btc: ComputedPerBlock::forced_import(
                db,
                "cointime_adj_tx_velocity_btc",
                version,
                indexes,
            )?,
            adj_tx_velocity_usd: ComputedPerBlock::forced_import(
                db,
                "cointime_adj_tx_velocity_usd",
                version,
                indexes,
            )?,
        })
    }
}
