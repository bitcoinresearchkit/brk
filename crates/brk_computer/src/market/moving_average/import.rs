use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, IterableCloneableVec};

use super::Vecs;
use crate::{
    grouped::{ComputedRatioVecsFromDateIndex, DollarsTimesTenths, LazyVecsFromDateIndex},
    indexes, price,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let v0 = Version::ZERO;

        let indexes_to_price_1w_sma = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_1w_sma",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_8d_sma = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_8d_sma",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_13d_sma = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_13d_sma",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_21d_sma = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_21d_sma",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_1m_sma = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_1m_sma",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_34d_sma = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_34d_sma",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_55d_sma = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_55d_sma",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_89d_sma = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_89d_sma",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_144d_sma = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_144d_sma",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_200d_sma = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_200d_sma",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_1y_sma = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_1y_sma",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_2y_sma = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_2y_sma",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_200w_sma = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_200w_sma",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_4y_sma = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_4y_sma",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;

        let indexes_to_price_1w_ema = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_1w_ema",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_8d_ema = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_8d_ema",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_13d_ema = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_13d_ema",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_21d_ema = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_21d_ema",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_1m_ema = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_1m_ema",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_34d_ema = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_34d_ema",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_55d_ema = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_55d_ema",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_89d_ema = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_89d_ema",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_144d_ema = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_144d_ema",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_200d_ema = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_200d_ema",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_1y_ema = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_1y_ema",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_2y_ema = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_2y_ema",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_200w_ema = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_200w_ema",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;
        let indexes_to_price_4y_ema = ComputedRatioVecsFromDateIndex::forced_import(
            db,
            "price_4y_ema",
            None,
            version + v0,
            indexes,
            true,
            price,
        )?;

        let price_200d_sma_source = indexes_to_price_200d_sma.price.as_ref().unwrap();
        let indexes_to_price_200d_sma_x2_4 =
            LazyVecsFromDateIndex::from_computed::<DollarsTimesTenths<24>>(
                "price_200d_sma_x2_4",
                version + v0,
                price_200d_sma_source
                    .dateindex
                    .as_ref()
                    .map(|v| v.boxed_clone()),
                price_200d_sma_source,
            );
        let indexes_to_price_200d_sma_x0_8 =
            LazyVecsFromDateIndex::from_computed::<DollarsTimesTenths<8>>(
                "price_200d_sma_x0_8",
                version + v0,
                price_200d_sma_source
                    .dateindex
                    .as_ref()
                    .map(|v| v.boxed_clone()),
                price_200d_sma_source,
            );

        Ok(Self {
            indexes_to_price_1w_sma,
            indexes_to_price_8d_sma,
            indexes_to_price_13d_sma,
            indexes_to_price_21d_sma,
            indexes_to_price_1m_sma,
            indexes_to_price_34d_sma,
            indexes_to_price_55d_sma,
            indexes_to_price_89d_sma,
            indexes_to_price_144d_sma,
            indexes_to_price_200d_sma,
            indexes_to_price_1y_sma,
            indexes_to_price_2y_sma,
            indexes_to_price_200w_sma,
            indexes_to_price_4y_sma,

            indexes_to_price_1w_ema,
            indexes_to_price_8d_ema,
            indexes_to_price_13d_ema,
            indexes_to_price_21d_ema,
            indexes_to_price_1m_ema,
            indexes_to_price_34d_ema,
            indexes_to_price_55d_ema,
            indexes_to_price_89d_ema,
            indexes_to_price_144d_ema,
            indexes_to_price_200d_ema,
            indexes_to_price_1y_ema,
            indexes_to_price_2y_ema,
            indexes_to_price_200w_ema,
            indexes_to_price_4y_ema,

            indexes_to_price_200d_sma_x2_4,
            indexes_to_price_200d_sma_x0_8,
        })
    }
}
