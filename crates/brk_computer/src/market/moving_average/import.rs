use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeightRatio, DollarsTimesTenths, LazyPriceFromHeight},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let price_1w_sma = ComputedFromHeightRatio::forced_import(
            db,
            "price_1w_sma",
            None,
            version,
            indexes,
            true,
        )?;
        let price_8d_sma = ComputedFromHeightRatio::forced_import(
            db,
            "price_8d_sma",
            None,
            version,
            indexes,
            true,
        )?;
        let price_13d_sma = ComputedFromHeightRatio::forced_import(
            db,
            "price_13d_sma",
            None,
            version,
            indexes,
            true,
        )?;
        let price_21d_sma = ComputedFromHeightRatio::forced_import(
            db,
            "price_21d_sma",
            None,
            version,
            indexes,
            true,
        )?;
        let price_1m_sma = ComputedFromHeightRatio::forced_import(
            db,
            "price_1m_sma",
            None,
            version,
            indexes,
            true,
        )?;
        let price_34d_sma = ComputedFromHeightRatio::forced_import(
            db,
            "price_34d_sma",
            None,
            version,
            indexes,
            true,
        )?;
        let price_55d_sma = ComputedFromHeightRatio::forced_import(
            db,
            "price_55d_sma",
            None,
            version,
            indexes,
            true,
        )?;
        let price_89d_sma = ComputedFromHeightRatio::forced_import(
            db,
            "price_89d_sma",
            None,
            version,
            indexes,
            true,
        )?;
        let price_111d_sma = ComputedFromHeightRatio::forced_import(
            db,
            "price_111d_sma",
            None,
            version,
            indexes,
            true,
        )?;
        let price_144d_sma = ComputedFromHeightRatio::forced_import(
            db,
            "price_144d_sma",
            None,
            version,
            indexes,
            true,
        )?;
        let price_200d_sma = ComputedFromHeightRatio::forced_import(
            db,
            "price_200d_sma",
            None,
            version,
            indexes,
            true,
        )?;
        let price_350d_sma = ComputedFromHeightRatio::forced_import(
            db,
            "price_350d_sma",
            None,
            version,
            indexes,
            true,
        )?;
        let price_1y_sma = ComputedFromHeightRatio::forced_import(
            db,
            "price_1y_sma",
            None,
            version,
            indexes,
            true,
        )?;
        let price_2y_sma = ComputedFromHeightRatio::forced_import(
            db,
            "price_2y_sma",
            None,
            version,
            indexes,
            true,
        )?;
        let price_200w_sma = ComputedFromHeightRatio::forced_import(
            db,
            "price_200w_sma",
            None,
            version,
            indexes,
            true,
        )?;
        let price_4y_sma = ComputedFromHeightRatio::forced_import(
            db,
            "price_4y_sma",
            None,
            version,
            indexes,
            true,
        )?;

        let price_1w_ema = ComputedFromHeightRatio::forced_import(
            db,
            "price_1w_ema",
            None,
            version,
            indexes,
            true,
        )?;
        let price_8d_ema = ComputedFromHeightRatio::forced_import(
            db,
            "price_8d_ema",
            None,
            version,
            indexes,
            true,
        )?;
        let price_12d_ema = ComputedFromHeightRatio::forced_import(
            db,
            "price_12d_ema",
            None,
            version,
            indexes,
            true,
        )?;
        let price_13d_ema = ComputedFromHeightRatio::forced_import(
            db,
            "price_13d_ema",
            None,
            version,
            indexes,
            true,
        )?;
        let price_21d_ema = ComputedFromHeightRatio::forced_import(
            db,
            "price_21d_ema",
            None,
            version,
            indexes,
            true,
        )?;
        let price_26d_ema = ComputedFromHeightRatio::forced_import(
            db,
            "price_26d_ema",
            None,
            version,
            indexes,
            true,
        )?;
        let price_1m_ema = ComputedFromHeightRatio::forced_import(
            db,
            "price_1m_ema",
            None,
            version,
            indexes,
            true,
        )?;
        let price_34d_ema = ComputedFromHeightRatio::forced_import(
            db,
            "price_34d_ema",
            None,
            version,
            indexes,
            true,
        )?;
        let price_55d_ema = ComputedFromHeightRatio::forced_import(
            db,
            "price_55d_ema",
            None,
            version,
            indexes,
            true,
        )?;
        let price_89d_ema = ComputedFromHeightRatio::forced_import(
            db,
            "price_89d_ema",
            None,
            version,
            indexes,
            true,
        )?;
        let price_144d_ema = ComputedFromHeightRatio::forced_import(
            db,
            "price_144d_ema",
            None,
            version,
            indexes,
            true,
        )?;
        let price_200d_ema = ComputedFromHeightRatio::forced_import(
            db,
            "price_200d_ema",
            None,
            version,
            indexes,
            true,
        )?;
        let price_1y_ema = ComputedFromHeightRatio::forced_import(
            db,
            "price_1y_ema",
            None,
            version,
            indexes,
            true,
        )?;
        let price_2y_ema = ComputedFromHeightRatio::forced_import(
            db,
            "price_2y_ema",
            None,
            version,
            indexes,
            true,
        )?;
        let price_200w_ema = ComputedFromHeightRatio::forced_import(
            db,
            "price_200w_ema",
            None,
            version,
            indexes,
            true,
        )?;
        let price_4y_ema = ComputedFromHeightRatio::forced_import(
            db,
            "price_4y_ema",
            None,
            version,
            indexes,
            true,
        )?;

        let price_200d_sma_source = &price_200d_sma.price.as_ref().unwrap().usd;
        let price_200d_sma_x2_4 = LazyPriceFromHeight::from_computed::<DollarsTimesTenths<24>>(
            "price_200d_sma_x2_4",
            version,
            price_200d_sma_source,
        );
        let price_200d_sma_x0_8 = LazyPriceFromHeight::from_computed::<DollarsTimesTenths<8>>(
            "price_200d_sma_x0_8",
            version,
            price_200d_sma_source,
        );

        let price_350d_sma_source = &price_350d_sma.price.as_ref().unwrap().usd;
        let price_350d_sma_x2 = LazyPriceFromHeight::from_computed::<DollarsTimesTenths<20>>(
            "price_350d_sma_x2",
            version,
            price_350d_sma_source,
        );

        Ok(Self {
            price_1w_sma,
            price_8d_sma,
            price_13d_sma,
            price_21d_sma,
            price_1m_sma,
            price_34d_sma,
            price_55d_sma,
            price_89d_sma,
            price_111d_sma,
            price_144d_sma,
            price_200d_sma,
            price_350d_sma,
            price_1y_sma,
            price_2y_sma,
            price_200w_sma,
            price_4y_sma,

            price_1w_ema,
            price_8d_ema,
            price_12d_ema,
            price_13d_ema,
            price_21d_ema,
            price_26d_ema,
            price_1m_ema,
            price_34d_ema,
            price_55d_ema,
            price_89d_ema,
            price_144d_ema,
            price_200d_ema,
            price_1y_ema,
            price_2y_ema,
            price_200w_ema,
            price_4y_ema,

            price_200d_sma_x2_4,
            price_200d_sma_x0_8,
            price_350d_sma_x2,
        })
    }
}
