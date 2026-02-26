use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, ReadableCloneableVec};

use super::super::returns;
use super::Vecs;
use crate::indexes;
use crate::internal::{
    ComputedFromHeightLast, LazyFromHeightLast, StoredF32TimesSqrt7, StoredF32TimesSqrt30,
    StoredF32TimesSqrt365,
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        returns: &returns::Vecs,
    ) -> Result<Self> {
        let v2 = Version::TWO;

        let price_1w_volatility = LazyFromHeightLast::from_computed::<StoredF32TimesSqrt7>(
            "price_1w_volatility",
            version + v2,
            returns._1d_returns_1w_sd.sd.height.read_only_boxed_clone(),
            &returns._1d_returns_1w_sd.sd,
        );

        let price_1m_volatility = LazyFromHeightLast::from_computed::<StoredF32TimesSqrt30>(
            "price_1m_volatility",
            version + v2,
            returns._1d_returns_1m_sd.sd.height.read_only_boxed_clone(),
            &returns._1d_returns_1m_sd.sd,
        );

        let price_1y_volatility = LazyFromHeightLast::from_computed::<StoredF32TimesSqrt365>(
            "price_1y_volatility",
            version + v2,
            returns._1d_returns_1y_sd.sd.height.read_only_boxed_clone(),
            &returns._1d_returns_1y_sd.sd,
        );

        let sharpe_1w =
            ComputedFromHeightLast::forced_import(db, "sharpe_1w", version + v2, indexes)?;
        let sharpe_1m =
            ComputedFromHeightLast::forced_import(db, "sharpe_1m", version + v2, indexes)?;
        let sharpe_1y =
            ComputedFromHeightLast::forced_import(db, "sharpe_1y", version + v2, indexes)?;

        let sortino_1w =
            ComputedFromHeightLast::forced_import(db, "sortino_1w", version + v2, indexes)?;
        let sortino_1m =
            ComputedFromHeightLast::forced_import(db, "sortino_1m", version + v2, indexes)?;
        let sortino_1y =
            ComputedFromHeightLast::forced_import(db, "sortino_1y", version + v2, indexes)?;

        Ok(Self {
            price_1w_volatility,
            price_1m_volatility,
            price_1y_volatility,
            sharpe_1w,
            sharpe_1m,
            sharpe_1y,
            sortino_1w,
            sortino_1m,
            sortino_1y,
        })
    }
}
