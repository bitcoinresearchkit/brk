use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, ReadableCloneableVec};

use super::super::returns;
use super::Vecs;
use crate::indexes;
use crate::internal::{
    ComputedFromHeight, Days30, Days365, Days7, LazyFromHeight, TimesSqrt,
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        returns: &returns::Vecs,
    ) -> Result<Self> {
        let v2 = Version::TWO;

        let price_volatility_1w = LazyFromHeight::from_computed::<TimesSqrt<Days7>>(
            "price_volatility_1w",
            version + v2,
            returns.price_return_24h_sd_1w.sd.height.read_only_boxed_clone(),
            &returns.price_return_24h_sd_1w.sd,
        );

        let price_volatility_1m = LazyFromHeight::from_computed::<TimesSqrt<Days30>>(
            "price_volatility_1m",
            version + v2,
            returns.price_return_24h_sd_1m.sd.height.read_only_boxed_clone(),
            &returns.price_return_24h_sd_1m.sd,
        );

        let price_volatility_1y = LazyFromHeight::from_computed::<TimesSqrt<Days365>>(
            "price_volatility_1y",
            version + v2,
            returns.price_return_24h_sd_1y.sd.height.read_only_boxed_clone(),
            &returns.price_return_24h_sd_1y.sd,
        );

        let price_sharpe_1w =
            ComputedFromHeight::forced_import(db, "price_sharpe_1w", version + v2, indexes)?;
        let price_sharpe_1m =
            ComputedFromHeight::forced_import(db, "price_sharpe_1m", version + v2, indexes)?;
        let price_sharpe_1y =
            ComputedFromHeight::forced_import(db, "price_sharpe_1y", version + v2, indexes)?;

        let price_sortino_1w =
            ComputedFromHeight::forced_import(db, "price_sortino_1w", version + v2, indexes)?;
        let price_sortino_1m =
            ComputedFromHeight::forced_import(db, "price_sortino_1m", version + v2, indexes)?;
        let price_sortino_1y =
            ComputedFromHeight::forced_import(db, "price_sortino_1y", version + v2, indexes)?;

        Ok(Self {
            price_volatility_1w,
            price_volatility_1m,
            price_volatility_1y,
            price_sharpe_1w,
            price_sharpe_1m,
            price_sharpe_1y,
            price_sortino_1w,
            price_sortino_1m,
            price_sortino_1y,
        })
    }
}
