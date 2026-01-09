use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Date, Height, Version};
use vecdb::{Database, EagerVec, ImportableVec, IterableCloneableVec, LazyVecFrom1, VecIndex};

use super::Vecs;
use crate::{indexes, internal::DerivedComputedBlockFirst};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let height_to_timestamp_fixed = EagerVec::forced_import(db, "timestamp_fixed", version)?;

        Ok(Self {
            date: LazyVecFrom1::init(
                "date",
                version,
                indexer.vecs.blocks.timestamp.boxed_clone(),
                |height: Height, timestamp_iter| {
                    timestamp_iter.get_at(height.to_usize()).map(Date::from)
                },
            ),
            date_fixed: LazyVecFrom1::init(
                "date_fixed",
                version,
                height_to_timestamp_fixed.boxed_clone(),
                |height: Height, timestamp_iter| timestamp_iter.get(height).map(Date::from),
            ),
            timestamp_fixed: height_to_timestamp_fixed,
            timestamp: DerivedComputedBlockFirst::forced_import(
                db,
                "timestamp",
                indexer.vecs.blocks.timestamp.boxed_clone(),
                version,
                indexes,
            )?,
        })
    }
}
