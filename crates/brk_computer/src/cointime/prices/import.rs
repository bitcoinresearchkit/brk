use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{PerBlock, PriceWithRatioExtendedPerBlock},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        macro_rules! import {
            ($name:expr) => {
                PriceWithRatioExtendedPerBlock::forced_import(db, $name, version, indexes)?
            };
        }

        Ok(Self {
            vaulted: import!("vaulted_price"),
            active: import!("active_price"),
            true_market_mean: import!("true_market_mean"),
            cointime: import!("cointime_price"),
            transfer: import!("transfer_price"),
            balanced: import!("balanced_price"),
            terminal: import!("terminal_price"),
            delta: import!("delta_price"),
            cumulative_market_cap: PerBlock::forced_import(
                db,
                "cumulative_market_cap",
                version,
                indexes,
            )?,
        })
    }
}
