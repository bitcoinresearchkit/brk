use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::{Exit, ReadableVec};

use super::Vecs;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        starting_height: brk_types::Height,
        exit: &Exit,
    ) -> Result<()> {
        let mut prev_timestamp_monotonic = None;
        self.timestamp_monotonic.compute_transform(
            starting_height,
            &indexer.vecs.blocks.timestamp,
            |(h, timestamp, this)| {
                if prev_timestamp_monotonic.is_none()
                    && let Some(prev_h) = h.decremented()
                {
                    prev_timestamp_monotonic.replace(this.collect_one(prev_h).unwrap());
                }
                let timestamp_monotonic =
                    prev_timestamp_monotonic.map_or(timestamp, |prev_d| prev_d.max(timestamp));
                prev_timestamp_monotonic.replace(timestamp_monotonic);
                (h, timestamp_monotonic)
            },
            exit,
        )?;

        Ok(())
    }
}
