use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Height, StoredU32};
use vecdb::{EagerVec, Exit, PcoVec, TypedVecIterator};

use super::super::time;
use super::Vecs;
use crate::{ComputeIndexes, indexes, internal::ComputedFromHeightLast};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        time: &time::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.block_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_range(
                    starting_indexes.height,
                    &indexer.vecs.blocks.weight,
                    |h| (h, StoredU32::from(1_u32)),
                    exit,
                )?;
                Ok(())
            })?;

        // Compute rolling window starts
        self.compute_rolling_start(time, starting_indexes, exit, 1, |s| &mut s._24h_start)?;
        self.compute_rolling_start(time, starting_indexes, exit, 7, |s| &mut s._1w_start)?;
        self.compute_rolling_start(time, starting_indexes, exit, 30, |s| &mut s._1m_start)?;
        self.compute_rolling_start(time, starting_indexes, exit, 365, |s| &mut s._1y_start)?;

        // Compute rolling window block counts
        self.compute_rolling_block_count(
            indexes,
            starting_indexes,
            exit,
            &self._24h_start.clone(),
            |s| &mut s._24h_block_count,
        )?;
        self.compute_rolling_block_count(
            indexes,
            starting_indexes,
            exit,
            &self._1w_start.clone(),
            |s| &mut s._1w_block_count,
        )?;
        self.compute_rolling_block_count(
            indexes,
            starting_indexes,
            exit,
            &self._1m_start.clone(),
            |s| &mut s._1m_block_count,
        )?;
        self.compute_rolling_block_count(
            indexes,
            starting_indexes,
            exit,
            &self._1y_start.clone(),
            |s| &mut s._1y_block_count,
        )?;

        Ok(())
    }

    fn compute_rolling_start<F>(
        &mut self,
        time: &time::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        days: usize,
        get_field: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut Self) -> &mut EagerVec<PcoVec<Height, Height>>,
    {
        let mut iter = time.timestamp_fixed.into_iter();
        let mut prev = Height::ZERO;
        Ok(get_field(self).compute_transform(
            starting_indexes.height,
            &time.timestamp_fixed,
            |(h, t, ..)| {
                while t.difference_in_days_between(iter.get_unwrap(prev)) >= days {
                    prev.increment();
                    if prev > h {
                        unreachable!()
                    }
                }
                (h, prev)
            },
            exit,
        )?)
    }

    fn compute_rolling_block_count<F>(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        start_height: &EagerVec<PcoVec<Height, Height>>,
        get_field: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut Self) -> &mut ComputedFromHeightLast<StoredU32>,
    {
        get_field(self).compute_all(indexes, starting_indexes, exit, |v| {
            v.compute_transform(
                starting_indexes.height,
                start_height,
                |(h, start, ..)| (h, StoredU32::from(*h + 1 - *start)),
                exit,
            )?;
            Ok(())
        })
    }
}
