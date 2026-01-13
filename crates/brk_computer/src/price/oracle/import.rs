use brk_error::Result;
use brk_types::Version;
use vecdb::{BytesVec, Database, ImportableVec, PcoVec};

use super::Vecs;

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let height_to_price = PcoVec::forced_import(db, "oracle_height_to_price", version)?;
        let dateindex_to_ohlc = BytesVec::forced_import(db, "oracle_dateindex_to_ohlc", version)?;
        let dateindex_to_tx_count =
            PcoVec::forced_import(db, "oracle_dateindex_to_tx_count", version)?;

        Ok(Self {
            price: height_to_price,
            ohlc: dateindex_to_ohlc,
            tx_count: dateindex_to_tx_count,
        })
    }
}
