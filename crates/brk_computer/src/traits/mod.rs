use brk_error::Result;
use brk_types::StoredF32;
use vecdb::{
    AnyStoredVec, AnyVec, EagerVec, Exit, PcoVec, PcoVecValue, ReadableVec, VecIndex, VecValue,
    WritableVec,
};

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
        self.validate_computed_version_or_reset(window_starts.version() + values.version())?;
        self.truncate_if_needed(max_from)?;

        self.repeat_until_complete(exit, |this| {
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
                    if *back >= value {
                        deque.pop_back();
                    } else {
                        break;
                    }
                }
                deque.push_back((i, value));

                if i >= skip {
                    let min_val = deque.front().unwrap().1.clone();
                    this.checked_push_at(i, T::from(min_val))?;
                    if this.batch_limit_reached() {
                        break;
                    }
                }
            }

            Ok(())
        })?;

        Ok(())
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
        self.validate_computed_version_or_reset(window_starts.version() + values.version())?;
        self.truncate_if_needed(max_from)?;

        self.repeat_until_complete(exit, |this| {
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
                    if *back <= value {
                        deque.pop_back();
                    } else {
                        break;
                    }
                }
                deque.push_back((i, value));

                if i >= skip {
                    let max_val = deque.front().unwrap().1.clone();
                    this.checked_push_at(i, T::from(max_val))?;
                    if this.batch_limit_reached() {
                        break;
                    }
                }
            }

            Ok(())
        })?;

        Ok(())
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
        self.validate_computed_version_or_reset(window_starts.version() + values.version())?;

        self.truncate_if_needed(max_from)?;

        self.repeat_until_complete(exit, |this| {
            let skip = this.len();
            let end = window_starts.len().min(values.len());

            // Only collect the range needed: from window start of previous
            // element to end.  For incremental (1 block) this is ~window_size
            // instead of the full history.
            let range_start = if skip > 0 {
                window_starts.collect_one_at(skip - 1).unwrap().to_usize()
            } else {
                0
            };
            let partial_values: Vec<A> = values.collect_range_at(range_start, end);

            let mut sorted: Vec<f64> = Vec::new();
            let mut prev_start_usize: usize = range_start;

            // Reconstruct state from historical data
            if skip > 0 {
                (range_start..skip).for_each(|idx| {
                    let v = f64::from(partial_values[idx - range_start]);
                    let pos = sorted
                        .binary_search_by(|a| {
                            a.partial_cmp(&v).unwrap_or(std::cmp::Ordering::Equal)
                        })
                        .unwrap_or_else(|x| x);
                    sorted.insert(pos, v);
                });
            }

            let starts_batch = window_starts.collect_range_at(skip, end);

            for (j, start) in starts_batch.into_iter().enumerate() {
                let i = skip + j;
                let v = f64::from(partial_values[i - range_start]);
                let pos = sorted
                    .binary_search_by(|a| a.partial_cmp(&v).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or_else(|x| x);
                sorted.insert(pos, v);

                let start_usize = start.to_usize();
                while prev_start_usize < start_usize {
                    let old = f64::from(partial_values[prev_start_usize - range_start]);
                    if let Ok(pos) = sorted.binary_search_by(|a| {
                        a.partial_cmp(&old).unwrap_or(std::cmp::Ordering::Equal)
                    }) {
                        sorted.remove(pos);
                    }
                    prev_start_usize += 1;
                }

                let median = if sorted.is_empty() {
                    0.0
                } else if sorted.len().is_multiple_of(2) {
                    let mid = sorted.len() / 2;
                    (sorted[mid - 1] + sorted[mid]) / 2.0
                } else {
                    sorted[sorted.len() / 2]
                };

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
///
/// Since the percentile pass already sorts data, min = sorted[0], max = sorted[last],
/// and average = running_sum / count — all extracted at negligible extra cost.
/// This replaces 3 separate passes (avg, min, max) + 1 percentile pass = 4 passes
/// with a single unified pass.
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

    average_out.validate_computed_version_or_reset(version)?;
    min_out.validate_computed_version_or_reset(version)?;
    max_out.validate_computed_version_or_reset(version)?;
    p10_out.validate_computed_version_or_reset(version)?;
    p25_out.validate_computed_version_or_reset(version)?;
    median_out.validate_computed_version_or_reset(version)?;
    p75_out.validate_computed_version_or_reset(version)?;
    p90_out.validate_computed_version_or_reset(version)?;

    average_out.truncate_if_needed(max_from)?;
    min_out.truncate_if_needed(max_from)?;
    max_out.truncate_if_needed(max_from)?;
    p10_out.truncate_if_needed(max_from)?;
    p25_out.truncate_if_needed(max_from)?;
    median_out.truncate_if_needed(max_from)?;
    p75_out.truncate_if_needed(max_from)?;
    p90_out.truncate_if_needed(max_from)?;

    // All 8 vecs should be at the same length; use min to be safe
    let skip = average_out
        .len()
        .min(min_out.len())
        .min(max_out.len())
        .min(p10_out.len())
        .min(p25_out.len())
        .min(median_out.len())
        .min(p75_out.len())
        .min(p90_out.len());

    let end = window_starts.len().min(values.len());
    if skip >= end {
        return Ok(());
    }

    let range_start = if skip > 0 {
        window_starts.collect_one_at(skip - 1).unwrap().to_usize()
    } else {
        0
    };
    let partial_values: Vec<A> = values.collect_range_at(range_start, end);

    let mut sorted: Vec<f64> = Vec::new();
    let mut running_sum: f64 = 0.0;
    let mut prev_start_usize: usize = range_start;

    // Reconstruct sorted state + running sum from historical data
    if skip > 0 {
        for idx in range_start..skip {
            let v = f64::from(partial_values[idx - range_start]);
            running_sum += v;
            let pos = sorted
                .binary_search_by(|a| a.partial_cmp(&v).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or_else(|x| x);
            sorted.insert(pos, v);
        }
    }

    let starts_batch = window_starts.collect_range_at(skip, end);

    for (j, start) in starts_batch.into_iter().enumerate() {
        let i = skip + j;
        let v = f64::from(partial_values[i - range_start]);
        running_sum += v;
        let pos = sorted
            .binary_search_by(|a| a.partial_cmp(&v).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or_else(|x| x);
        sorted.insert(pos, v);

        let start_usize = start.to_usize();
        while prev_start_usize < start_usize {
            let old = f64::from(partial_values[prev_start_usize - range_start]);
            running_sum -= old;
            if let Ok(pos) = sorted
                .binary_search_by(|a| a.partial_cmp(&old).unwrap_or(std::cmp::Ordering::Equal))
            {
                sorted.remove(pos);
            }
            prev_start_usize += 1;
        }

        let len = sorted.len();
        if len == 0 {
            let zero = T::from(0.0);
            average_out.checked_push_at(i, zero)?;
            min_out.checked_push_at(i, zero)?;
            max_out.checked_push_at(i, zero)?;
            p10_out.checked_push_at(i, zero)?;
            p25_out.checked_push_at(i, zero)?;
            median_out.checked_push_at(i, zero)?;
            p75_out.checked_push_at(i, zero)?;
            p90_out.checked_push_at(i, zero)?;
        } else {
            average_out.checked_push_at(i, T::from(running_sum / len as f64))?;
            min_out.checked_push_at(i, T::from(sorted[0]))?;
            max_out.checked_push_at(i, T::from(sorted[len - 1]))?;
            p10_out.checked_push_at(i, T::from(percentile_of_sorted(&sorted, 0.10)))?;
            p25_out.checked_push_at(i, T::from(percentile_of_sorted(&sorted, 0.25)))?;
            median_out.checked_push_at(i, T::from(percentile_of_sorted(&sorted, 0.50)))?;
            p75_out.checked_push_at(i, T::from(percentile_of_sorted(&sorted, 0.75)))?;
            p90_out.checked_push_at(i, T::from(percentile_of_sorted(&sorted, 0.90)))?;
        }

        if average_out.batch_limit_reached() {
            let _lock = exit.lock();
            average_out.write()?;
            min_out.write()?;
            max_out.write()?;
            p10_out.write()?;
            p25_out.write()?;
            median_out.write()?;
            p75_out.write()?;
            p90_out.write()?;
        }
    }

    // Final flush
    let _lock = exit.lock();
    average_out.write()?;
    min_out.write()?;
    max_out.write()?;
    p10_out.write()?;
    p25_out.write()?;
    median_out.write()?;
    p75_out.write()?;
    p90_out.write()?;

    Ok(())
}

/// Extract a percentile (0.0-1.0) from a sorted slice using linear interpolation.
fn percentile_of_sorted(sorted: &[f64], p: f64) -> f64 {
    let len = sorted.len();
    if len == 1 {
        return sorted[0];
    }
    let rank = p * (len - 1) as f64;
    let lo = rank.floor() as usize;
    let hi = rank.ceil() as usize;
    if lo == hi {
        sorted[lo]
    } else {
        let frac = rank - lo as f64;
        sorted[lo] * (1.0 - frac) + sorted[hi] * frac
    }
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
