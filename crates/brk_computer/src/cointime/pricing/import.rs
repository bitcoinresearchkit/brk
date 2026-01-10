use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeightLast, ComputedFromDateRatio},
    price,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        macro_rules! computed_h {
            ($name:expr) => {
                ComputedFromHeightLast::forced_import(db, $name, version, indexes)?
            };
        }

        // Extract price vecs before struct literal so they can be used as sources for ratios
        let vaulted_price = computed_h!("vaulted_price");
        let active_price = computed_h!("active_price");
        let true_market_mean = computed_h!("true_market_mean");
        let cointime_price = computed_h!("cointime_price");

        macro_rules! ratio_di {
            ($name:expr, $source:expr) => {
                ComputedFromDateRatio::forced_import(
                    db,
                    $name,
                    Some($source),
                    version,
                    indexes,
                    true,
                    price,
                )?
            };
        }

        Ok(Self {
            vaulted_price_ratio: ratio_di!("vaulted_price", &vaulted_price),
            vaulted_price,
            active_price_ratio: ratio_di!("active_price", &active_price),
            active_price,
            true_market_mean_ratio: ratio_di!("true_market_mean", &true_market_mean),
            true_market_mean,
            cointime_price_ratio: ratio_di!("cointime_price", &cointime_price),
            cointime_price,
        })
    }
}
