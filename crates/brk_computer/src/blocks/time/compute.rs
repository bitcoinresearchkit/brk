use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Timestamp;
use vecdb::{Exit, TypedVecIterator};

use super::Vecs;
use crate::{ComputeIndexes, indexes};

impl Vecs {
    /// Compute height-to-time fields early, before indexes are computed.
    /// These are needed by indexes::block to compute height_to_dateindex.
    pub fn compute_early(
        &mut self,
        indexer: &Indexer,
        starting_height: brk_types::Height,
        exit: &Exit,
    ) -> Result<()> {
        let mut prev_timestamp_fixed = None;
        self.timestamp_fixed.compute_transform(
            starting_height,
            &indexer.vecs.blocks.timestamp,
            |(h, timestamp, height_to_timestamp_fixed_iter)| {
                if prev_timestamp_fixed.is_none()
                    && let Some(prev_h) = h.decremented()
                {
                    prev_timestamp_fixed.replace(
                        height_to_timestamp_fixed_iter
                            .into_iter()
                            .get_unwrap(prev_h),
                    );
                }
                let timestamp_fixed =
                    prev_timestamp_fixed.map_or(timestamp, |prev_d| prev_d.max(timestamp));
                prev_timestamp_fixed.replace(timestamp_fixed);
                (h, timestamp_fixed)
            },
            exit,
        )?;

        Ok(())
    }

    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.timestamp.compute_all(|vec| {
            vec.compute_transform(
                starting_indexes.dateindex,
                &indexes.dateindex.date,
                |(di, d, ..)| (di, Timestamp::from(d)),
                exit,
            )?;
            Ok(())
        })?;

        Ok(())
    }
}
