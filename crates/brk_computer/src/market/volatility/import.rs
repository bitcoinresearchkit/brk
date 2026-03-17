use brk_error::Result;
use brk_types::Version;
use vecdb::ReadableCloneableVec;

use super::super::returns;
use super::Vecs;
use crate::internal::{Days1, Days7, Days30, Days365, LazyPerBlock, TimesSqrt, Windows};

impl Vecs {
    pub(crate) fn forced_import(version: Version, returns: &returns::Vecs) -> Result<Self> {
        let v2 = Version::TWO;

        let _24h = LazyPerBlock::from_computed::<TimesSqrt<Days1>>(
            "price_volatility_24h",
            version + v2,
            returns.sd_24h._24h.sd.height.read_only_boxed_clone(),
            &returns.sd_24h._24h.sd,
        );

        let _1w = LazyPerBlock::from_computed::<TimesSqrt<Days7>>(
            "price_volatility_1w",
            version + v2,
            returns.sd_24h._1w.sd.height.read_only_boxed_clone(),
            &returns.sd_24h._1w.sd,
        );

        let _1m = LazyPerBlock::from_computed::<TimesSqrt<Days30>>(
            "price_volatility_1m",
            version + v2,
            returns.sd_24h._1m.sd.height.read_only_boxed_clone(),
            &returns.sd_24h._1m.sd,
        );

        let _1y = LazyPerBlock::from_computed::<TimesSqrt<Days365>>(
            "price_volatility_1y",
            version + v2,
            returns.sd_24h._1y.sd.height.read_only_boxed_clone(),
            &returns.sd_24h._1y.sd,
        );

        Ok(Windows { _24h, _1w, _1m, _1y })
    }
}
