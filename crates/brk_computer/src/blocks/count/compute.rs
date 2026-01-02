use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Height, StoredU32};
use vecdb::{Exit, TypedVecIterator};

use super::super::time;
use super::Vecs;
use crate::{indexes, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        time: &time::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let mut height_to_timestamp_fixed_iter =
            time.height_to_timestamp_fixed.into_iter();
        let mut prev = Height::ZERO;
        self.height_to_24h_block_count.compute_transform(
            starting_indexes.height,
            &time.height_to_timestamp_fixed,
            |(h, t, ..)| {
                while t.difference_in_days_between(height_to_timestamp_fixed_iter.get_unwrap(prev))
                    > 0
                {
                    prev.increment();
                    if prev > h {
                        unreachable!()
                    }
                }
                (h, StoredU32::from(*h + 1 - *prev))
            },
            exit,
        )?;

        self.indexes_to_block_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_range(
                    starting_indexes.height,
                    &indexer.vecs.block.height_to_weight,
                    |h| (h, StoredU32::from(1_u32)),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1w_block_count
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_block_count.dateindex.unwrap_sum(),
                    7,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1m_block_count
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_block_count.dateindex.unwrap_sum(),
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1y_block_count
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_block_count.dateindex.unwrap_sum(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
