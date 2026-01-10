use brk_error::Result;
use brk_types::{DateIndex, Height, OHLCCents, Version};
use vecdb::{Database, IterableCloneableVec, LazyVecFrom1};

use super::Vecs;
use crate::internal::{ComputedHeightAndDateBytes, LazyHeightAndDateOHLC, LazyOHLC};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let ohlc: ComputedHeightAndDateBytes<OHLCCents> =
            ComputedHeightAndDateBytes::forced_import(db, "ohlc_cents", version)?;

        let components = LazyHeightAndDateOHLC {
            height: LazyOHLC {
                open: LazyVecFrom1::init(
                    "price_open_cents",
                    version,
                    ohlc.height.boxed_clone(),
                    |h: Height, iter| iter.get(h).map(|o: OHLCCents| o.open),
                ),
                high: LazyVecFrom1::init(
                    "price_high_cents",
                    version,
                    ohlc.height.boxed_clone(),
                    |h: Height, iter| iter.get(h).map(|o: OHLCCents| o.high),
                ),
                low: LazyVecFrom1::init(
                    "price_low_cents",
                    version,
                    ohlc.height.boxed_clone(),
                    |h: Height, iter| iter.get(h).map(|o: OHLCCents| o.low),
                ),
                close: LazyVecFrom1::init(
                    "price_close_cents",
                    version,
                    ohlc.height.boxed_clone(),
                    |h: Height, iter| iter.get(h).map(|o: OHLCCents| o.close),
                ),
            },
            dateindex: LazyOHLC {
                open: LazyVecFrom1::init(
                    "price_open_cents",
                    version,
                    ohlc.dateindex.boxed_clone(),
                    |di: DateIndex, iter| iter.get(di).map(|o: OHLCCents| o.open),
                ),
                high: LazyVecFrom1::init(
                    "price_high_cents",
                    version,
                    ohlc.dateindex.boxed_clone(),
                    |di: DateIndex, iter| iter.get(di).map(|o: OHLCCents| o.high),
                ),
                low: LazyVecFrom1::init(
                    "price_low_cents",
                    version,
                    ohlc.dateindex.boxed_clone(),
                    |di: DateIndex, iter| iter.get(di).map(|o: OHLCCents| o.low),
                ),
                close: LazyVecFrom1::init(
                    "price_close_cents",
                    version,
                    ohlc.dateindex.boxed_clone(),
                    |di: DateIndex, iter| iter.get(di).map(|o: OHLCCents| o.close),
                ),
            },
        };

        Ok(Self {
            split: components,
            ohlc,
        })
    }
}
