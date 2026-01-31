use brk_error::Result;
use brk_types::{DateIndex, Height, OHLCCentsUnsigned, Version};
use vecdb::{Database, IterableCloneableVec, LazyVecFrom1};

use super::Vecs;
use crate::internal::{ComputedHeightAndDateBytes, LazyHeightAndDateOHLC, LazyOHLC};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let ohlc: ComputedHeightAndDateBytes<OHLCCentsUnsigned> =
            ComputedHeightAndDateBytes::forced_import(db, "ohlc_cents", version)?;

        let components = LazyHeightAndDateOHLC {
            height: LazyOHLC {
                open: LazyVecFrom1::init(
                    "price_open_cents",
                    version,
                    ohlc.height.boxed_clone(),
                    |h: Height, iter| iter.get(h).map(|o: OHLCCentsUnsigned| o.open),
                ),
                high: LazyVecFrom1::init(
                    "price_high_cents",
                    version,
                    ohlc.height.boxed_clone(),
                    |h: Height, iter| iter.get(h).map(|o: OHLCCentsUnsigned| o.high),
                ),
                low: LazyVecFrom1::init(
                    "price_low_cents",
                    version,
                    ohlc.height.boxed_clone(),
                    |h: Height, iter| iter.get(h).map(|o: OHLCCentsUnsigned| o.low),
                ),
                close: LazyVecFrom1::init(
                    "price_close_cents",
                    version,
                    ohlc.height.boxed_clone(),
                    |h: Height, iter| iter.get(h).map(|o: OHLCCentsUnsigned| o.close),
                ),
            },
            dateindex: LazyOHLC {
                open: LazyVecFrom1::init(
                    "price_open_cents",
                    version,
                    ohlc.dateindex.boxed_clone(),
                    |di: DateIndex, iter| iter.get(di).map(|o: OHLCCentsUnsigned| o.open),
                ),
                high: LazyVecFrom1::init(
                    "price_high_cents",
                    version,
                    ohlc.dateindex.boxed_clone(),
                    |di: DateIndex, iter| iter.get(di).map(|o: OHLCCentsUnsigned| o.high),
                ),
                low: LazyVecFrom1::init(
                    "price_low_cents",
                    version,
                    ohlc.dateindex.boxed_clone(),
                    |di: DateIndex, iter| iter.get(di).map(|o: OHLCCentsUnsigned| o.low),
                ),
                close: LazyVecFrom1::init(
                    "price_close_cents",
                    version,
                    ohlc.dateindex.boxed_clone(),
                    |di: DateIndex, iter| iter.get(di).map(|o: OHLCCentsUnsigned| o.close),
                ),
            },
        };

        Ok(Self {
            split: components,
            ohlc,
        })
    }
}
