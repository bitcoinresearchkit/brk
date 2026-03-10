use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::{
    vecs::{EmaVecs, SmaVecs},
    Vecs,
};
use crate::{
    indexes,
    internal::{CentsTimesTenths, Price, PriceWithRatioPerBlock},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        macro_rules! import {
            ($name:expr) => {
                PriceWithRatioPerBlock::forced_import(db, $name, version, indexes)?
            };
        }

        let sma_200d = import!("price_sma_200d");
        let sma_350d = import!("price_sma_350d");

        let price_sma_200d_source = &sma_200d.price.cents;
        let _200d_x2_4 = Price::from_cents_source::<CentsTimesTenths<24>>(
            "price_sma_200d_x2_4",
            version,
            price_sma_200d_source,
        );
        let _200d_x0_8 = Price::from_cents_source::<CentsTimesTenths<8>>(
            "price_sma_200d_x0_8",
            version,
            price_sma_200d_source,
        );

        let price_sma_350d_source = &sma_350d.price.cents;
        let _350d_x2 = Price::from_cents_source::<CentsTimesTenths<20>>(
            "price_sma_350d_x2",
            version,
            price_sma_350d_source,
        );

        let sma = SmaVecs {
            _1w: import!("price_sma_1w"),
            _8d: import!("price_sma_8d"),
            _13d: import!("price_sma_13d"),
            _21d: import!("price_sma_21d"),
            _1m: import!("price_sma_1m"),
            _34d: import!("price_sma_34d"),
            _55d: import!("price_sma_55d"),
            _89d: import!("price_sma_89d"),
            _111d: import!("price_sma_111d"),
            _144d: import!("price_sma_144d"),
            _200d: sma_200d,
            _350d: sma_350d,
            _1y: import!("price_sma_1y"),
            _2y: import!("price_sma_2y"),
            _200w: import!("price_sma_200w"),
            _4y: import!("price_sma_4y"),
            _200d_x2_4,
            _200d_x0_8,
            _350d_x2,
        };

        let ema = EmaVecs {
            _1w: import!("price_ema_1w"),
            _8d: import!("price_ema_8d"),
            _12d: import!("price_ema_12d"),
            _13d: import!("price_ema_13d"),
            _21d: import!("price_ema_21d"),
            _26d: import!("price_ema_26d"),
            _1m: import!("price_ema_1m"),
            _34d: import!("price_ema_34d"),
            _55d: import!("price_ema_55d"),
            _89d: import!("price_ema_89d"),
            _144d: import!("price_ema_144d"),
            _200d: import!("price_ema_200d"),
            _1y: import!("price_ema_1y"),
            _2y: import!("price_ema_2y"),
            _200w: import!("price_ema_200w"),
            _4y: import!("price_ema_4y"),
        };

        Ok(Self { sma, ema })
    }
}
