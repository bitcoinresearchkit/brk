use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, IterableCloneableVec};

use super::Vecs;
use crate::{
    grouped::{
        ComputedStandardDeviationVecsFromDateIndex, LazyVecsFromDateIndex, Source,
        StandardDeviationVecsOptions, StoredF32TimesSqrt7, StoredF32TimesSqrt30,
        StoredF32TimesSqrt365,
    },
    indexes,
};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let v1 = Version::ONE;
        let v2 = Version::TWO;

        let indexes_to_1d_returns_1w_sd =
            ComputedStandardDeviationVecsFromDateIndex::forced_import(
                db,
                "1d_returns_1w_sd",
                7,
                Source::Compute,
                version + v1,
                indexes,
                StandardDeviationVecsOptions::default(),
                None,
            )?;

        let indexes_to_1d_returns_1m_sd =
            ComputedStandardDeviationVecsFromDateIndex::forced_import(
                db,
                "1d_returns_1m_sd",
                30,
                Source::Compute,
                version + v1,
                indexes,
                StandardDeviationVecsOptions::default(),
                None,
            )?;

        let indexes_to_1d_returns_1y_sd =
            ComputedStandardDeviationVecsFromDateIndex::forced_import(
                db,
                "1d_returns_1y_sd",
                365,
                Source::Compute,
                version + v1,
                indexes,
                StandardDeviationVecsOptions::default(),
                None,
            )?;

        let indexes_to_price_1w_volatility =
            LazyVecsFromDateIndex::from_computed::<StoredF32TimesSqrt7>(
                "price_1w_volatility",
                version + v2,
                indexes_to_1d_returns_1w_sd
                    .sd
                    .dateindex
                    .as_ref()
                    .map(|v| v.boxed_clone()),
                &indexes_to_1d_returns_1w_sd.sd,
            );

        let indexes_to_price_1m_volatility =
            LazyVecsFromDateIndex::from_computed::<StoredF32TimesSqrt30>(
                "price_1m_volatility",
                version + v2,
                indexes_to_1d_returns_1m_sd
                    .sd
                    .dateindex
                    .as_ref()
                    .map(|v| v.boxed_clone()),
                &indexes_to_1d_returns_1m_sd.sd,
            );

        let indexes_to_price_1y_volatility =
            LazyVecsFromDateIndex::from_computed::<StoredF32TimesSqrt365>(
                "price_1y_volatility",
                version + v2,
                indexes_to_1d_returns_1y_sd
                    .sd
                    .dateindex
                    .as_ref()
                    .map(|v| v.boxed_clone()),
                &indexes_to_1d_returns_1y_sd.sd,
            );

        Ok(Self {
            indexes_to_1d_returns_1w_sd,
            indexes_to_1d_returns_1m_sd,
            indexes_to_1d_returns_1y_sd,
            indexes_to_price_1w_volatility,
            indexes_to_price_1m_volatility,
            indexes_to_price_1y_volatility,
        })
    }
}
