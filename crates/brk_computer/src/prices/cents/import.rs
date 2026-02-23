use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, ImportableVec, ReadableCloneableVec, PcoVec};

use super::Vecs;
use crate::indexes;
use crate::internal::{ComputedHeightDerivedOHLC, ComputedHeightDerivedSplitOHLC};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let version = parent_version + Version::new(11);

        let price = PcoVec::forced_import(db, "price_cents", version)?;

        let split = ComputedHeightDerivedSplitOHLC::forced_import(
            "price_cents",
            version,
            indexes,
            price.read_only_boxed_clone(),
        );

        let ohlc = ComputedHeightDerivedOHLC::forced_import(
            "price_cents",
            version,
            indexes,
            price.read_only_boxed_clone(),
        );

        Ok(Self { price, split, ohlc })
    }
}
