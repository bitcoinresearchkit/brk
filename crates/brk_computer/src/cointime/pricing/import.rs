use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes, price,
    internal::{ComputedRatioVecsFromDateIndex, ComputedVecsFromHeight, Source, VecBuilderOptions},
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let last = || VecBuilderOptions::default().add_last();

        macro_rules! computed_h {
            ($name:expr) => {
                ComputedVecsFromHeight::forced_import(
                    db,
                    $name,
                    Source::Compute,
                    version,
                    indexes,
                    last(),
                )?
            };
        }

        // Extract price vecs before struct literal so they can be used as sources for ratios
        let indexes_to_vaulted_price = computed_h!("vaulted_price");
        let indexes_to_active_price = computed_h!("active_price");
        let indexes_to_true_market_mean = computed_h!("true_market_mean");
        let indexes_to_cointime_price = computed_h!("cointime_price");

        macro_rules! ratio_di {
            ($name:expr, $source:expr) => {
                ComputedRatioVecsFromDateIndex::forced_import(
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
            indexes_to_vaulted_price_ratio: ratio_di!("vaulted_price", &indexes_to_vaulted_price),
            indexes_to_vaulted_price,
            indexes_to_active_price_ratio: ratio_di!("active_price", &indexes_to_active_price),
            indexes_to_active_price,
            indexes_to_true_market_mean_ratio: ratio_di!(
                "true_market_mean",
                &indexes_to_true_market_mean
            ),
            indexes_to_true_market_mean,
            indexes_to_cointime_price_ratio: ratio_di!(
                "cointime_price",
                &indexes_to_cointime_price
            ),
            indexes_to_cointime_price,
        })
    }
}
