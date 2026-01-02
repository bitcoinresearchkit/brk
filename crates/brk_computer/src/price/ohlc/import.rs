use brk_error::Result;
use brk_types::Version;
use vecdb::{BytesVec, Database, ImportableVec};

use super::Vecs;

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            dateindex_to_ohlc_in_cents: BytesVec::forced_import(
                db,
                "ohlc_in_cents",
                version,
            )?,
            height_to_ohlc_in_cents: BytesVec::forced_import(
                db,
                "ohlc_in_cents",
                version,
            )?,
        })
    }
}
