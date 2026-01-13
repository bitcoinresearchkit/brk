use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Date, Height, Version};
use vecdb::{Database, EagerVec, ImportableVec, IterableCloneableVec, LazyVecFrom1, VecIndex};

use super::Vecs;
use crate::{indexes, internal::ComputedHeightDerivedFirst};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let height_to_timestamp_monotonic =
            EagerVec::forced_import(db, "timestamp_monotonic", version)?;

        Ok(Self {
            date: LazyVecFrom1::init(
                "date",
                version,
                indexer.vecs.blocks.timestamp.boxed_clone(),
                |height: Height, timestamp_iter| {
                    timestamp_iter.get_at(height.to_usize()).map(Date::from)
                },
            ),
            date_monotonic: LazyVecFrom1::init(
                "date_monotonic",
                version,
                height_to_timestamp_monotonic.boxed_clone(),
                |height: Height, timestamp_iter| timestamp_iter.get(height).map(Date::from),
            ),
            timestamp_monotonic: height_to_timestamp_monotonic,
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
