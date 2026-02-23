use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Date, Height, Version};
use vecdb::{Database, EagerVec, ImportableVec, ReadableCloneableVec, LazyVecFrom1};

use super::Vecs;
use crate::{indexes, internal::ComputedHeightDerivedFirst};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
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
            timestamp: ComputedHeightDerivedFirst::forced_import(
                "timestamp",
                indexer.vecs.blocks.timestamp.read_only_boxed_clone(),
                version,
                indexes,
            ),
        })
    }
}
