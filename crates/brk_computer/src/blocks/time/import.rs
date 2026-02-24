use brk_error::Result;
use brk_types::{Date, Height, Version};
use vecdb::{Database, EagerVec, ImportableVec, LazyVecFrom1, ReadableCloneableVec};

use super::Vecs;
use crate::internal::EagerIndexes;

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
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
            timestamp: EagerIndexes::forced_import(db, "timestamp", version)?,
        })
    }
}
