use brk_error::Result;
use brk_types::Version;
use vecdb::ReadableCloneableVec;

use super::super::returns;
use super::Vecs;
use crate::internal::{Days30, Days365, Days7, LazyPerBlock, TimesSqrt};

impl Vecs {
    pub(crate) fn forced_import(version: Version, returns: &returns::Vecs) -> Result<Self> {
        let v2 = Version::TWO;

        let _1w = LazyPerBlock::from_computed::<TimesSqrt<Days7>>(
            "price_volatility_1w",
            version + v2,
            returns
                .price_return_24h_sd
                ._1w
                .sd
                .height
                .read_only_boxed_clone(),
            &returns.price_return_24h_sd._1w.sd,
        );

        let _1m = LazyPerBlock::from_computed::<TimesSqrt<Days30>>(
            "price_volatility_1m",
            version + v2,
            returns
                .price_return_24h_sd
                ._1m
                .sd
                .height
                .read_only_boxed_clone(),
            &returns.price_return_24h_sd._1m.sd,
        );

        let _1y = LazyPerBlock::from_computed::<TimesSqrt<Days365>>(
            "price_volatility_1y",
            version + v2,
            returns
                .price_return_24h_sd
                ._1y
                .sd
                .height
                .read_only_boxed_clone(),
            &returns.price_return_24h_sd._1y.sd,
        );

        Ok(Self { _1w, _1m, _1y })
    }
}
