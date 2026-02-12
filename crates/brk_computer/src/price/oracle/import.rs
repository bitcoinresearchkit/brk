use brk_error::Result;
use brk_types::{DateIndex, OHLCCentsUnsigned, OHLCDollars, Version};
use vecdb::{BytesVec, Database, ImportableVec, IterableCloneableVec, LazyVecFrom1, PcoVec};

use super::Vecs;

impl Vecs {
    pub fn forced_import(db: &Database, parent_version: Version) -> Result<Self> {
        let version = parent_version + Version::new(10);

        let price_cents = PcoVec::forced_import(db, "oracle_price_cents", version)?;
        let ohlc_cents = BytesVec::forced_import(db, "oracle_ohlc_cents", version)?;

        let ohlc_dollars = LazyVecFrom1::init(
            "oracle_ohlc_dollars",
            version,
            ohlc_cents.boxed_clone(),
            |di: DateIndex, iter| {
                iter.get(di)
                    .map(|o: OHLCCentsUnsigned| OHLCDollars::from(o))
            },
        );

        Ok(Self {
            price_cents,
            ohlc_cents,
            ohlc_dollars,
        })
    }
}
