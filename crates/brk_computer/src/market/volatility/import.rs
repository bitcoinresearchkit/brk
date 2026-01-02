use brk_types::Version;
use vecdb::{IterableCloneableVec, LazyVecFrom2};

use super::super::returns;
use super::Vecs;
use crate::internal::{
    LazyVecsFromDateIndex, RatioF32, StoredF32TimesSqrt30, StoredF32TimesSqrt365, StoredF32TimesSqrt7,
};

impl Vecs {
    pub fn forced_import(version: Version, returns: &returns::Vecs) -> Self {
        let v2 = Version::TWO;

        let indexes_to_price_1w_volatility =
            LazyVecsFromDateIndex::from_computed::<StoredF32TimesSqrt7>(
                "price_1w_volatility",
                version + v2,
                returns
                    .indexes_to_1d_returns_1w_sd
                    .sd
                    .dateindex
                    .as_ref()
                    .map(|v| v.boxed_clone()),
                &returns.indexes_to_1d_returns_1w_sd.sd,
            );

        let indexes_to_price_1m_volatility =
            LazyVecsFromDateIndex::from_computed::<StoredF32TimesSqrt30>(
                "price_1m_volatility",
                version + v2,
                returns
                    .indexes_to_1d_returns_1m_sd
                    .sd
                    .dateindex
                    .as_ref()
                    .map(|v| v.boxed_clone()),
                &returns.indexes_to_1d_returns_1m_sd.sd,
            );

        let indexes_to_price_1y_volatility =
            LazyVecsFromDateIndex::from_computed::<StoredF32TimesSqrt365>(
                "price_1y_volatility",
                version + v2,
                returns
                    .indexes_to_1d_returns_1y_sd
                    .sd
                    .dateindex
                    .as_ref()
                    .map(|v| v.boxed_clone()),
                &returns.indexes_to_1d_returns_1y_sd.sd,
            );

        let dateindex_to_sharpe_1w = returns
            ._1w_price_returns
            .dateindex
            .as_ref()
            .zip(indexes_to_price_1w_volatility.dateindex.as_ref())
            .map(|(ret, vol)| {
                LazyVecFrom2::transformed::<RatioF32>(
                    "sharpe_1w",
                    version + v2,
                    ret.boxed_clone(),
                    vol.boxed_clone(),
                )
            });

        let dateindex_to_sharpe_1m = returns
            ._1m_price_returns
            .dateindex
            .as_ref()
            .zip(indexes_to_price_1m_volatility.dateindex.as_ref())
            .map(|(ret, vol)| {
                LazyVecFrom2::transformed::<RatioF32>(
                    "sharpe_1m",
                    version + v2,
                    ret.boxed_clone(),
                    vol.boxed_clone(),
                )
            });

        let dateindex_to_sharpe_1y = returns
            ._1y_price_returns
            .dateindex
            .as_ref()
            .zip(indexes_to_price_1y_volatility.dateindex.as_ref())
            .map(|(ret, vol)| {
                LazyVecFrom2::transformed::<RatioF32>(
                    "sharpe_1y",
                    version + v2,
                    ret.boxed_clone(),
                    vol.boxed_clone(),
                )
            });

        // Sortino ratio = returns / downside volatility
        let dateindex_to_sortino_1w = returns
            ._1w_price_returns
            .dateindex
            .as_ref()
            .zip(returns.indexes_to_downside_1w_sd.sd.dateindex.as_ref())
            .map(|(ret, downside_sd)| {
                LazyVecFrom2::transformed::<RatioF32>(
                    "sortino_1w",
                    version + v2,
                    ret.boxed_clone(),
                    downside_sd.boxed_clone(),
                )
            });

        let dateindex_to_sortino_1m = returns
            ._1m_price_returns
            .dateindex
            .as_ref()
            .zip(returns.indexes_to_downside_1m_sd.sd.dateindex.as_ref())
            .map(|(ret, downside_sd)| {
                LazyVecFrom2::transformed::<RatioF32>(
                    "sortino_1m",
                    version + v2,
                    ret.boxed_clone(),
                    downside_sd.boxed_clone(),
                )
            });

        let dateindex_to_sortino_1y = returns
            ._1y_price_returns
            .dateindex
            .as_ref()
            .zip(returns.indexes_to_downside_1y_sd.sd.dateindex.as_ref())
            .map(|(ret, downside_sd)| {
                LazyVecFrom2::transformed::<RatioF32>(
                    "sortino_1y",
                    version + v2,
                    ret.boxed_clone(),
                    downside_sd.boxed_clone(),
                )
            });

        Self {
            indexes_to_price_1w_volatility,
            indexes_to_price_1m_volatility,
            indexes_to_price_1y_volatility,
            dateindex_to_sharpe_1w,
            dateindex_to_sharpe_1m,
            dateindex_to_sharpe_1y,
            dateindex_to_sortino_1w,
            dateindex_to_sortino_1m,
            dateindex_to_sortino_1y,
        }
    }
}
