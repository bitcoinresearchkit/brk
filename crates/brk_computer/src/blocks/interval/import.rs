use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{CheckedSub, Height, Timestamp, Version};
use vecdb::{Database, VecIndex};

use super::Vecs;
use crate::{indexes, internal::LazyBlockDistribution};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let interval = LazyBlockDistribution::forced_import_with_init(
            db,
            "block_interval",
            version,
            indexer.vecs.blocks.timestamp.clone(),
            indexes,
            |height: Height, timestamp_iter| {
                let timestamp = timestamp_iter.get_at(height.to_usize())?;
                let interval = height.decremented().map_or(Timestamp::ZERO, |prev_h| {
                    timestamp_iter
                        .get_at(prev_h.to_usize())
                        .map_or(Timestamp::ZERO, |prev_t| {
                            timestamp.checked_sub(prev_t).unwrap_or(Timestamp::ZERO)
                        })
                });
                Some(interval)
            },
        )?;

        Ok(Self { interval })
    }
}
