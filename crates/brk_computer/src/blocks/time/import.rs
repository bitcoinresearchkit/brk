use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Date, Height, Version};
use vecdb::{Database, EagerVec, ImportableVec, IterableCloneableVec, LazyVecFrom1};

use super::Vecs;
use crate::{indexes, internal::ComputedHeightDerivedFirst};

impl Vecs {
    pub fn forced_import(
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
                timestamp_monotonic.boxed_clone(),
                |height: Height, timestamp_iter| timestamp_iter.get(height).map(Date::from),
            ),
            timestamp_monotonic,
            timestamp: ComputedHeightDerivedFirst::forced_import(
                db,
                "timestamp",
                indexer.vecs.blocks.timestamp.boxed_clone(),
                version,
                indexes,
            )?,
        })
    }
}
