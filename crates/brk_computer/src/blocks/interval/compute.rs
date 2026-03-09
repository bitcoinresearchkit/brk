use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{CheckedSub, Indexes, Timestamp};
use vecdb::{Exit, ReadableVec};

use super::Vecs;
use crate::blocks;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        lookback: &blocks::LookbackVecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let mut prev_timestamp = None;
        let window_starts = lookback.window_starts();
        self.0
            .compute(starting_indexes.height, &window_starts, exit, |vec| {
                vec.compute_transform(
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
                Ok(())
            })?;

        Ok(())
    }
}
