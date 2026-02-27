use brk_error::Result;
use brk_types::StoredF32;
use vecdb::{
    AnyStoredVec, AnyVec, EagerVec, Exit, PcoVec, PcoVecValue, ReadableVec, VecIndex, VecValue,
    WritableVec,
};

use crate::internal::sliding_window::SlidingWindowSorted;

/// Unified rolling extremum (min or max) from window starts.
///
/// `should_replace` determines whether to evict the deque back:
/// - For min: `|back, new| *back >= *new`
/// - For max: `|back, new| *back <= *new`
pub fn compute_rolling_extremum_from_starts<I, T, A>(
    out: &mut EagerVec<PcoVec<I, T>>,
    max_from: I,
    window_starts: &impl ReadableVec<I, I>,
    values: &impl ReadableVec<I, A>,
    should_replace: fn(&A, &A) -> bool,
    exit: &Exit,
) -> Result<()>
where
    I: VecIndex,
    T: PcoVecValue + From<A>,
    A: VecValue + Ord,
{
    out.validate_and_truncate(window_starts.version() + values.version(), max_from)?;

    out.repeat_until_complete(exit, |this| {
        let skip = this.len();
        let mut deque: std::collections::VecDeque<(usize, A)> =
            std::collections::VecDeque::new();

        let start_offset = if skip > 0 {
            window_starts.collect_one_at(skip - 1).unwrap().to_usize()
        } else {
            0
        };

        let end = window_starts.len().min(values.len());
        let starts_batch = window_starts.collect_range_at(start_offset, end);
        let values_batch = values.collect_range_at(start_offset, end);

        for (j, (start, value)) in starts_batch.into_iter().zip(values_batch).enumerate() {
            let i = start_offset + j;
            let start_usize = start.to_usize();
            while let Some(&(idx, _)) = deque.front() {
                if idx < start_usize {
                    deque.pop_front();
                } else {
                    break;
                }
            }
            while let Some((_, back)) = deque.back() {
                if should_replace(back, &value) {
                    deque.pop_back();
                } else {
                    break;
                }
            }
            deque.push_back((i, value));

            if i >= skip {
                let extremum = deque.front().unwrap().1.clone();
                this.checked_push_at(i, T::from(extremum))?;
                if this.batch_limit_reached() {
                    break;
                }
            }
        }

        Ok(())
    })?;

    Ok(())
}

