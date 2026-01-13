use brk_error::Result;
use brk_types::Version;
use vecdb::{BytesVec, Database, ImportableVec, PcoVec};

use super::Vecs;

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let price_cents = PcoVec::forced_import(db, "orange_price_cents", version)?;
        let ohlc_cents = BytesVec::forced_import(db, "oracle_ohlc_cents", version)?;
        let tx_count = PcoVec::forced_import(db, "oracle_tx_count", version)?;

        Ok(Self {
            price_cents,
            ohlc_cents,
            tx_count,
        })
    }
}
