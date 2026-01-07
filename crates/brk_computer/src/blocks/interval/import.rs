use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{CheckedSub, Height, Timestamp, Version};
use vecdb::{Database, IterableCloneableVec, LazyVecFrom1};

use super::Vecs;
use crate::{indexes, internal::DerivedComputedBlockDistribution};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let height_to_interval = LazyVecFrom1::init(
            "interval",
            version,
            indexer.vecs.block.height_to_timestamp.boxed_clone(),
            |height: Height, timestamp_iter| {
                let timestamp = timestamp_iter.get(height)?;
                let interval = height.decremented().map_or(Timestamp::ZERO, |prev_h| {
                    timestamp_iter
                        .get(prev_h)
                        .map_or(Timestamp::ZERO, |prev_t| {
                            timestamp.checked_sub(prev_t).unwrap_or(Timestamp::ZERO)
                        })
                });
                Some(interval)
            },
        );

        let indexes_to_block_interval = DerivedComputedBlockDistribution::forced_import(
            db,
            "block_interval",
            height_to_interval.boxed_clone(),
            version,
            indexes,
        )?;

        Ok(Self {
            height_to_interval,
            indexes_to_block_interval,
        })
    }
}
