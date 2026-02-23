use brk_types::Version;
use vecdb::{ReadableCloneableVec, LazyVecFrom2};

use super::super::returns;
use super::Vecs;
use crate::internal::{
    LazyFromHeightLast, RatioF32, StoredF32TimesSqrt7, StoredF32TimesSqrt30, StoredF32TimesSqrt365,
};

impl Vecs {
    pub(crate) fn forced_import(version: Version, returns: &returns::Vecs) -> Self {
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

        let sharpe_1w = LazyVecFrom2::transformed::<RatioF32>(
            "sharpe_1w",
            version + v2,
            returns.price_returns._1w.height.read_only_boxed_clone(),
            price_1w_volatility.height.read_only_boxed_clone(),
        );

        let sharpe_1m = LazyVecFrom2::transformed::<RatioF32>(
            "sharpe_1m",
            version + v2,
            returns.price_returns._1m.height.read_only_boxed_clone(),
            price_1m_volatility.height.read_only_boxed_clone(),
        );

        let sharpe_1y = LazyVecFrom2::transformed::<RatioF32>(
            "sharpe_1y",
            version + v2,
            returns.price_returns._1y.height.read_only_boxed_clone(),
            price_1y_volatility.height.read_only_boxed_clone(),
        );

        // Sortino ratio = returns / downside volatility
        let sortino_1w = LazyVecFrom2::transformed::<RatioF32>(
            "sortino_1w",
            version + v2,
            returns.price_returns._1w.height.read_only_boxed_clone(),
            returns.downside_1w_sd.sd.height.read_only_boxed_clone(),
        );

        let sortino_1m = LazyVecFrom2::transformed::<RatioF32>(
            "sortino_1m",
            version + v2,
            returns.price_returns._1m.height.read_only_boxed_clone(),
            returns.downside_1m_sd.sd.height.read_only_boxed_clone(),
        );

        let sortino_1y = LazyVecFrom2::transformed::<RatioF32>(
            "sortino_1y",
            version + v2,
            returns.price_returns._1y.height.read_only_boxed_clone(),
            returns.downside_1y_sd.sd.height.read_only_boxed_clone(),
        );

        Self {
            price_1w_volatility,
            price_1m_volatility,
            price_1y_volatility,
            sharpe_1w,
            sharpe_1m,
            sharpe_1y,
            sortino_1w,
            sortino_1m,
            sortino_1y,
        }
    }
}
