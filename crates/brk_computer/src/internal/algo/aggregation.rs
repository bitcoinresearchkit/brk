use brk_error::Result;
use brk_types::{CheckedSub, StoredU64};
use schemars::JsonSchema;
use vecdb::{
    AnyStoredVec, AnyVec, EagerVec, Exit, PcoVec, ReadableVec, VecIndex, VecValue, WritableVec,
};

use brk_types::get_percentile;

use crate::internal::ComputedVecValue;

fn validate_and_start<I: VecIndex, T: ComputedVecValue + JsonSchema>(
    vec: &mut EagerVec<PcoVec<I, T>>,
    combined_version: vecdb::Version,
    current_start: I,
) -> Result<I> {
    vec.validate_computed_version_or_reset(combined_version)?;
    Ok(current_start.min(I::from(vec.len())))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn compute_aggregations<I, T, A>(
    max_from: I,
    source: &impl ReadableVec<A, T>,
    first_indexes: &impl ReadableVec<I, A>,
    count_indexes: &impl ReadableVec<I, StoredU64>,
    exit: &Exit,
    skip_count: usize,
    mut first: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut last: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut min: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut max: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut average: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut sum: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut cumulative: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut median: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut pct10: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut pct25: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut pct75: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut pct90: Option<&mut EagerVec<PcoVec<I, T>>>,
) -> Result<()>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
    A: VecIndex + VecValue + CheckedSub<A>,
{
    let combined_version = source.version() + first_indexes.version() + count_indexes.version();

    macro_rules! validate_vec {
        ($($vec:ident),*) => {{
            let mut idx = max_from;
            $(if let Some(ref mut v) = $vec {
                idx = validate_and_start(v, combined_version, idx)?;
            })*
            idx
        }};
    }

    let index = validate_vec!(
        first, last, min, max, average, sum, cumulative, median, pct10, pct25, pct75, pct90
    );

    let needs_first = first.is_some();
    let needs_last = last.is_some();
    let needs_min = min.is_some();
    let needs_max = max.is_some();
    let needs_average = average.is_some();
    let needs_sum = sum.is_some();
    let needs_cumulative = cumulative.is_some();
    let needs_percentiles = median.is_some()
        || pct10.is_some()
        || pct25.is_some()
        || pct75.is_some()
        || pct90.is_some();
    let needs_minmax = needs_min || needs_max;
    let needs_sum_or_cumulative = needs_sum || needs_cumulative;
    let needs_aggregates = needs_sum_or_cumulative || needs_average;

    if !needs_first && !needs_last && !needs_minmax && !needs_aggregates && !needs_percentiles {
        return Ok(());
    }

    let mut cumulative_val = cumulative.as_ref().map(|cumulative_vec| {
        index.decremented().map_or(T::from(0_usize), |idx| {
            cumulative_vec
                .collect_one_at(idx.to_usize())
                .unwrap_or(T::from(0_usize))
        })
    });

    let start = index.to_usize();

    // Truncate all vecs to start once, so the loop only pushes
    macro_rules! truncate_vec {
        ($($vec:ident),*) => {
            $(if let Some(ref mut v) = $vec {
                v.truncate_if_needed_at(start)?;
            })*
        };
    }
    truncate_vec!(first, last, min, max, average, sum, cumulative, median, pct10, pct25, pct75, pct90);

    let fi_len = first_indexes.len();
    let first_indexes_batch: Vec<A> = first_indexes.collect_range_at(start, fi_len);
    let count_indexes_batch: Vec<StoredU64> = count_indexes.collect_range_at(start, fi_len);

    first_indexes_batch
        .into_iter()
        .zip(count_indexes_batch)
        .enumerate()
        .try_for_each(|(_, (first_index, count_index))| -> Result<()> {
            let count = u64::from(count_index) as usize;

            // Effective count after skipping (e.g., skip coinbase for fee calculations)
            let effective_count = count.saturating_sub(skip_count);
            let effective_first_index = first_index + skip_count.min(count);

            if let Some(ref mut first_vec) = first {
                let f = if effective_count > 0 {
                    source
                        .collect_one_at(effective_first_index.to_usize())
                        .unwrap()
                } else {
                    T::from(0_usize)
                };
                first_vec.push(f);
            }

            if let Some(ref mut last_vec) = last {
                if effective_count == 0 {
                    last_vec.push(T::from(0_usize));
                } else {
                    let last_index = first_index + (count - 1);
                    last_vec.push(source.collect_one_at(last_index.to_usize()).unwrap());
                }
            }

            // Fast path: only min/max needed, no sorting or allocation required
            if needs_minmax && !needs_percentiles && !needs_aggregates {
                let efi = effective_first_index.to_usize();
                let mut min_val: Option<T> = None;
                let mut max_val: Option<T> = None;

                source.for_each_range_at(efi, efi + effective_count, |val| {
                    if needs_min {
                        min_val = Some(min_val.map_or(val, |m| if val < m { val } else { m }));
                    }
                    if needs_max {
                        max_val = Some(max_val.map_or(val, |m| if val > m { val } else { m }));
                    }
                });

                if let Some(ref mut min_vec) = min {
                    min_vec.push(min_val.or(max_val).unwrap_or_else(|| T::from(0_usize)));
                }
                if let Some(ref mut max_vec) = max {
                    max_vec.push(max_val.or(min_val).unwrap_or_else(|| T::from(0_usize)));
                }
            } else if needs_percentiles || needs_minmax {
                let mut values: Vec<T> = source.collect_range_at(
                    effective_first_index.to_usize(),
                    effective_first_index.to_usize() + effective_count,
                );

                if values.is_empty() {
                    macro_rules! push_zero {
                        ($($vec:ident),*) => {
                            $(if let Some(ref mut v) = $vec {
                                v.push(T::from(0_usize));
                            })*
                        };
                    }
                    push_zero!(max, pct90, pct75, median, pct25, pct10, min, average, sum);
                    if let Some(ref mut cumulative_vec) = cumulative {
                        cumulative_vec.push(cumulative_val.unwrap());
                    }
                } else if needs_percentiles {
                    let aggregate_result = if needs_aggregates {
                        let len = values.len();
                        let sum_val = values.iter().copied().fold(T::from(0), |a, b| a + b);
                        Some((len, sum_val))
                    } else {
                        None
                    };

                    values.sort_unstable();

                    if let Some(ref mut max_vec) = max {
                        max_vec.push(*values.last().unwrap());
                    }
                    if let Some(ref mut pct90_vec) = pct90 {
                        pct90_vec.push(get_percentile(&values, 0.90));
                    }
                    if let Some(ref mut pct75_vec) = pct75 {
                        pct75_vec.push(get_percentile(&values, 0.75));
                    }
                    if let Some(ref mut median_vec) = median {
                        median_vec.push(get_percentile(&values, 0.50));
                    }
                    if let Some(ref mut pct25_vec) = pct25 {
                        pct25_vec.push(get_percentile(&values, 0.25));
                    }
                    if let Some(ref mut pct10_vec) = pct10 {
                        pct10_vec.push(get_percentile(&values, 0.10));
                    }
                    if let Some(ref mut min_vec) = min {
                        min_vec.push(*values.first().unwrap());
                    }

                    if let Some((len, sum_val)) = aggregate_result {
                        if let Some(ref mut average_vec) = average {
                            average_vec.push(sum_val / len);
                        }

                        if needs_sum_or_cumulative {
                            if let Some(ref mut sum_vec) = sum {
                                sum_vec.push(sum_val);
                            }
                            if let Some(ref mut cumulative_vec) = cumulative {
                                let t = cumulative_val.unwrap() + sum_val;
                                cumulative_val.replace(t);
                                cumulative_vec.push(t);
                            }
                        }
                    }
                } else if needs_minmax {
                    if let Some(ref mut min_vec) = min {
                        min_vec.push(*values.iter().min().unwrap());
                    }
                    if let Some(ref mut max_vec) = max {
                        max_vec.push(*values.iter().max().unwrap());
                    }

                    if needs_aggregates {
                        let len = values.len();
                        let sum_val = values.into_iter().fold(T::from(0), |a, b| a + b);

                        if let Some(ref mut average_vec) = average {
                            average_vec.push(sum_val / len);
                        }

                        if needs_sum_or_cumulative {
                            if let Some(ref mut sum_vec) = sum {
                                sum_vec.push(sum_val);
                            }
                            if let Some(ref mut cumulative_vec) = cumulative {
                                let t = cumulative_val.unwrap() + sum_val;
                                cumulative_val.replace(t);
                                cumulative_vec.push(t);
                            }
                        }
                    }
                }
            } else if needs_aggregates {
                let efi = effective_first_index.to_usize();
                let (sum_val, len) = source.fold_range_at(
                    efi,
                    efi + effective_count,
                    (T::from(0_usize), 0_usize),
                    |(acc, cnt), val| (acc + val, cnt + 1),
                );

                if let Some(ref mut average_vec) = average {
                    let avg = if len > 0 {
                        sum_val / len
                    } else {
                        T::from(0_usize)
                    };
                    average_vec.push(avg);
                }

                if needs_sum_or_cumulative {
                    if let Some(ref mut sum_vec) = sum {
                        sum_vec.push(sum_val);
                    }
                    if let Some(ref mut cumulative_vec) = cumulative {
                        let t = cumulative_val.unwrap() + sum_val;
                        cumulative_val.replace(t);
                        cumulative_vec.push(t);
                    }
                }
            }

            Ok(())
        })?;

    let _lock = exit.lock();

    macro_rules! write_vec {
        ($($vec:ident),*) => {
            $(if let Some(v) = $vec { v.write()?; })*
        };
    }

    write_vec!(
        first, last, min, max, average, sum, cumulative, median, pct10, pct25, pct75, pct90
    );

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn compute_aggregations_nblock_window<I, T, A>(
    max_from: I,
    source: &impl ReadableVec<A, T>,
    first_indexes: &impl ReadableVec<I, A>,
    count_indexes: &impl ReadableVec<I, StoredU64>,
    n_blocks: usize,
    exit: &Exit,
    min: &mut EagerVec<PcoVec<I, T>>,
    max: &mut EagerVec<PcoVec<I, T>>,
    average: &mut EagerVec<PcoVec<I, T>>,
    median: &mut EagerVec<PcoVec<I, T>>,
    pct10: &mut EagerVec<PcoVec<I, T>>,
    pct25: &mut EagerVec<PcoVec<I, T>>,
    pct75: &mut EagerVec<PcoVec<I, T>>,
    pct90: &mut EagerVec<PcoVec<I, T>>,
) -> Result<()>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
    A: VecIndex + VecValue + CheckedSub<A>,
{
    let combined_version = source.version() + first_indexes.version() + count_indexes.version();

    let mut idx = max_from;
    for vec in [
        &mut *min,
        &mut *max,
        &mut *average,
        &mut *median,
        &mut *pct10,
        &mut *pct25,
        &mut *pct75,
        &mut *pct90,
    ] {
        idx = validate_and_start(vec, combined_version, idx)?;
    }
    let index = idx;

    let start = index.to_usize();
    let fi_len = first_indexes.len();

    // Only fetch first_indexes from the earliest possible window start
    let batch_start = start.saturating_sub(n_blocks - 1);
    let first_indexes_batch: Vec<A> = first_indexes.collect_range_at(batch_start, fi_len);
    let count_indexes_batch: Vec<StoredU64> = count_indexes.collect_range_at(start, fi_len);

    let zero = T::from(0_usize);
    let mut values: Vec<T> = Vec::new();

    for vec in [
        &mut *min,
        &mut *max,
        &mut *average,
        &mut *median,
        &mut *pct10,
        &mut *pct25,
        &mut *pct75,
        &mut *pct90,
    ] {
        vec.truncate_if_needed_at(start)?;
    }

    count_indexes_batch
        .iter()
        .enumerate()
        .try_for_each(|(j, ci)| -> Result<()> {
            let idx = start + j;

            // Window start: max(0, idx - n_blocks + 1)
            let window_start = idx.saturating_sub(n_blocks - 1);

            // Last tx index (exclusive) of current block
            let count = u64::from(*ci) as usize;
            let fi = first_indexes_batch[idx - batch_start];
            let range_end_usize = fi.to_usize() + count;

            // First tx index of the window start block
            let range_start_usize = first_indexes_batch[window_start - batch_start].to_usize();

            let effective_count = range_end_usize.saturating_sub(range_start_usize);

            if effective_count == 0 {
                for vec in [
                    &mut *min,
                    &mut *max,
                    &mut *average,
                    &mut *median,
                    &mut *pct10,
                    &mut *pct25,
                    &mut *pct75,
                    &mut *pct90,
                ] {
                    vec.push(zero);
                }
            } else {
                source.collect_range_into_at(range_start_usize, range_end_usize, &mut values);

                // Compute sum before sorting
                let len = values.len();
                let sum_val = values.iter().copied().fold(T::from(0), |a, b| a + b);
                let avg = sum_val / len;

                values.sort_unstable();

                max.push(*values.last().unwrap());
                pct90.push(get_percentile(&values, 0.90));
                pct75.push(get_percentile(&values, 0.75));
                median.push(get_percentile(&values, 0.50));
                pct25.push(get_percentile(&values, 0.25));
                pct10.push(get_percentile(&values, 0.10));
                min.push(*values.first().unwrap());
                average.push(avg);
            }

            Ok(())
        })?;

    let _lock = exit.lock();
    for vec in [min, max, average, median, pct10, pct25, pct75, pct90] {
        vec.write()?;
    }

    Ok(())
}
