use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::PriceWithRatioExtendedPerBlock,
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
        })
    }
}
