use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{Bps32ToFloat, Bps32ToPercent, ComputedFromHeight, PercentFromHeight},
};

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            cointime_adj_inflation_rate: PercentFromHeight::forced_import::<Bps32ToFloat, Bps32ToPercent>(
                db,
                "cointime_adj_inflation_rate",
                version,
                indexes,
            )?,
            cointime_adj_tx_velocity_btc: ComputedFromHeight::forced_import(
                db,
                "cointime_adj_tx_velocity_btc",
                version,
                indexes,
            )?,
            cointime_adj_tx_velocity_usd: ComputedFromHeight::forced_import(
                db,
                "cointime_adj_tx_velocity_usd",
                version,
                indexes,
            )?,
        })
    }
}
