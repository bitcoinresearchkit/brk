//! Compute functions for aggregation - take optional vecs, compute what's needed.
//!
//! These functions replace the Option-based compute logic in flexible builders.
//! Each function takes optional mutable references and computes only for Some() vecs.

use brk_error::{Error, Result};
use brk_types::{CheckedSub, StoredU64};
use schemars::JsonSchema;
use vecdb::{
    AnyStoredVec, AnyVec, EagerVec, Exit, GenericStoredVec, IterableVec, PcoVec, VecIndex, VecValue,
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
#[allow(clippy::too_many_arguments)]
pub fn compute_aggregations<I, T, A>(
    max_from: I,
    source: &impl IterableVec<A, T>,
    first_indexes: &impl IterableVec<I, A>,
    count_indexes: &impl IterableVec<I, StoredU64>,
    exit: &Exit,
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

    let index = validate_vec!(first, last, min, max, average, sum, cumulative, median, pct10, pct25, pct75, pct90);

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

    let mut source_iter = source.iter();

    let mut cumulative_val = cumulative.as_ref().map(|cumulative_vec| {
        index.decremented().map_or(T::from(0_usize), |idx| {
            cumulative_vec.iter().get_unwrap(idx)
        })
    });

    let mut count_indexes_iter = count_indexes.iter().skip(index.to_usize());

    first_indexes
        .iter()
        .enumerate()
        .skip(index.to_usize())
        .try_for_each(|(idx, first_index)| -> Result<()> {
            let count_index = count_indexes_iter.next().unwrap();
            let count = *count_index as usize;

            if let Some(ref mut first_vec) = first {
                let f = source_iter
                    .get(first_index)
                    .unwrap_or_else(|| T::from(0_usize));
                first_vec.truncate_push_at(idx, f)?;
            }

            if let Some(ref mut last_vec) = last {
                if count == 0 {
                    panic!("should not compute last if count can be 0");
                }
                let last_index = first_index + (count - 1);
                let v = source_iter.get_unwrap(last_index);
                last_vec.truncate_push_at(idx, v)?;
            }

            // Fast path: only min/max needed, no sorting or allocation required
            if needs_minmax && !needs_percentiles && !needs_aggregates {
                source_iter.set_position(first_index);
                let mut min_val: Option<T> = None;
                let mut max_val: Option<T> = None;

                for val in (&mut source_iter).take(count) {
                    if needs_min {
                        min_val = Some(min_val.map_or(val, |m| if val < m { val } else { m }));
                    }
                    if needs_max {
                        max_val = Some(max_val.map_or(val, |m| if val > m { val } else { m }));
                    }
                }

                if let Some(ref mut min_vec) = min {
                    min_vec.truncate_push_at(idx, min_val.unwrap())?;
                }
                if let Some(ref mut max_vec) = max {
                    max_vec.truncate_push_at(idx, max_val.unwrap())?;
                }
            } else if needs_percentiles || needs_aggregates || needs_minmax {
                source_iter.set_position(first_index);
                let mut values: Vec<T> = (&mut source_iter).take(count).collect();

                if needs_percentiles {
                    values.sort_unstable();

                    if let Some(ref mut max_vec) = max {
                        max_vec.truncate_push_at(
                            idx,
                            *values
                                .last()
                                .ok_or(Error::Internal("Empty values for percentiles"))?,
                        )?;
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
                } else if needs_minmax {
                    if let Some(ref mut min_vec) = min {
                        min_vec.truncate_push_at(idx, *values.iter().min().unwrap())?;
                    }
                    if let Some(ref mut max_vec) = max {
                        max_vec.truncate_push_at(idx, *values.iter().max().unwrap())?;
                    }
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

            Ok(())
        })?;

    let _lock = exit.lock();

    macro_rules! write_vec {
        ($($vec:ident),*) => {
            $(if let Some(v) = $vec { v.write()?; })*
        };
    }

    write_vec!(first, last, min, max, average, sum, cumulative, median, pct10, pct25, pct75, pct90);

    Ok(())
}

/// Compute cumulative extension from a source vec.
///
/// Used when only cumulative needs to be extended from an existing source.
pub fn compute_cumulative_extend<I, T>(
    max_from: I,
    source: &impl IterableVec<I, T>,
    cumulative: &mut EagerVec<PcoVec<I, T>>,
    exit: &Exit,
) -> Result<()>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
{
    cumulative.validate_computed_version_or_reset(source.version())?;

    let index = max_from.min(I::from(cumulative.len()));

    let mut cumulative_val = index
        .decremented()
        .map_or(T::from(0_usize), |idx| cumulative.iter().get_unwrap(idx));

    source
        .iter()
        .enumerate()
        .skip(index.to_usize())
        .try_for_each(|(i, v)| -> Result<()> {
            cumulative_val += v;
            cumulative.truncate_push_at(i, cumulative_val)?;
            Ok(())
        })?;

    let _lock = exit.lock();
    cumulative.write()?;

    Ok(())
}

/// Compute coarser aggregations from already-aggregated source data.
///
/// This is used for dateindex → weekindex, monthindex, etc. where we derive
/// coarser aggregations from finer ones.
///
/// NOTE: Percentiles are NOT supported - they cannot be derived from finer percentiles.
#[allow(clippy::too_many_arguments)]
pub fn compute_aggregations_from_aligned<I, T, A>(
    max_from: I,
    first_indexes: &impl IterableVec<I, A>,
    count_indexes: &impl IterableVec<I, StoredU64>,
    exit: &Exit,
    // Source vecs (already aggregated at finer level)
    source_first: Option<&EagerVec<PcoVec<A, T>>>,
    source_last: Option<&EagerVec<PcoVec<A, T>>>,
    source_min: Option<&EagerVec<PcoVec<A, T>>>,
    source_max: Option<&EagerVec<PcoVec<A, T>>>,
    source_average: Option<&EagerVec<PcoVec<A, T>>>,
    source_sum: Option<&EagerVec<PcoVec<A, T>>>,
    // Target vecs
    mut first: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut last: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut min: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut max: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut average: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut sum: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut cumulative: Option<&mut EagerVec<PcoVec<I, T>>>,
) -> Result<()>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
    A: VecIndex + VecValue + CheckedSub<A>,
{
    let combined_version = first_indexes.version() + count_indexes.version();

    macro_rules! validate_vec {
        ($($vec:ident),*) => {{
            let mut idx = max_from;
            $(if let Some(ref mut v) = $vec {
                idx = validate_and_start(v, combined_version, idx)?;
            })*
            idx
        }};
    }

    let index = validate_vec!(first, last, min, max, average, sum, cumulative);

    let needs_first = first.is_some();
    let needs_last = last.is_some();
    let needs_min = min.is_some();
    let needs_max = max.is_some();
    let needs_average = average.is_some();
    let needs_sum = sum.is_some();
    let needs_cumulative = cumulative.is_some();

    if !needs_first
        && !needs_last
        && !needs_min
        && !needs_max
        && !needs_average
        && !needs_sum
        && !needs_cumulative
    {
        return Ok(());
    }

    let mut source_first_iter = source_first.map(|f| f.iter());
    let mut source_last_iter = source_last.map(|f| f.iter());
    let mut source_min_iter = source_min.map(|f| f.iter());
    let mut source_max_iter = source_max.map(|f| f.iter());
    let mut source_average_iter = source_average.map(|f| f.iter());
    let mut source_sum_iter = source_sum.map(|f| f.iter());

    let mut cumulative_val = cumulative.as_ref().map(|cumulative_vec| {
        index.decremented().map_or(T::from(0_usize), |idx| {
            cumulative_vec.iter().get_unwrap(idx)
        })
    });

    let mut count_indexes_iter = count_indexes.iter().skip(index.to_usize());

    first_indexes
        .iter()
        .enumerate()
        .skip(index.to_usize())
        .try_for_each(|(idx, first_index)| -> Result<()> {
            let count_index = count_indexes_iter.next().unwrap();
            let count = *count_index as usize;

            if let Some(ref mut first_vec) = first {
                let source_iter = source_first_iter
                    .as_mut()
                    .expect("source_first required for first");
                let v = source_iter.get_unwrap(first_index);
                first_vec.truncate_push_at(idx, v)?;
            }

            if let Some(ref mut last_vec) = last {
                if count == 0 {
                    panic!("should not compute last if count can be 0");
                }
                let last_index = first_index + (count - 1);
                let source_iter = source_last_iter
                    .as_mut()
                    .expect("source_last required for last");
                let v = source_iter.get_unwrap(last_index);
                last_vec.truncate_push_at(idx, v)?;
            }

            if let Some(ref mut min_vec) = min {
                let source_iter = source_min_iter
                    .as_mut()
                    .expect("source_min required for min");
                source_iter.set_position(first_index);
                let min_val = source_iter.take(count).min().unwrap();
                min_vec.truncate_push_at(idx, min_val)?;
            }

            if let Some(ref mut max_vec) = max {
                let source_iter = source_max_iter
                    .as_mut()
                    .expect("source_max required for max");
                source_iter.set_position(first_index);
                let max_val = source_iter.take(count).max().unwrap();
                max_vec.truncate_push_at(idx, max_val)?;
            }

            if let Some(ref mut average_vec) = average {
                let source_iter = source_average_iter
                    .as_mut()
                    .expect("source_average required for average");
                source_iter.set_position(first_index);
                let mut len = 0usize;
                let sum_val = (&mut *source_iter)
                    .take(count)
                    .inspect(|_| len += 1)
                    .fold(T::from(0), |a, b| a + b);
                // TODO: Multiply by count then divide by cumulative for accuracy
                let average = sum_val / len;
                average_vec.truncate_push_at(idx, average)?;
            }

            if needs_sum || needs_cumulative {
                let source_iter = source_sum_iter
                    .as_mut()
                    .expect("source_sum required for sum/cumulative");
                source_iter.set_position(first_index);
                let sum_val = source_iter.take(count).fold(T::from(0), |a, b| a + b);

                if let Some(ref mut sum_vec) = sum {
                    sum_vec.truncate_push_at(idx, sum_val)?;
                }

                if let Some(ref mut cumulative_vec) = cumulative {
                    let t = cumulative_val.unwrap() + sum_val;
                    cumulative_val.replace(t);
                    cumulative_vec.truncate_push_at(idx, t)?;
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

    write_vec!(first, last, min, max, average, sum, cumulative);

    Ok(())
}
