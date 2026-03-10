use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedPerBlock, PriceWithRatioExtendedPerBlock},
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
            vaulted_price: import!("vaulted_price"),
            active_price: import!("active_price"),
            true_market_mean: import!("true_market_mean"),
            cointime_price: import!("cointime_price"),
            transfer_price: import!("transfer_price"),
            balanced_price: import!("balanced_price"),
            terminal_price: import!("terminal_price"),
            delta_price: import!("delta_price"),
            cumulative_market_cap: ComputedPerBlock::forced_import(
                db,
                "cumulative_market_cap",
                version,
                indexes,
            )?,
        })
    }
}
