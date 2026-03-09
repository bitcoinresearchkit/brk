use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{CentsTimesTenths, PriceWithRatioPerBlock, Price},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        macro_rules! import {
            ($name:expr) => {
                PriceWithRatioPerBlock::forced_import(
                    db, $name, version, indexes,
                )?
            };
        }

        let price_sma_200d = import!("price_sma_200d");
        let price_sma_350d = import!("price_sma_350d");

        let price_sma_200d_source = &price_sma_200d.price.cents;
        let price_sma_200d_x2_4 = Price::from_cents_source::<CentsTimesTenths<24>>(
            "price_sma_200d_x2_4",
            version,
            price_sma_200d_source,
        );
        let price_sma_200d_x0_8 = Price::from_cents_source::<CentsTimesTenths<8>>(
            "price_sma_200d_x0_8",
            version,
            price_sma_200d_source,
        );

        let price_sma_350d_source = &price_sma_350d.price.cents;
        let price_sma_350d_x2 = Price::from_cents_source::<CentsTimesTenths<20>>(
            "price_sma_350d_x2",
            version,
            price_sma_350d_source,
        );

        Ok(Self {
            price_sma_1w: import!("price_sma_1w"),
            price_sma_8d: import!("price_sma_8d"),
            price_sma_13d: import!("price_sma_13d"),
            price_sma_21d: import!("price_sma_21d"),
            price_sma_1m: import!("price_sma_1m"),
            price_sma_34d: import!("price_sma_34d"),
            price_sma_55d: import!("price_sma_55d"),
            price_sma_89d: import!("price_sma_89d"),
            price_sma_111d: import!("price_sma_111d"),
            price_sma_144d: import!("price_sma_144d"),
            price_sma_200d,
            price_sma_350d,
            price_sma_1y: import!("price_sma_1y"),
            price_sma_2y: import!("price_sma_2y"),
            price_sma_200w: import!("price_sma_200w"),
            price_sma_4y: import!("price_sma_4y"),

            price_ema_1w: import!("price_ema_1w"),
            price_ema_8d: import!("price_ema_8d"),
            price_ema_12d: import!("price_ema_12d"),
            price_ema_13d: import!("price_ema_13d"),
            price_ema_21d: import!("price_ema_21d"),
            price_ema_26d: import!("price_ema_26d"),
            price_ema_1m: import!("price_ema_1m"),
            price_ema_34d: import!("price_ema_34d"),
            price_ema_55d: import!("price_ema_55d"),
            price_ema_89d: import!("price_ema_89d"),
            price_ema_144d: import!("price_ema_144d"),
            price_ema_200d: import!("price_ema_200d"),
            price_ema_1y: import!("price_ema_1y"),
            price_ema_2y: import!("price_ema_2y"),
            price_ema_200w: import!("price_ema_200w"),
            price_ema_4y: import!("price_ema_4y"),

            price_sma_200d_x2_4,
            price_sma_200d_x0_8,
            price_sma_350d_x2,
        })
    }
}
