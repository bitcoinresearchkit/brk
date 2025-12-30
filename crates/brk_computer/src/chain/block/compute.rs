use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{CheckedSub, Height, StoredU32, StoredU64, Timestamp};
use vecdb::{Exit, TypedVecIterator};

use super::Vecs;
use crate::{ComputeIndexes, indexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let mut height_to_timestamp_fixed_iter =
            indexes.block.height_to_timestamp_fixed.into_iter();
        let mut prev = Height::ZERO;
        self.height_to_24h_block_count.compute_transform(
            starting_indexes.height,
            &indexes.block.height_to_timestamp_fixed,
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

        let mut height_to_timestamp_iter = indexer.vecs.block.height_to_timestamp.iter()?;
        self.height_to_interval.compute_transform(
            starting_indexes.height,
            &indexer.vecs.block.height_to_timestamp,
            |(height, timestamp, ..)| {
                let interval = height.decremented().map_or(Timestamp::ZERO, |prev_h| {
                    let prev_timestamp = height_to_timestamp_iter.get_unwrap(prev_h);
                    timestamp
                        .checked_sub(prev_timestamp)
                        .unwrap_or(Timestamp::ZERO)
                });
                (height, interval)
            },
            exit,
        )?;

        self.indexes_to_block_interval.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_interval),
        )?;

        self.indexes_to_block_weight.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&indexer.vecs.block.height_to_weight),
        )?;

        self.indexes_to_block_size.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&indexer.vecs.block.height_to_total_size),
        )?;

        self.height_to_vbytes.compute_transform(
            starting_indexes.height,
            &indexer.vecs.block.height_to_weight,
            |(h, w, ..)| {
                (
                    h,
                    StoredU64::from(bitcoin::Weight::from(w).to_vbytes_floor()),
                )
            },
            exit,
        )?;

        self.indexes_to_block_vbytes.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_vbytes),
        )?;

        // Timestamp metrics (moved from epoch)
        self.timeindexes_to_timestamp
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &indexes.time.dateindex_to_date,
                    |(di, d, ..)| (di, Timestamp::from(d)),
                    exit,
                )?;
                Ok(())
            })?;

        let mut height_to_timestamp_iter = indexer.vecs.block.height_to_timestamp.iter()?;

        self.difficultyepoch_to_timestamp.compute_transform(
            starting_indexes.difficultyepoch,
            &indexes.block.difficultyepoch_to_first_height,
            |(i, h, ..)| (i, height_to_timestamp_iter.get_unwrap(h)),
            exit,
        )?;

        Ok(())
    }
}