pub trait ComputeRollingMinFromStarts<I: VecIndex, T> {
    fn compute_rolling_min_from_starts<A>(
        &mut self,
        max_from: I,
        window_starts: &impl ReadableVec<I, I>,
        values: &impl ReadableVec<I, A>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue + Ord,
        T: From<A>;
}

impl<I, T> ComputeRollingMinFromStarts<I, T> for EagerVec<PcoVec<I, T>>
where
    I: VecIndex,
    T: PcoVecValue,
{
    fn compute_rolling_min_from_starts<A>(
        &mut self,
        max_from: I,
        window_starts: &impl ReadableVec<I, I>,
        values: &impl ReadableVec<I, A>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue + Ord,
        T: From<A>,
    {
        compute_rolling_extremum_from_starts(
            self,
            max_from,
            window_starts,
            values,
            |back, new| *back >= *new,
            exit,
        )
    }
}

pub trait ComputeRollingMaxFromStarts<I: VecIndex, T> {
    fn compute_rolling_max_from_starts<A>(
        &mut self,
        max_from: I,
        window_starts: &impl ReadableVec<I, I>,
        values: &impl ReadableVec<I, A>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue + Ord,
        T: From<A>;
}

impl<I, T> ComputeRollingMaxFromStarts<I, T> for EagerVec<PcoVec<I, T>>
where
    I: VecIndex,
    T: PcoVecValue,
{
    fn compute_rolling_max_from_starts<A>(
        &mut self,
        max_from: I,
        window_starts: &impl ReadableVec<I, I>,
        values: &impl ReadableVec<I, A>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue + Ord,
        T: From<A>,
    {
        compute_rolling_extremum_from_starts(
            self,
            max_from,
            window_starts,
            values,
            |back, new| *back <= *new,
            exit,
        )
    }
}

pub trait ComputeRollingMedianFromStarts<I: VecIndex, T> {
    fn compute_rolling_median_from_starts<A>(
        &mut self,
        max_from: I,
        window_starts: &impl ReadableVec<I, I>,
        values: &impl ReadableVec<I, A>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue + Copy,
        f64: From<A>;
}

impl<I, T> ComputeRollingMedianFromStarts<I, T> for EagerVec<PcoVec<I, T>>
where
    I: VecIndex,
    T: PcoVecValue + From<f64>,
{
    fn compute_rolling_median_from_starts<A>(
        &mut self,
        max_from: I,
        window_starts: &impl ReadableVec<I, I>,
        values: &impl ReadableVec<I, A>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue + Copy,
        f64: From<A>,
    {
        self.validate_and_truncate(window_starts.version() + values.version(), max_from)?;

        self.repeat_until_complete(exit, |this| {
            let skip = this.len();
            let end = window_starts.len().min(values.len());

            let range_start = if skip > 0 {
                window_starts.collect_one_at(skip - 1).unwrap().to_usize()
            } else {
                0
            };
            let partial_values: Vec<f64> = values
                .collect_range_at(range_start, end)
                .into_iter()
                .map(|a| f64::from(a))
                .collect();

            let capacity = if skip > 0 && skip < end {
                let first_start = window_starts.collect_one_at(skip).unwrap().to_usize();
                (skip + 1).saturating_sub(first_start)
            } else if !partial_values.is_empty() {
                partial_values.len().min(1024)
            } else {
                0
            };

            let mut window = SlidingWindowSorted::with_capacity(capacity);

            if skip > 0 {
                window.reconstruct(&partial_values, range_start, skip);
            }

            let starts_batch = window_starts.collect_range_at(skip, end);

            for (j, start) in starts_batch.into_iter().enumerate() {
                let i = skip + j;
                let v = partial_values[i - range_start];
                let start_usize = start.to_usize();
                window.advance(v, start_usize, &partial_values, range_start);

                let median = window.percentile(0.50);
                this.checked_push_at(i, T::from(median))?;

                if this.batch_limit_reached() {
                    break;
                }
            }

            Ok(())
        })?;

        Ok(())
    }
}

/// Compute all 8 rolling distribution stats (avg, min, max, p10, p25, median, p75, p90)
/// in a single sorted-vec pass per window.
#[allow(clippy::too_many_arguments)]
pub fn compute_rolling_distribution_from_starts<I, T, A>(
    max_from: I,
    window_starts: &impl ReadableVec<I, I>,
    values: &impl ReadableVec<I, A>,
    average_out: &mut EagerVec<PcoVec<I, T>>,
    min_out: &mut EagerVec<PcoVec<I, T>>,
    max_out: &mut EagerVec<PcoVec<I, T>>,
    p10_out: &mut EagerVec<PcoVec<I, T>>,
    p25_out: &mut EagerVec<PcoVec<I, T>>,
    median_out: &mut EagerVec<PcoVec<I, T>>,
    p75_out: &mut EagerVec<PcoVec<I, T>>,
    p90_out: &mut EagerVec<PcoVec<I, T>>,
    exit: &Exit,
) -> Result<()>
where
    I: VecIndex,
    T: PcoVecValue + From<f64>,
    A: VecValue + Copy,
    f64: From<A>,
{
    let version = window_starts.version() + values.version();

    for v in [&mut *average_out, &mut *min_out, &mut *max_out, &mut *p10_out, &mut *p25_out, &mut *median_out, &mut *p75_out, &mut *p90_out] {
        v.validate_and_truncate(version, max_from)?;
    }

    let skip = [average_out.len(), min_out.len(), max_out.len(), p10_out.len(), p25_out.len(), median_out.len(), p75_out.len(), p90_out.len()]
        .into_iter().min().unwrap();

    let end = window_starts.len().min(values.len());
    if skip >= end {
        return Ok(());
    }

    let range_start = if skip > 0 {
        window_starts.collect_one_at(skip - 1).unwrap().to_usize()
    } else {
        0
    };
    let partial_values: Vec<f64> = values
        .collect_range_at(range_start, end)
        .into_iter()
        .map(|a| f64::from(a))
        .collect();

    let capacity = if skip > 0 && skip < end {
        let first_start = window_starts.collect_one_at(skip).unwrap().to_usize();
        (skip + 1).saturating_sub(first_start)
    } else if !partial_values.is_empty() {
        partial_values.len().min(1024)
    } else {
        0
    };

    let mut window = SlidingWindowSorted::with_capacity(capacity);

    if skip > 0 {
        window.reconstruct(&partial_values, range_start, skip);
    }

    let starts_batch = window_starts.collect_range_at(skip, end);

    for (j, start) in starts_batch.into_iter().enumerate() {
        let i = skip + j;
        let v = partial_values[i - range_start];
        let start_usize = start.to_usize();
        window.advance(v, start_usize, &partial_values, range_start);

        if window.is_empty() {
            let zero = T::from(0.0);
            for v in [&mut *average_out, &mut *min_out, &mut *max_out, &mut *p10_out, &mut *p25_out, &mut *median_out, &mut *p75_out, &mut *p90_out] {
                v.checked_push_at(i, zero)?;
            }
        } else {
            average_out.checked_push_at(i, T::from(window.average()))?;
            min_out.checked_push_at(i, T::from(window.min()))?;
            max_out.checked_push_at(i, T::from(window.max()))?;
            p10_out.checked_push_at(i, T::from(window.percentile(0.10)))?;
            p25_out.checked_push_at(i, T::from(window.percentile(0.25)))?;
            median_out.checked_push_at(i, T::from(window.percentile(0.50)))?;
            p75_out.checked_push_at(i, T::from(window.percentile(0.75)))?;
            p90_out.checked_push_at(i, T::from(window.percentile(0.90)))?;
        }

        if average_out.batch_limit_reached() {
            let _lock = exit.lock();
            for v in [&mut *average_out, &mut *min_out, &mut *max_out, &mut *p10_out, &mut *p25_out, &mut *median_out, &mut *p75_out, &mut *p90_out] {
                v.write()?;
            }
        }
    }

    // Final flush
    let _lock = exit.lock();
    for v in [average_out, min_out, max_out, p10_out, p25_out, median_out, p75_out, p90_out] {
        v.write()?;
    }

    Ok(())
}

pub trait ComputeDrawdown<I: VecIndex> {
    fn compute_drawdown<C, A>(
        &mut self,
        max_from: I,
        current: &impl ReadableVec<I, C>,
        ath: &impl ReadableVec<I, A>,
        exit: &Exit,
    ) -> Result<()>
    where
        C: VecValue,
        A: VecValue,
        f64: From<C> + From<A>;
}

impl<I> ComputeDrawdown<I> for EagerVec<PcoVec<I, StoredF32>>
where
    I: VecIndex,
{
    fn compute_drawdown<C, A>(
        &mut self,
        max_from: I,
        current: &impl ReadableVec<I, C>,
        ath: &impl ReadableVec<I, A>,
        exit: &Exit,
    ) -> Result<()>
    where
        C: VecValue,
        A: VecValue,
        f64: From<C> + From<A>,
    {
        self.compute_transform2(
            max_from,
            current,
            ath,
            |(i, current, ath, _)| {
                let ath_f64 = f64::from(ath);
                let drawdown = if ath_f64 == 0.0 {
                    StoredF32::default()
                } else {
                    StoredF32::from((f64::from(current) - ath_f64) / ath_f64 * 100.0)
                };
                (i, drawdown)
            },
            exit,
        )?;
        Ok(())
    }
}
