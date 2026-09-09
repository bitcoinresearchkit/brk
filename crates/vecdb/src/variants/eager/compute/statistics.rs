use std::{
    cmp::Ordering,
    collections::VecDeque,
    ops::{Add, AddAssign, Div, Sub, SubAssign},
};

use brk_exit::Exit;

use super::super::EagerVec;
use crate::{
    AnyVec, CheckedSub, Error, ReadableVec, Result, StoredVec, VecIndex, VecValue, Version,
    WritableVec,
};

impl<V> EagerVec<V>
where
    V: StoredVec,
{
    fn compute_monotonic_window<A, F>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        window: usize,
        exit: &Exit,
        should_pop: F,
    ) -> Result<()>
    where
        A: VecValue + Ord,
        V::T: From<A>,
        F: Fn(&A, &A) -> bool,
    {
        #[inline]
        fn update_deque<A>(
            deque: &mut VecDeque<(usize, A)>,
            i: usize,
            value: A,
            window: usize,
            should_pop: &impl Fn(&A, &A) -> bool,
        ) {
            while let Some(&(idx, _)) = deque.front() {
                if i >= window && idx <= i - window {
                    deque.pop_front();
                } else {
                    break;
                }
            }
            while let Some((_, v)) = deque.back() {
                if should_pop(v, &value) {
                    deque.pop_back();
                } else {
                    break;
                }
            }
            deque.push_back((i, value));
        }

        self.compute_init(source.version(), max_from, exit, |this| {
            let skip = this.len();
            let end = this.batch_end(source.len());
            if skip >= end {
                return Ok(());
            }

            let mut deque: VecDeque<(usize, A)> = VecDeque::with_capacity(window.min(1024));

            // Rebuild deque state from source
            let rebuild_start = skip.saturating_sub(window);
            let mut rebuild_i = rebuild_start;
            source.for_each_range_dyn_at(rebuild_start, skip, &mut |value: A| {
                update_deque(&mut deque, rebuild_i, value, window, &should_pop);
                rebuild_i += 1;
            });

            // Process new elements
            let mut i = skip;
            source.try_fold_range_at(skip, end, (), |(), value: A| {
                update_deque(&mut deque, i, value, window, &should_pop);

                let v = deque.front().unwrap().1.clone();
                this.push(V::T::from(v));
                i += 1;
                Ok(())
            })
        })
    }

    pub fn compute_max<A>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        window: usize,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue + Ord,
        V::T: From<A>,
    {
        self.compute_monotonic_window(max_from, source, window, exit, |v, value| v < value)
    }

    pub fn compute_min<A>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        window: usize,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue + Ord,
        V::T: From<A>,
    {
        self.compute_monotonic_window(max_from, source, window, exit, |v, value| v > value)
    }

    pub fn compute_sum<A>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        window: usize,
        exit: &Exit,
    ) -> Result<()>
    where
        V::T: Add<V::T, Output = V::T> + From<A> + Default + CheckedSub,
        A: VecValue,
    {
        // Cursor for the leaving-value reads — persists across batches so each
        // compressed page is decompressed at most once instead of once per element.
        let mut leaving = source.cursor();

        self.compute_init(Version::new(2) + source.version(), max_from, exit, |this| {
            let skip = this.len();
            let end = this.batch_end(source.len());
            if skip >= end {
                return Ok(());
            }

            let mut prev_sum = if skip > 0 {
                this.collect_one_at(skip - 1).unwrap()
            } else {
                V::T::default()
            };

            // Position cursor at the start of the leaving-values window.
            let pop_start = skip.saturating_sub(window);
            if leaving.position() < pop_start {
                leaving.advance(pop_start - leaving.position());
            }

            let mut i = skip;
            source.try_fold_range_at(skip, end, (), |(), value: A| {
                let value = V::T::from(value);

                let sum = if i >= window {
                    let old = V::T::from(leaving.next().unwrap());
                    match prev_sum.clone().checked_sub(old) {
                        Some(diff) => diff + value,
                        None => return Err(Error::Underflow),
                    }
                } else {
                    prev_sum.clone() + value
                };

                prev_sum = sum.clone();
                this.push(sum);
                i += 1;
                Ok(())
            })
        })
    }

    /// Compute rolling sum with variable window starts.
    /// For each index i, computes sum of values from `window_starts[i]` to i (inclusive).
    pub fn compute_rolling_sum<A>(
        &mut self,
        max_from: V::I,
        window_starts: &impl ReadableVec<V::I, V::I>,
        values: &impl ReadableVec<V::I, A>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue,
        V::T: From<A> + Default + AddAssign + SubAssign,
    {
        // Cursor for the leaving-value reads — persists across batches so each
        // compressed page is decompressed at most once instead of once per element.
        let mut leaving = values.cursor();

        self.compute_init(
            window_starts.version() + values.version(),
            max_from,
            exit,
            |this| {
                let skip = this.len();
                let source_len = window_starts.len().min(values.len());
                let end = this.batch_end(source_len);
                if skip >= end {
                    return Ok(());
                }

                let (mut running_sum, mut prev_start) = if skip > 0 {
                    let prev_idx = skip - 1;
                    let prev_start = window_starts.collect_one_at(prev_idx).unwrap();
                    let sum = this.collect_one_at(prev_idx).unwrap();
                    // Position cursor at the current window start.
                    if leaving.position() < prev_start.to_usize() {
                        leaving.advance(prev_start.to_usize() - leaving.position());
                    }
                    (sum, prev_start)
                } else {
                    (V::T::default(), V::I::from(0))
                };

                let starts_batch = window_starts.collect_range_at(skip, end);
                let values_batch = values.collect_range_at(skip, end);

                for (start, value) in starts_batch.into_iter().zip(values_batch) {
                    running_sum += V::T::from(value);

                    if prev_start < start {
                        let n = start.to_usize() - prev_start.to_usize();
                        leaving.for_each(n, |v: A| {
                            running_sum -= V::T::from(v);
                        });
                        prev_start = start;
                    }

                    this.push(running_sum.clone());
                }

                Ok(())
            },
        )
    }

    /// Compute rolling average with variable window starts.
    /// For each index i, computes mean of values from `window_starts[i]` to i (inclusive).
    pub fn compute_rolling_average<A>(
        &mut self,
        max_from: V::I,
        window_starts: &impl ReadableVec<V::I, V::I>,
        values: &impl ReadableVec<V::I, A>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue,
        f64: From<A> + From<V::T>,
        V::T: From<f64> + Default,
    {
        // Cursor for the leaving-value reads — persists across batches so each
        // compressed page is decompressed at most once instead of once per element.
        let mut leaving = values.cursor();

        self.compute_init(
            window_starts.version() + values.version() + Version::new(2),
            max_from,
            exit,
            |this| {
                let skip = this.len();
                let source_len = window_starts.len().min(values.len());
                let end = this.batch_end(source_len);
                if skip >= end {
                    return Ok(());
                }

                // Recover running_sum from stored average (fast but lossy).
                let (mut running_sum, mut prev_start) = if skip > 0 {
                    let prev_idx = skip - 1;
                    let prev_start = window_starts.collect_one_at(prev_idx).unwrap();
                    if leaving.position() < prev_start.to_usize() {
                        leaving.advance(prev_start.to_usize() - leaving.position());
                    }
                    let stored_avg = f64::from(this.collect_one_at(prev_idx).unwrap());
                    let window_count = prev_idx + 1 - prev_start.to_usize();
                    (stored_avg * window_count as f64, prev_start)
                } else {
                    (0.0_f64, V::I::from(0))
                };

                let starts_batch = window_starts.collect_range_at(skip, end);
                let values_batch = values.collect_range_at(skip, end);

                for (j, (start, value)) in starts_batch.into_iter().zip(values_batch).enumerate() {
                    let i = skip + j;
                    running_sum += f64::from(value);

                    if prev_start < start {
                        let n = start.to_usize() - prev_start.to_usize();
                        leaving.for_each(n, |v: A| {
                            running_sum -= f64::from(v);
                        });
                        prev_start = start;
                    }

                    let count = i - start.to_usize() + 1;
                    let avg = running_sum / count as f64;
                    this.push(V::T::from(avg));
                }

                Ok(())
            },
        )
    }

    /// Compute rolling standard deviation with variable window starts.
    /// For each index `i`, computes SD of values from `window_starts[i]` to `i` (inclusive),
    /// using the provided rolling mean.
    /// `SD = sqrt(E[X²] - E[X]²)` where `E[X²]` is the rolling mean of squares
    /// and `E[X]` is the rolling mean from the `mean` parameter.
    pub fn compute_rolling_sd<A, B>(
        &mut self,
        max_from: V::I,
        window_starts: &impl ReadableVec<V::I, V::I>,
        values: &impl ReadableVec<V::I, A>,
        mean: &impl ReadableVec<V::I, B>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue,
        B: VecValue,
        f64: From<A> + From<B> + From<V::T>,
        V::T: From<f64>,
    {
        let mut leaving = values.cursor();

        self.compute_init(
            window_starts.version() + values.version() + mean.version(),
            max_from,
            exit,
            |this| {
                let skip = this.len();
                let source_len = window_starts.len().min(values.len()).min(mean.len());
                let end = this.batch_end(source_len);
                if skip >= end {
                    return Ok(());
                }

                let (mut running_sum_sq, mut prev_start) = if skip > 0 {
                    let prev_idx = skip - 1;
                    let prev_start = window_starts.collect_one_at(prev_idx).unwrap();
                    let count = (prev_idx + 1 - prev_start.to_usize()) as f64;
                    let sd_val = f64::from(this.collect_one_at(prev_idx).unwrap());
                    let mean_val = f64::from(mean.collect_one_at(prev_idx).unwrap());
                    let sum_sq = (sd_val * sd_val + mean_val * mean_val) * count;
                    if leaving.position() < prev_start.to_usize() {
                        leaving.advance(prev_start.to_usize() - leaving.position());
                    }
                    (sum_sq, prev_start)
                } else {
                    (0.0f64, V::I::from(0))
                };

                let starts_batch = window_starts.collect_range_at(skip, end);
                let values_batch = values.collect_range_at(skip, end);
                let mean_batch = mean.collect_range_at(skip, end);

                for (j, ((start, value), m)) in starts_batch
                    .into_iter()
                    .zip(values_batch)
                    .zip(mean_batch)
                    .enumerate()
                {
                    let i = skip + j;
                    let val = f64::from(value);
                    running_sum_sq += val * val;

                    if prev_start < start {
                        let n = start.to_usize() - prev_start.to_usize();
                        leaving.for_each(n, |v: A| {
                            let old = f64::from(v);
                            running_sum_sq -= old * old;
                        });
                        prev_start = start;
                    }

                    let count = (i - start.to_usize() + 1) as f64;
                    let mean_val = f64::from(m);
                    let variance = (running_sum_sq / count - mean_val * mean_val).max(0.0);
                    this.push(V::T::from(variance.sqrt()));
                }

                Ok(())
            },
        )
    }

    /// Compute expanding (all-time) standard deviation.
    /// For each index `i`, computes SD of all values from 0 to `i` (inclusive).
    /// `SD = sqrt(E[X²] - E[X]²)`.
    pub fn compute_expanding_sd<A, B>(
        &mut self,
        max_from: V::I,
        values: &impl ReadableVec<V::I, A>,
        mean: &impl ReadableVec<V::I, B>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue,
        B: VecValue,
        f64: From<A> + From<B> + From<V::T>,
        V::T: From<f64>,
    {
        self.compute_init(values.version() + mean.version(), max_from, exit, |this| {
            let skip = this.len();
            let source_len = values.len().min(mean.len());
            let end = this.batch_end(source_len);
            if skip >= end {
                return Ok(());
            }

            let mut running_sum_sq = if skip > 0 {
                let count = skip as f64;
                let sd_val = f64::from(this.collect_one_at(skip - 1).unwrap());
                let mean_val = f64::from(mean.collect_one_at(skip - 1).unwrap());
                (sd_val * sd_val + mean_val * mean_val) * count
            } else {
                0.0f64
            };

            let values_batch = values.collect_range_at(skip, end);
            let mean_batch = mean.collect_range_at(skip, end);

            for (j, (value, m)) in values_batch.into_iter().zip(mean_batch).enumerate() {
                let i = skip + j;
                let val = f64::from(value);
                running_sum_sq += val * val;

                let count = (i + 1) as f64;
                let mean_val = f64::from(m);
                let variance = (running_sum_sq / count - mean_val * mean_val).max(0.0);
                this.push(V::T::from(variance.sqrt()));
            }

            Ok(())
        })
    }

    /// Compute rolling EMA with variable window starts.
    /// For each index `i`, computes an exponential moving average with
    /// `α = 2/(span+1)` where `span = i - window_starts[i] + 1`.
    pub fn compute_rolling_ema<A>(
        &mut self,
        max_from: V::I,
        window_starts: &impl ReadableVec<V::I, V::I>,
        values: &impl ReadableVec<V::I, A>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue,
        f64: From<A> + From<V::T>,
        V::T: From<f64> + Default,
    {
        self.compute_rolling_exponential(max_from, window_starts, values, exit, |span| {
            2.0 / (span + 1.0)
        })
    }

    /// Compute rolling RMA (Wilder's smoothing) with variable window starts.
    /// `α = 1/span` where `span = i - window_starts[i] + 1`.
    pub fn compute_rolling_rma<A>(
        &mut self,
        max_from: V::I,
        window_starts: &impl ReadableVec<V::I, V::I>,
        values: &impl ReadableVec<V::I, A>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue,
        f64: From<A> + From<V::T>,
        V::T: From<f64> + Default,
    {
        self.compute_rolling_exponential(max_from, window_starts, values, exit, |span| 1.0 / span)
    }

    fn compute_rolling_exponential<A, F>(
        &mut self,
        max_from: V::I,
        window_starts: &impl ReadableVec<V::I, V::I>,
        values: &impl ReadableVec<V::I, A>,
        exit: &Exit,
        alpha_fn: F,
    ) -> Result<()>
    where
        A: VecValue,
        f64: From<A> + From<V::T>,
        V::T: From<f64> + Default,
        F: Fn(f64) -> f64,
    {
        self.compute_init(
            Version::new(2) + window_starts.version() + values.version(),
            max_from,
            exit,
            |this| {
                let skip = this.len();
                let source_len = window_starts.len().min(values.len());
                let end = this.batch_end(source_len);
                if skip >= end {
                    return Ok(());
                }

                let mut prev = if skip > 0 {
                    f64::from(this.collect_one_at(skip - 1).unwrap())
                } else {
                    0.0_f64
                };

                let starts_batch = window_starts.collect_range_at(skip, end);
                let values_batch = values.collect_range_at(skip, end);

                for (j, (start, value)) in starts_batch.into_iter().zip(values_batch).enumerate() {
                    let i = skip + j;
                    let span = (i - start.to_usize() + 1) as f64;
                    let alpha = alpha_fn(span);
                    let value = f64::from(value);
                    prev = alpha * value + (1.0 - alpha) * prev;
                    this.push(V::T::from(prev));
                }

                Ok(())
            },
        )
    }

    /// Compute rolling maximum with variable window starts (deque-based).
    pub fn compute_rolling_max_from_starts<A>(
        &mut self,
        max_from: V::I,
        window_starts: &impl ReadableVec<V::I, V::I>,
        source: &impl ReadableVec<V::I, A>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue + Ord,
        V::T: From<A>,
    {
        self.compute_rolling_monotonic_from_starts(
            max_from,
            window_starts,
            source,
            exit,
            |back, new| *back <= *new,
        )
    }

    /// Compute rolling minimum with variable window starts (deque-based).
    pub fn compute_rolling_min_from_starts<A>(
        &mut self,
        max_from: V::I,
        window_starts: &impl ReadableVec<V::I, V::I>,
        source: &impl ReadableVec<V::I, A>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue + Ord,
        V::T: From<A>,
    {
        self.compute_rolling_monotonic_from_starts(
            max_from,
            window_starts,
            source,
            exit,
            |back, new| *back >= *new,
        )
    }

    fn compute_rolling_monotonic_from_starts<A, F>(
        &mut self,
        max_from: V::I,
        window_starts: &impl ReadableVec<V::I, V::I>,
        source: &impl ReadableVec<V::I, A>,
        exit: &Exit,
        should_pop: F,
    ) -> Result<()>
    where
        A: VecValue + Ord,
        V::T: From<A>,
        F: Fn(&A, &A) -> bool,
    {
        self.compute_init(
            window_starts.version() + source.version(),
            max_from,
            exit,
            |this| {
                let skip = this.len();
                let source_len = window_starts.len().min(source.len());
                let end = this.batch_end(source_len);
                if skip >= end {
                    return Ok(());
                }

                // Rebuild deque from source values in the current window
                let mut deque: VecDeque<(usize, A)> = VecDeque::new();
                if skip > 0 {
                    let window_start = window_starts.collect_one_at(skip - 1).unwrap().to_usize();
                    let window_values = source.collect_range_at(window_start, skip);
                    for (k, v) in (window_start..skip).zip(window_values) {
                        while deque.back().is_some_and(|(_, dv)| should_pop(dv, &v)) {
                            deque.pop_back();
                        }
                        deque.push_back((k, v));
                    }
                }

                let starts_batch = window_starts.collect_range_at(skip, end);
                let values_batch = source.collect_range_at(skip, end);

                for (j, (start, value)) in starts_batch.into_iter().zip(values_batch).enumerate() {
                    let i = skip + j;
                    let ws = start.to_usize();

                    while let Some(&(idx, _)) = deque.front() {
                        if idx < ws {
                            deque.pop_front();
                        } else {
                            break;
                        }
                    }

                    while deque.back().is_some_and(|(_, v)| should_pop(v, &value)) {
                        deque.pop_back();
                    }
                    deque.push_back((i, value));

                    this.push(V::T::from(deque.front().unwrap().1.clone()));
                }

                Ok(())
            },
        )
    }

    /// Compute rolling ratio with variable window starts.
    /// For each index i, computes `sum(numerator[window_starts[i]..=i]) / denominator[i]`.
    pub fn compute_rolling_ratio<A, B>(
        &mut self,
        max_from: V::I,
        window_starts: &impl ReadableVec<V::I, V::I>,
        numerator: &impl ReadableVec<V::I, A>,
        denominator: &impl ReadableVec<V::I, B>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue,
        B: VecValue,
        f64: From<A> + From<B> + From<V::T>,
        V::T: From<f64>,
    {
        // Cursor for the leaving-value reads — persists across batches so each
        // compressed page is decompressed at most once instead of once per element.
        let mut leaving = numerator.cursor();

        self.compute_init(
            window_starts.version() + numerator.version() + denominator.version(),
            max_from,
            exit,
            |this| {
                let skip = this.len();
                let source_len = window_starts
                    .len()
                    .min(numerator.len())
                    .min(denominator.len());
                let end = this.batch_end(source_len);
                if skip >= end {
                    return Ok(());
                }

                // Recover running_sum from stored_ratio * denom[prev_idx].
                // Reads 3 values instead of window_count from numerator.
                // Falls back to full scan when prev denom is 0.
                let (mut running_sum, mut prev_start) = if skip > 0 {
                    let prev_idx = skip - 1;
                    let prev_start = window_starts.collect_one_at(prev_idx).unwrap();
                    let prev_denom = f64::from(denominator.collect_one_at(prev_idx).unwrap());
                    if prev_denom != 0.0 {
                        let stored_ratio = f64::from(this.collect_one_at(prev_idx).unwrap());
                        if leaving.position() < prev_start.to_usize() {
                            leaving.advance(prev_start.to_usize() - leaving.position());
                        }
                        (stored_ratio * prev_denom, prev_start)
                    } else {
                        let mut sum = 0.0_f64;
                        // Full scan fallback — position cursor at window start
                        // so the main loop can subtract leaving values correctly.
                        if leaving.position() < prev_start.to_usize() {
                            leaving.advance(prev_start.to_usize() - leaving.position());
                        }
                        numerator.for_each_range_dyn_at(
                            prev_start.to_usize(),
                            prev_idx + 1,
                            &mut |v: A| {
                                sum += f64::from(v);
                            },
                        );
                        (sum, prev_start)
                    }
                } else {
                    (0.0_f64, V::I::from(0))
                };

                let starts_batch = window_starts.collect_range_at(skip, end);
                let num_batch = numerator.collect_range_at(skip, end);
                let denom_batch = denominator.collect_range_at(skip, end);

                for ((start, num_val), denom_val) in
                    starts_batch.into_iter().zip(num_batch).zip(denom_batch)
                {
                    running_sum += f64::from(num_val);

                    if prev_start < start {
                        let n = start.to_usize() - prev_start.to_usize();
                        leaving.for_each(n, |v: A| {
                            running_sum -= f64::from(v);
                        });
                        prev_start = start;
                    }

                    let denom = f64::from(denom_val);
                    let ratio = if denom != 0.0 {
                        running_sum / denom
                    } else {
                        0.0
                    };
                    this.push(V::T::from(ratio));
                }

                Ok(())
            },
        )
    }

    pub fn compute_sma<A>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        sma: usize,
        exit: &Exit,
    ) -> Result<()>
    where
        V::T: Add<V::T, Output = V::T> + From<A> + From<f32>,
        A: VecValue,
        f32: From<V::T> + From<A>,
    {
        self.compute_sma_(max_from, source, sma, exit, None)
    }

    pub fn compute_sma_<A>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        window: usize,
        exit: &Exit,
        min_i: Option<V::I>,
    ) -> Result<()>
    where
        V::T: Add<V::T, Output = V::T> + From<A> + From<f32>,
        A: VecValue,
        f32: From<V::T> + From<A>,
    {
        self.compute_init(Version::new(2) + source.version(), max_from, exit, |this| {
            let skip = this.len();
            let end = this.batch_end(source.len());
            if skip >= end {
                return Ok(());
            }

            let min_i = min_i.map(|i| i.to_usize());
            let min_prev_i = min_i.unwrap_or_default();

            let mut prev_sma = if skip > 0 && skip > min_prev_i {
                f32::from(this.collect_one_at(skip - 1).unwrap())
            } else {
                0.0
            };

            // Collect only the values that leave the window during this batch.
            // At position i, source[i - window] leaves (when i >= min_prev_i + window).
            // Reads batch_size elements instead of window.
            let pop_start = skip.saturating_sub(window).max(min_prev_i);
            let pop_end = end.saturating_sub(window).max(pop_start);
            let pop_batch: Vec<f32> = if pop_end > pop_start {
                let mut v = Vec::with_capacity(pop_end - pop_start);
                source.for_each_range_dyn_at(pop_start, pop_end, &mut |val: A| {
                    v.push(f32::from(val));
                });
                v
            } else {
                vec![]
            };

            let mut pop_idx = 0;
            let mut i = skip;
            source.fold_range_at(skip, end, (), |(), value: A| {
                if min_i.is_none_or(|m| m <= i) {
                    let value_f32 = f32::from(value);
                    let effective_i = i - min_prev_i;

                    let sma_result = if effective_i >= window {
                        let old = pop_batch[pop_idx];
                        pop_idx += 1;
                        prev_sma + (value_f32 - old) / window as f32
                    } else {
                        (prev_sma * effective_i as f32 + value_f32) / (effective_i + 1) as f32
                    };

                    prev_sma = sma_result;
                    this.push(V::T::from(sma_result));
                } else {
                    this.push(V::T::from(f32::NAN));
                }
                i += 1;
            });
            Ok(())
        })
    }

    pub fn compute_rolling_median<A>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        window: usize,
        exit: &Exit,
    ) -> Result<()>
    where
        V::T: From<f32>,
        A: VecValue,
        f32: From<A>,
    {
        #[inline]
        fn median(buf: &VecDeque<f32>, scratch: &mut Vec<f32>) -> f32 {
            scratch.clear();
            scratch.extend(buf.iter().copied());
            let cmp = |a: &f32, b: &f32| a.partial_cmp(b).unwrap_or(Ordering::Equal);
            let mid = scratch.len() / 2;
            scratch.select_nth_unstable_by(mid, cmp);
            if scratch.len().is_multiple_of(2) {
                let upper = scratch[mid];
                let lower = scratch[..mid]
                    .iter()
                    .copied()
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                    .unwrap_or(upper);
                (lower + upper) / 2.0
            } else {
                scratch[mid]
            }
        }

        self.compute_init(Version::new(2) + source.version(), max_from, exit, |this| {
            let skip = this.len();
            let end = this.batch_end(source.len());
            if skip >= end {
                return Ok(());
            }

            // Median inherently needs the full window for sorting — rebuild it.
            let mut buf: VecDeque<f32> =
                VecDeque::with_capacity(window.saturating_add(1).min(1024));
            if skip > 0 {
                let start = skip.saturating_sub(window);
                source.for_each_range_dyn_at(start, skip, &mut |v: A| {
                    buf.push_back(f32::from(v));
                });
            }

            let mut scratch = Vec::with_capacity(window.min(1024));
            source.fold_range_at(skip, end, (), |(), value: A| {
                buf.push_back(f32::from(value));
                if buf.len() > window {
                    buf.pop_front();
                }
                this.push(V::T::from(median(&buf, &mut scratch)));
            });
            Ok(())
        })
    }

    pub fn compute_ema<A>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        ema: usize,
        exit: &Exit,
    ) -> Result<()>
    where
        V::T: From<A> + From<f32>,
        A: VecValue,
        f32: From<A> + From<V::T>,
    {
        self.compute_ema_(max_from, source, ema, exit, None)
    }

    pub fn compute_ema_<A>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        ema: usize,
        exit: &Exit,
        min_i: Option<V::I>,
    ) -> Result<()>
    where
        V::T: From<A> + From<f32>,
        A: VecValue,
        f32: From<A> + From<V::T>,
    {
        let k = 2.0 / (ema as f32 + 1.0);
        self.compute_exponential_smoothing(max_from, source, ema, Version::new(3), k, min_i, exit)
    }

    /// Compute Wilder's Running Moving Average (RMA).
    /// Uses alpha = 1/period instead of EMA's 2/(period+1).
    /// This is the standard smoothing method for RSI.
    pub fn compute_rma<A>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        period: usize,
        exit: &Exit,
    ) -> Result<()>
    where
        V::T: From<A> + From<f32>,
        A: VecValue,
        f32: From<A> + From<V::T>,
    {
        let k = 1.0 / period as f32;
        self.compute_exponential_smoothing(max_from, source, period, Version::new(4), k, None, exit)
    }

    /// Shared implementation for EMA and RMA.
    /// - EMA: k = 2/(period+1)
    /// - RMA (Wilder's): k = 1/period
    #[allow(clippy::too_many_arguments)]
    fn compute_exponential_smoothing<A>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        period: usize,
        version: Version,
        k: f32,
        min_i: Option<V::I>,
        exit: &Exit,
    ) -> Result<()>
    where
        V::T: From<A> + From<f32>,
        A: VecValue,
        f32: From<A> + From<V::T>,
    {
        let one_minus_k = 1.0 - k;

        self.compute_init(version + source.version(), max_from, exit, |this| {
            let skip = this.len();
            let end = this.batch_end(source.len());
            if skip >= end {
                return Ok(());
            }

            let min_start = min_i.map(|i| i.to_usize()).unwrap_or(0);

            let mut prev = if skip > 0 && skip > min_start {
                this.collect_one_at(skip - 1).unwrap()
            } else {
                V::T::from(0.0)
            };

            let mut index = skip;
            source.fold_range_at(skip, end, (), |(), value: A| {
                if index >= min_start {
                    let processed = index - min_start + 1;
                    let value = f32::from(value);

                    let p = f32::from(prev.clone());
                    let result = if processed > period {
                        let p = if p.is_nan() { 0.0 } else { p };
                        V::T::from(value * k + p * one_minus_k)
                    } else {
                        V::T::from((p * (processed - 1) as f32 + value) / processed as f32)
                    };

                    prev = result.clone();
                    this.push(result);
                } else {
                    this.push(V::T::from(f32::NAN));
                }

                index += 1;
            });
            Ok(())
        })
    }

    fn compute_all_time_extreme<A, F>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        exit: &Exit,
        compare: F,
        exclude_default: bool,
    ) -> Result<()>
    where
        V::T: From<A> + Ord + Default,
        A: VecValue,
        F: Fn(V::T, V::T) -> V::T + Copy,
    {
        let mut prev = None;
        self.compute_transform(
            max_from,
            source,
            |(i, v, this)| {
                let v = V::T::from(v);
                if prev.is_none() {
                    let idx = i.to_usize();
                    prev = Some(if idx > 0 {
                        this.collect_one_at(idx - 1).unwrap()
                    } else {
                        v.clone()
                    });
                }
                let extreme = compare(prev.as_ref().unwrap().clone(), v.clone());

                let next = if !exclude_default || extreme != V::T::default() {
                    extreme
                } else {
                    // Keep the non-default value for future comparisons
                    if v != V::T::default() {
                        v
                    } else {
                        prev.as_ref().unwrap().clone()
                    }
                };
                prev.replace(next.clone());
                (i, next)
            },
            exit,
        )
    }

    /// Computes the all time high of a source.
    /// This version is more optimized than `compute_max` with a window set to `usize::MAX`.
    pub fn compute_all_time_high<A>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        exit: &Exit,
    ) -> Result<()>
    where
        V::T: From<A> + Ord + Default,
        A: VecValue,
    {
        self.compute_all_time_extreme(max_from, source, exit, |prev, v| prev.max(v), false)
    }

    /// Computes the all time low of a source.
    /// This version is more optimized than `compute_min` with a window set to `usize::MAX`.
    pub fn compute_all_time_low<A>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        exit: &Exit,
    ) -> Result<()>
    where
        V::T: From<A> + Ord + Default,
        A: VecValue,
    {
        self.compute_all_time_low_(max_from, source, exit, false)
    }

    /// Computes the all time low of a source.
    /// This version is more optimized than `compute_min` with a window set to `usize::MAX`.
    pub fn compute_all_time_low_<A>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        exit: &Exit,
        exclude_default: bool,
    ) -> Result<()>
    where
        V::T: From<A> + Ord + Default,
        A: VecValue,
    {
        self.compute_all_time_extreme(
            max_from,
            source,
            exit,
            |prev, v| prev.min(v),
            exclude_default,
        )
    }

    /// Computes the all time high starting from a specific index.
    /// Values before `from` will be the default value (typically 0).
    pub fn compute_all_time_high_from<A>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        from: V::I,
        exit: &Exit,
    ) -> Result<()>
    where
        V::T: From<A> + Ord + Default + Copy,
        A: VecValue,
    {
        self.compute_all_time_extreme_from(max_from, source, from, exit, V::T::max)
    }

    /// Computes the all time low starting from a specific index.
    /// Values before `from` will be the default value (typically 0).
    pub fn compute_all_time_low_from<A>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        from: V::I,
        exit: &Exit,
    ) -> Result<()>
    where
        V::T: From<A> + Ord + Default + Copy,
        A: VecValue,
    {
        self.compute_all_time_extreme_from(max_from, source, from, exit, V::T::min)
    }

    fn compute_all_time_extreme_from<A>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        from: V::I,
        exit: &Exit,
        compare: fn(V::T, V::T) -> V::T,
    ) -> Result<()>
    where
        V::T: From<A> + Ord + Default + Copy,
        A: VecValue,
    {
        let from_usize = from.to_usize();
        let mut prev: Option<V::T> = None;
        self.compute_transform(
            max_from,
            source,
            |(i, v, this)| {
                let idx = i.to_usize();
                if prev.is_none() {
                    prev = Some(if idx > 0 {
                        this.collect_one_at(idx - 1).unwrap()
                    } else {
                        V::T::default()
                    });
                }
                if idx >= from_usize {
                    *prev.as_mut().unwrap() = compare(prev.unwrap(), V::T::from(v));
                }
                (i, prev.unwrap())
            },
            exit,
        )
    }

    pub fn compute_zscore<A, B, C>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, A>,
        sma: &impl ReadableVec<V::I, B>,
        sd: &impl ReadableVec<V::I, C>,
        exit: &Exit,
    ) -> Result<()>
    where
        V::T: From<f32>,
        A: VecValue + Sub<B, Output = A> + Div<C, Output = V::T>,
        B: VecValue,
        C: VecValue,
        f32: From<A> + From<B> + From<C>,
    {
        self.compute_transform3(
            max_from,
            source,
            sma,
            sd,
            |(i, ratio, sma, sd, ..)| (i, (ratio - sma) / sd),
            exit,
        )
    }
}
