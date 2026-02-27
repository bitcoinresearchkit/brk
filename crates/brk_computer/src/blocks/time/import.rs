use brk_error::Result;
use brk_types::{Date, Height, Version};
use vecdb::{Database, EagerVec, ImportableVec, LazyVecFrom1, ReadableCloneableVec};

use super::{TimestampIndexes, Vecs};
use crate::indexes;

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let timestamp_monotonic =
            EagerVec::forced_import(db, "timestamp_monotonic", version)?;

        Ok(Self {
            date: LazyVecFrom1::init(
                "date",
                version,
                timestamp_monotonic.read_only_boxed_clone(),
                |_height: Height, timestamp| Date::from(timestamp),
            ),
            timestamp_monotonic,
            timestamp: TimestampIndexes::forced_import(db, version, indexes)?,
        })
    }
}

impl TimestampIndexes {
    fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        macro_rules! period {
            ($field:ident) => {
                LazyVecFrom1::init(
                    "timestamp",
                    version,
                    indexes.$field.first_height.read_only_boxed_clone(),
                    |idx, _: Height| idx.to_timestamp(),
                )
            };
        }

        macro_rules! epoch {
            ($field:ident) => {
                ImportableVec::forced_import(db, "timestamp", version)?
            };
        }

        Ok(Self(crate::indexes_from!(period, epoch)))
    }
}
