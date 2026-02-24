use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, ImportableVec, PcoVec, ReadableCloneableVec};

use super::Vecs;
use crate::indexes;
use crate::internal::{ComputedHeightDerivedLast, EagerIndexes};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let version = parent_version + Version::new(11);

        let price = PcoVec::forced_import(db, "price_cents", version)?;

        let open = EagerIndexes::forced_import(db, "price_cents_open", version)?;
        let high = EagerIndexes::forced_import(db, "price_cents_high", version)?;
        let low = EagerIndexes::forced_import(db, "price_cents_low", version)?;

        let close = ComputedHeightDerivedLast::forced_import(
            "price_cents_close",
            price.read_only_boxed_clone(),
            version,
            indexes,
        );

        Ok(Self {
            price,
            open,
            high,
            low,
            close,
        })
    }
}
