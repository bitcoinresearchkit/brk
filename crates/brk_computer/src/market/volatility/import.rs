use brk_types::Version;
use vecdb::{IterableCloneableVec, LazyVecFrom2};

use super::super::returns;
use super::Vecs;
use crate::internal::{
    LazyDateLast, RatioF32, StoredF32TimesSqrt7, StoredF32TimesSqrt30, StoredF32TimesSqrt365,
};

impl Vecs {
    pub fn forced_import(version: Version, returns: &returns::Vecs) -> Self {
        let v2 = Version::TWO;

        let price_1w_volatility = LazyDateLast::from_source::<StoredF32TimesSqrt7>(
            "price_1w_volatility",
            version + v2,
            &returns._1d_returns_1w_sd.sd,
        );

        let price_1m_volatility = LazyDateLast::from_source::<StoredF32TimesSqrt30>(
            "price_1m_volatility",
            version + v2,
            &returns._1d_returns_1m_sd.sd,
        );

        let price_1y_volatility = LazyDateLast::from_source::<StoredF32TimesSqrt365>(
            "price_1y_volatility",
            version + v2,
            &returns._1d_returns_1y_sd.sd,
        );

        let dateindex_to_sharpe_1w = LazyVecFrom2::transformed::<RatioF32>(
            "sharpe_1w",
            version + v2,
            returns.price_returns._1w.dateindex.boxed_clone(),
            price_1w_volatility.dateindex.boxed_clone(),
        );

        let dateindex_to_sharpe_1m = LazyVecFrom2::transformed::<RatioF32>(
            "sharpe_1m",
            version + v2,
            returns.price_returns._1m.dateindex.boxed_clone(),
            price_1m_volatility.dateindex.boxed_clone(),
        );

        let dateindex_to_sharpe_1y = LazyVecFrom2::transformed::<RatioF32>(
            "sharpe_1y",
            version + v2,
            returns.price_returns._1y.dateindex.boxed_clone(),
            price_1y_volatility.dateindex.boxed_clone(),
        );

        // Sortino ratio = returns / downside volatility
        let dateindex_to_sortino_1w = LazyVecFrom2::transformed::<RatioF32>(
            "sortino_1w",
            version + v2,
            returns.price_returns._1w.dateindex.boxed_clone(),
            returns.downside_1w_sd.sd.dateindex.boxed_clone(),
        );

        let dateindex_to_sortino_1m = LazyVecFrom2::transformed::<RatioF32>(
            "sortino_1m",
            version + v2,
            returns.price_returns._1m.dateindex.boxed_clone(),
            returns.downside_1m_sd.sd.dateindex.boxed_clone(),
        );

        let dateindex_to_sortino_1y = LazyVecFrom2::transformed::<RatioF32>(
            "sortino_1y",
            version + v2,
            returns.price_returns._1y.dateindex.boxed_clone(),
            returns.downside_1y_sd.sd.dateindex.boxed_clone(),
        );

        Self {
            price_1w_volatility,
            price_1m_volatility,
            price_1y_volatility,
            sharpe_1w: dateindex_to_sharpe_1w,
            sharpe_1m: dateindex_to_sharpe_1m,
            sharpe_1y: dateindex_to_sharpe_1y,
            sortino_1w: dateindex_to_sortino_1w,
            sortino_1m: dateindex_to_sortino_1m,
            sortino_1y: dateindex_to_sortino_1y,
        }
    }
}
