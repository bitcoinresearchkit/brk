use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, ImportableVec, PcoVec, ReadableCloneableVec};

use super::Vecs;
use crate::indexes;
use crate::internal::{ComputedHeightDerivedLast, EagerIndexes};
use crate::prices::{ohlcs::OhlcVecs, split::SplitOhlc};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let version = parent_version + Version::new(11);

        let price = PcoVec::forced_import(db, "price_cents", version)?;

        let open = EagerIndexes::forced_import(db, "price_open_cents", version)?;
        let high = EagerIndexes::forced_import(db, "price_high_cents", version)?;
        let low = EagerIndexes::forced_import(db, "price_low_cents", version)?;

        let close = ComputedHeightDerivedLast::forced_import(
            "price_close_cents",
            price.read_only_boxed_clone(),
            version,
            indexes,
        );

        let split = SplitOhlc {
            open,
            high,
            low,
            close,
        };

        let ohlc = OhlcVecs::forced_import(db, "price_ohlc_cents", version)?;

        Ok(Self { split, ohlc, price })
    }
}
