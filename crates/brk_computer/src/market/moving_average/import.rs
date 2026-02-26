use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeightPriceWithRatioExtended, DollarsTimesTenths, Price},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        macro_rules! import {
            ($name:expr) => {
                ComputedFromHeightPriceWithRatioExtended::forced_import(
                    db,
                    $name,
                    version,
                    indexes,
                )?
            };
        }

        let price_200d_sma = import!("price_200d_sma");
        let price_350d_sma = import!("price_350d_sma");

        let price_200d_sma_source = &price_200d_sma.price.usd;
        let price_200d_sma_x2_4 = Price::from_computed::<DollarsTimesTenths<24>>(
            "price_200d_sma_x2_4",
            version,
            price_200d_sma_source,
        );
        let price_200d_sma_x0_8 = Price::from_computed::<DollarsTimesTenths<8>>(
            "price_200d_sma_x0_8",
            version,
            price_200d_sma_source,
        );

        let price_350d_sma_source = &price_350d_sma.price.usd;
        let price_350d_sma_x2 = Price::from_computed::<DollarsTimesTenths<20>>(
            "price_350d_sma_x2",
            version,
            price_350d_sma_source,
        );

        Ok(Self {
            price_1w_sma: import!("price_1w_sma"),
            price_8d_sma: import!("price_8d_sma"),
            price_13d_sma: import!("price_13d_sma"),
            price_21d_sma: import!("price_21d_sma"),
            price_1m_sma: import!("price_1m_sma"),
            price_34d_sma: import!("price_34d_sma"),
            price_55d_sma: import!("price_55d_sma"),
            price_89d_sma: import!("price_89d_sma"),
            price_111d_sma: import!("price_111d_sma"),
            price_144d_sma: import!("price_144d_sma"),
            price_200d_sma,
            price_350d_sma,
            price_1y_sma: import!("price_1y_sma"),
            price_2y_sma: import!("price_2y_sma"),
            price_200w_sma: import!("price_200w_sma"),
            price_4y_sma: import!("price_4y_sma"),

            price_1w_ema: import!("price_1w_ema"),
            price_8d_ema: import!("price_8d_ema"),
            price_12d_ema: import!("price_12d_ema"),
            price_13d_ema: import!("price_13d_ema"),
            price_21d_ema: import!("price_21d_ema"),
            price_26d_ema: import!("price_26d_ema"),
            price_1m_ema: import!("price_1m_ema"),
            price_34d_ema: import!("price_34d_ema"),
            price_55d_ema: import!("price_55d_ema"),
            price_89d_ema: import!("price_89d_ema"),
            price_144d_ema: import!("price_144d_ema"),
            price_200d_ema: import!("price_200d_ema"),
            price_1y_ema: import!("price_1y_ema"),
            price_2y_ema: import!("price_2y_ema"),
            price_200w_ema: import!("price_200w_ema"),
            price_4y_ema: import!("price_4y_ema"),

            price_200d_sma_x2_4,
            price_200d_sma_x0_8,
            price_350d_sma_x2,
        })
    }
}
