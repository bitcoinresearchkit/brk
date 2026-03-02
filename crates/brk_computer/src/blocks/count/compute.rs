use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Height, StoredU32, Timestamp};
use vecdb::{AnyVec, Cursor, EagerVec, Exit, PcoVec, ReadableVec, VecIndex};

use crate::{internal::WindowStarts, ComputeIndexes};

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

        // Compute rolling window starts
        self.compute_rolling_start_hours(time, starting_indexes, exit, 1, |s| {
            &mut s.height_1h_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 1, |s| {
            &mut s.height_24h_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 3, |s| {
            &mut s.height_3d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 7, |s| {
            &mut s.height_1w_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 8, |s| {
            &mut s.height_8d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 9, |s| {
            &mut s.height_9d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 12, |s| {
            &mut s.height_12d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 13, |s| {
            &mut s.height_13d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 14, |s| {
            &mut s.height_2w_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 21, |s| {
            &mut s.height_21d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 26, |s| {
            &mut s.height_26d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 30, |s| {
            &mut s.height_1m_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 34, |s| {
            &mut s.height_34d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 55, |s| {
            &mut s.height_55d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 60, |s| {
            &mut s.height_2m_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 63, |s| {
            &mut s.height_9w_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 84, |s| {
            &mut s.height_12w_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 89, |s| {
            &mut s.height_89d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 90, |s| {
            &mut s.height_3m_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 98, |s| {
            &mut s.height_14w_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 111, |s| {
            &mut s.height_111d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 144, |s| {
            &mut s.height_144d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 180, |s| {
            &mut s.height_6m_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 182, |s| {
            &mut s.height_26w_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 200, |s| {
            &mut s.height_200d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 270, |s| {
            &mut s.height_9m_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 350, |s| {
            &mut s.height_350d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 360, |s| {
            &mut s.height_12m_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 365, |s| {
            &mut s.height_1y_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 420, |s| {
            &mut s.height_14m_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 730, |s| {
            &mut s.height_2y_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 780, |s| {
            &mut s.height_26m_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 1095, |s| {
            &mut s.height_3y_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 1400, |s| {
            &mut s.height_200w_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 1460, |s| {
            &mut s.height_4y_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 1825, |s| {
            &mut s.height_5y_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 2190, |s| {
            &mut s.height_6y_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 2920, |s| {
            &mut s.height_8y_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 3285, |s| {
            &mut s.height_9y_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 3650, |s| {
            &mut s.height_10y_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 4380, |s| {
            &mut s.height_12y_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 5110, |s| {
            &mut s.height_14y_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 9490, |s| {
            &mut s.height_26y_ago
        })?;

        // Compute rolling window block counts (both block_count's own rolling + separate block_count_sum)
        let ws = WindowStarts {
            _24h: &self.height_24h_ago,
            _7d: &self.height_1w_ago,
            _30d: &self.height_1m_ago,
            _1y: &self.height_1y_ago,
        };
        self.block_count.sum.compute_rolling_sum(
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
        time: &time::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        days: usize,
        get_field: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut Self) -> &mut EagerVec<PcoVec<Height, Height>>,
    {
        self.compute_rolling_start_inner(
            time,
            starting_indexes,
            exit,
            get_field,
            |t, prev_ts| t.difference_in_days_between(prev_ts) >= days,
        )
    }

    fn compute_rolling_start_hours<F>(
        &mut self,
        time: &time::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        hours: usize,
        get_field: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut Self) -> &mut EagerVec<PcoVec<Height, Height>>,
    {
        self.compute_rolling_start_inner(
            time,
            starting_indexes,
            exit,
            get_field,
            |t, prev_ts| t.difference_in_hours_between(prev_ts) >= hours,
        )
    }

    fn compute_rolling_start_inner<F, D>(
        &mut self,
        time: &time::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        get_field: F,
        expired: D,
    ) -> Result<()>
    where
        F: FnOnce(&mut Self) -> &mut EagerVec<PcoVec<Height, Height>>,
        D: Fn(Timestamp, Timestamp) -> bool,
    {
        let field = get_field(self);
        let resume_from = field.len().min(starting_indexes.height.to_usize());
        let mut prev = if resume_from > 0 {
            field.collect_one_at(resume_from - 1).unwrap()
        } else {
            Height::ZERO
        };
        let mut cursor = Cursor::new(&time.timestamp_monotonic);
        cursor.advance(prev.to_usize());
        let mut prev_ts = cursor.next().unwrap();
        Ok(field.compute_transform(
            starting_indexes.height,
            &time.timestamp_monotonic,
            |(h, t, ..)| {
                while expired(t, prev_ts) {
                    prev.increment();
                    prev_ts = cursor.next().unwrap();
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
