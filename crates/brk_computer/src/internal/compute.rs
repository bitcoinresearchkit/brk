//! Compute functions for aggregation - take optional vecs, compute what's needed.
//!
//! These functions replace the Option-based compute logic in flexible builders.
//! Each function takes optional mutable references and computes only for Some() vecs.

use brk_error::Result;
use brk_types::{CheckedSub, StoredU64};
use schemars::JsonSchema;
use vecdb::{
    AnyStoredVec, AnyVec, EagerVec, Exit, WritableVec, ReadableVec, PcoVec, VecIndex,
    VecValue,
};

use crate::utils::get_percentile;

use super::ComputedVecValue;

/// Helper to validate and get starting index for a single vec
fn validate_and_start<I: VecIndex, T: ComputedVecValue + JsonSchema>(
    vec: &mut EagerVec<PcoVec<I, T>>,
    combined_version: vecdb::Version,
    current_start: I,
) -> Result<I> {
    vec.validate_computed_version_or_reset(combined_version)?;
    Ok(current_start.min(I::from(vec.len())))
}

/// Compute aggregations from a source vec into target vecs.
///
/// This function computes all requested aggregations in a single pass when possible,
/// optimizing for the common case where multiple aggregations are needed.
///
/// The `skip_count` parameter allows skipping the first N items from ALL calculations.
/// This is useful for excluding coinbase transactions (which have 0 fee) from
/// fee/feerate aggregations.
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
            cumulative_vec.collect_one_at(idx.to_usize()).unwrap_or(T::from(0_usize))
        })
    });

    let start = index.to_usize();
    let fi_len = first_indexes.len();
    let first_indexes_batch: Vec<A> = first_indexes.collect_range_at(start, fi_len);
    let count_indexes_batch: Vec<StoredU64> = count_indexes.collect_range_at(start, fi_len);

    first_indexes_batch.into_iter().zip(count_indexes_batch).enumerate().try_for_each(|(j, (first_index, count_index))| -> Result<()> {
            let idx = start + j;
            let count = u64::from(count_index) as usize;

            // Effective count after skipping (e.g., skip coinbase for fee calculations)
            let effective_count = count.saturating_sub(skip_count);
            let effective_first_index = first_index + skip_count.min(count);

            if let Some(ref mut first_vec) = first {
                let f = if effective_count > 0 {
                    source.collect_one_at(effective_first_index.to_usize()).unwrap()
                } else {
                    T::from(0_usize)
                };
                first_vec.truncate_push_at(idx, f)?;
            }

            if let Some(ref mut last_vec) = last {
                if effective_count == 0 {
                    // If all items skipped, use zero
                    last_vec.truncate_push_at(idx, T::from(0_usize))?;
                } else {
                    let last_index = first_index + (count - 1);
                    let v = source.collect_one_at(last_index.to_usize()).unwrap();
                    last_vec.truncate_push_at(idx, v)?;
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
                    let v = min_val.or(max_val).unwrap_or_else(|| T::from(0_usize));
                    min_vec.truncate_push_at(idx, v)?;
                }
                if let Some(ref mut max_vec) = max {
                    let v = max_val.or(min_val).unwrap_or_else(|| T::from(0_usize));
                    max_vec.truncate_push_at(idx, v)?;
                }
            } else if needs_percentiles || needs_minmax {
                let mut values: Vec<T> = source.collect_range_at(
                    effective_first_index.to_usize(),
                    effective_first_index.to_usize() + effective_count,
                );

                if values.is_empty() {
                    // Handle edge case where all items were skipped
                    if let Some(ref mut max_vec) = max {
                        max_vec.truncate_push_at(idx, T::from(0_usize))?;
                    }
                    if let Some(ref mut pct90_vec) = pct90 {
                        pct90_vec.truncate_push_at(idx, T::from(0_usize))?;
                    }
                    if let Some(ref mut pct75_vec) = pct75 {
                        pct75_vec.truncate_push_at(idx, T::from(0_usize))?;
                    }
                    if let Some(ref mut median_vec) = median {
                        median_vec.truncate_push_at(idx, T::from(0_usize))?;
                    }
                    if let Some(ref mut pct25_vec) = pct25 {
                        pct25_vec.truncate_push_at(idx, T::from(0_usize))?;
                    }
                    if let Some(ref mut pct10_vec) = pct10 {
                        pct10_vec.truncate_push_at(idx, T::from(0_usize))?;
                    }
                    if let Some(ref mut min_vec) = min {
                        min_vec.truncate_push_at(idx, T::from(0_usize))?;
                    }
                    if let Some(ref mut average_vec) = average {
                        average_vec.truncate_push_at(idx, T::from(0_usize))?;
                    }
                    if let Some(ref mut sum_vec) = sum {
                        sum_vec.truncate_push_at(idx, T::from(0_usize))?;
                    }
                    if let Some(ref mut cumulative_vec) = cumulative {
                        let t = cumulative_val.unwrap();
                        cumulative_vec.truncate_push_at(idx, t)?;
                    }
                } else if needs_percentiles {
                    // Compute aggregates from unsorted values first to avoid clone
                    let aggregate_result = if needs_aggregates {
                        let len = values.len();
                        let sum_val = values.iter().copied().fold(T::from(0), |a, b| a + b);
                        Some((len, sum_val))
                    } else {
                        None
                    };

                    // Sort in-place — no clone needed
                    values.sort_unstable();

                    if let Some(ref mut max_vec) = max {
                        max_vec.truncate_push_at(idx, *values.last().unwrap())?;
                    }
                    if let Some(ref mut pct90_vec) = pct90 {
                        pct90_vec.truncate_push_at(idx, get_percentile(&values, 0.90))?;
                    }
                    if let Some(ref mut pct75_vec) = pct75 {
                        pct75_vec.truncate_push_at(idx, get_percentile(&values, 0.75))?;
                    }
                    if let Some(ref mut median_vec) = median {
                        median_vec.truncate_push_at(idx, get_percentile(&values, 0.50))?;
                    }
                    if let Some(ref mut pct25_vec) = pct25 {
                        pct25_vec.truncate_push_at(idx, get_percentile(&values, 0.25))?;
                    }
                    if let Some(ref mut pct10_vec) = pct10 {
                        pct10_vec.truncate_push_at(idx, get_percentile(&values, 0.10))?;
                    }
                    if let Some(ref mut min_vec) = min {
                        min_vec.truncate_push_at(idx, *values.first().unwrap())?;
                    }

                    if let Some((len, sum_val)) = aggregate_result {
                        if let Some(ref mut average_vec) = average {
                            average_vec.truncate_push_at(idx, sum_val / len)?;
                        }

                        if needs_sum_or_cumulative {
                            if let Some(ref mut sum_vec) = sum {
                                sum_vec.truncate_push_at(idx, sum_val)?;
                            }
                            if let Some(ref mut cumulative_vec) = cumulative {
                                let t = cumulative_val.unwrap() + sum_val;
                                cumulative_val.replace(t);
                                cumulative_vec.truncate_push_at(idx, t)?;
                            }
                        }
                    }
                } else if needs_minmax {
                    if let Some(ref mut min_vec) = min {
                        min_vec.truncate_push_at(idx, *values.iter().min().unwrap())?;
                    }
                    if let Some(ref mut max_vec) = max {
                        max_vec.truncate_push_at(idx, *values.iter().max().unwrap())?;
                    }

                    if needs_aggregates {
                        let len = values.len();
                        let sum_val = values.into_iter().fold(T::from(0), |a, b| a + b);

                        if let Some(ref mut average_vec) = average {
                            average_vec.truncate_push_at(idx, sum_val / len)?;
                        }

                        if needs_sum_or_cumulative {
                            if let Some(ref mut sum_vec) = sum {
                                sum_vec.truncate_push_at(idx, sum_val)?;
                            }
                            if let Some(ref mut cumulative_vec) = cumulative {
                                let t = cumulative_val.unwrap() + sum_val;
                                cumulative_val.replace(t);
                                cumulative_vec.truncate_push_at(idx, t)?;
                            }
                        }
                    }
                }
            } else if needs_aggregates {
                // Aggregates only (sum/average/cumulative) — no Vec allocation needed
                let efi = effective_first_index.to_usize();
                let (sum_val, len) = source.fold_range_at(efi, efi + effective_count, (T::from(0_usize), 0_usize), |(acc, cnt), val| (acc + val, cnt + 1));

                if let Some(ref mut average_vec) = average {
                    let avg = if len > 0 { sum_val / len } else { T::from(0_usize) };
                    average_vec.truncate_push_at(idx, avg)?;
                }

                if needs_sum_or_cumulative {
                    if let Some(ref mut sum_vec) = sum {
                        sum_vec.truncate_push_at(idx, sum_val)?;
                    }
                    if let Some(ref mut cumulative_vec) = cumulative {
                        let t = cumulative_val.unwrap() + sum_val;
                        cumulative_val.replace(t);
                        cumulative_vec.truncate_push_at(idx, t)?;
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
