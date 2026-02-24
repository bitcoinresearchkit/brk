use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{CheckedSub, Timestamp};
use vecdb::{Exit, ReadableVec};

use super::Vecs;
use crate::{blocks, ComputeIndexes};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        count_vecs: &blocks::CountVecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let mut prev_timestamp = None;
        self.interval.height.compute_transform(
            starting_indexes.height,
            &indexer.vecs.blocks.timestamp,
            |(h, timestamp, ..)| {
                let interval = if let Some(prev_h) = h.decremented() {
                    let prev = prev_timestamp.unwrap_or_else(|| {
                        indexer.vecs.blocks.timestamp.collect_one(prev_h).unwrap()
                    });
                    timestamp.checked_sub(prev).unwrap_or(Timestamp::ZERO)
                } else {
                    Timestamp::ZERO
                };
                prev_timestamp = Some(timestamp);
                (h, interval)
            },
            exit,
        )?;

        let window_starts = count_vecs.window_starts();
        self.interval_rolling.compute_distribution(
            starting_indexes.height,
            &window_starts,
            &self.interval.height,
            exit,
        )?;

        Ok(())
    }
}
