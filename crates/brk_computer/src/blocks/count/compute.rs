use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Height, StoredU32, Timestamp};
use vecdb::{EagerVec, Exit, PcoVec, ReadableVec, VecIndex};

use crate::ComputeIndexes;

use super::{super::time, Vecs};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        time: &time::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Block count height + cumulative first (rolling computed after window starts)
        self.block_count.height.compute_range(
            starting_indexes.height,
            &indexer.vecs.blocks.weight,
            |h| (h, StoredU32::from(1_u32)),
            exit,
        )?;
        self.block_count.cumulative.height.compute_cumulative(
            starting_indexes.height,
            &self.block_count.height,
            exit,
        )?;

        // Compute rolling window starts (collect monotonic data once for all windows)
        let monotonic_data: Vec<Timestamp> = time.timestamp_monotonic.collect();
        self.compute_rolling_start_hours(&monotonic_data, time, starting_indexes, exit, 1, |s| {
            &mut s.height_1h_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 1, |s| {
            &mut s.height_24h_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 3, |s| {
            &mut s.height_3d_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 7, |s| {
            &mut s.height_1w_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 8, |s| {
            &mut s.height_8d_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 9, |s| {
            &mut s.height_9d_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 12, |s| {
            &mut s.height_12d_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 13, |s| {
            &mut s.height_13d_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 14, |s| {
            &mut s.height_2w_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 21, |s| {
            &mut s.height_21d_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 26, |s| {
            &mut s.height_26d_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 30, |s| {
            &mut s.height_1m_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 34, |s| {
            &mut s.height_34d_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 55, |s| {
            &mut s.height_55d_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 2 * 30, |s| {
            &mut s.height_2m_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 89, |s| {
            &mut s.height_89d_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 3 * 30, |s| {
            &mut s.height_3m_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 111, |s| {
            &mut s.height_111d_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 144, |s| {
            &mut s.height_144d_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 6 * 30, |s| {
            &mut s.height_6m_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 200, |s| {
            &mut s.height_200d_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 350, |s| {
            &mut s.height_350d_ago
        })?;
        self.compute_rolling_start(&monotonic_data, time, starting_indexes, exit, 365, |s| {
            &mut s.height_1y_ago
        })?;
        self.compute_rolling_start(
            &monotonic_data,
            time,
            starting_indexes,
            exit,
            2 * 365,
            |s| &mut s.height_2y_ago,
        )?;
        self.compute_rolling_start(
            &monotonic_data,
            time,
            starting_indexes,
            exit,
            200 * 7,
            |s| &mut s.height_200w_ago,
        )?;
        self.compute_rolling_start(
            &monotonic_data,
            time,
            starting_indexes,
            exit,
            3 * 365,
            |s| &mut s.height_3y_ago,
        )?;
        self.compute_rolling_start(
            &monotonic_data,
            time,
            starting_indexes,
            exit,
            4 * 365,
            |s| &mut s.height_4y_ago,
        )?;
        self.compute_rolling_start(
            &monotonic_data,
            time,
            starting_indexes,
            exit,
            5 * 365,
            |s| &mut s.height_5y_ago,
        )?;
        self.compute_rolling_start(
            &monotonic_data,
            time,
            starting_indexes,
            exit,
            6 * 365,
            |s| &mut s.height_6y_ago,
        )?;
        self.compute_rolling_start(
            &monotonic_data,
            time,
            starting_indexes,
            exit,
            8 * 365,
            |s| &mut s.height_8y_ago,
        )?;
        self.compute_rolling_start(
            &monotonic_data,
            time,
            starting_indexes,
            exit,
            10 * 365,
            |s| &mut s.height_10y_ago,
        )?;

        // Compute rolling window block counts (both block_count's own rolling + separate block_count_sum)
        let ws = crate::internal::WindowStarts {
            _24h: &self.height_24h_ago,
            _7d: &self.height_1w_ago,
            _30d: &self.height_1m_ago,
            _1y: &self.height_1y_ago,
        };
        self.block_count.rolling.compute_rolling_sum(
            starting_indexes.height,
            &ws,
            &self.block_count.height,
            exit,
        )?;
        self.block_count_sum.compute_rolling_sum(
            starting_indexes.height,
            &ws,
            &self.block_count.height,
            exit,
        )?;

        Ok(())
    }

    fn compute_rolling_start<F>(
        &mut self,
        monotonic_data: &[Timestamp],
        time: &time::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        days: usize,
        get_field: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut Self) -> &mut EagerVec<PcoVec<Height, Height>>,
    {
        let mut prev = Height::ZERO;
        Ok(get_field(self).compute_transform(
            starting_indexes.height,
            &time.timestamp_monotonic,
            |(h, t, ..)| {
                while t.difference_in_days_between(monotonic_data[prev.to_usize()]) >= days {
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

    fn compute_rolling_start_hours<F>(
        &mut self,
        monotonic_data: &[Timestamp],
        time: &time::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        hours: usize,
        get_field: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut Self) -> &mut EagerVec<PcoVec<Height, Height>>,
    {
        let mut prev = Height::ZERO;
        Ok(get_field(self).compute_transform(
            starting_indexes.height,
            &time.timestamp_monotonic,
            |(h, t, ..)| {
                while t.difference_in_hours_between(monotonic_data[prev.to_usize()]) >= hours {
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
}
