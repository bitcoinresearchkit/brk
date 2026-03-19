use std::collections::VecDeque;

use brk_error::Result;
use brk_types::{CheckedSub, StoredU64, get_percentile};
use schemars::JsonSchema;
use vecdb::{
    AnyStoredVec, AnyVec, EagerVec, Exit, PcoVec, ReadableVec, VecIndex, VecValue, WritableVec,
};

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
    mut min: Option<&mut EagerVec<PcoVec<I, T>>>,
    mut max: Option<&mut EagerVec<PcoVec<I, T>>>,
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
        min, max, sum, cumulative, median, pct10, pct25, pct75, pct90
    );

    let needs_min = min.is_some();
    let needs_max = max.is_some();
    let needs_sum = sum.is_some();
    let needs_cumulative = cumulative.is_some();
    let needs_percentiles = median.is_some()
        || pct10.is_some()
        || pct25.is_some()
        || pct75.is_some()
        || pct90.is_some();
    let needs_minmax = needs_min || needs_max;
    let needs_sum_or_cumulative = needs_sum || needs_cumulative;

    if !needs_minmax && !needs_sum_or_cumulative && !needs_percentiles {
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
    truncate_vec!(
        min, max, sum, cumulative, median, pct10, pct25, pct75, pct90
    );

    let fi_len = first_indexes.len();
    let first_indexes_batch: Vec<A> = first_indexes.collect_range_at(start, fi_len);
    let count_indexes_batch: Vec<StoredU64> = count_indexes.collect_range_at(start, fi_len);

    let mut values: Vec<T> = Vec::new();

    first_indexes_batch
        .into_iter()
        .zip(count_indexes_batch)
        .enumerate()
        .try_for_each(|(_, (first_index, count_index))| -> Result<()> {
            let count = u64::from(count_index) as usize;

            // Effective count after skipping (e.g., skip coinbase for fee calculations)
            let effective_count = count.saturating_sub(skip_count);
            let effective_first_index = first_index + skip_count.min(count);

            // Fast path: only min/max needed, no sorting or allocation required
            if needs_minmax && !needs_percentiles && !needs_sum_or_cumulative {
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
                source.collect_range_into_at(
                    effective_first_index.to_usize(),
                    effective_first_index.to_usize() + effective_count,
                    &mut values,
                );

                if values.is_empty() {
                    macro_rules! push_zero {
                        ($($vec:ident),*) => {
                            $(if let Some(ref mut v) = $vec {
                                v.push(T::from(0_usize));
                            })*
                        };
                    }
                    push_zero!(max, pct90, pct75, median, pct25, pct10, min, sum);
                    if let Some(ref mut cumulative_vec) = cumulative {
                        cumulative_vec.push(cumulative_val.unwrap());
                    }
                } else if needs_percentiles {
                    let sum_val = if needs_sum_or_cumulative {
                        Some(values.iter().copied().fold(T::from(0), |a, b| a + b))
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

                    if let Some(sum_val) = sum_val {
                        if let Some(ref mut sum_vec) = sum {
                            sum_vec.push(sum_val);
                        }
                        if let Some(ref mut cumulative_vec) = cumulative {
                            let t = cumulative_val.unwrap() + sum_val;
                            cumulative_val.replace(t);
                            cumulative_vec.push(t);
                        }
                    }
                } else if needs_minmax {
                    // Single pass for min + max + optional sum
                    let (min_val, max_val, sum_val, _len) = values.iter().copied().fold(
                        (values[0], values[0], T::from(0_usize), 0_usize),
                        |(mn, mx, s, c), v| (mn.min(v), mx.max(v), s + v, c + 1),
                    );

                    if let Some(ref mut min_vec) = min {
                        min_vec.push(min_val);
                    }
                    if let Some(ref mut max_vec) = max {
                        max_vec.push(max_val);
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
            } else if needs_sum_or_cumulative {
                let efi = effective_first_index.to_usize();
                let sum_val = source.fold_range_at(
                    efi,
                    efi + effective_count,
                    T::from(0_usize),
                    |acc, val| acc + val,
                );

                if let Some(ref mut sum_vec) = sum {
                    sum_vec.push(sum_val);
                }
                if let Some(ref mut cumulative_vec) = cumulative {
                    let t = cumulative_val.unwrap() + sum_val;
                    cumulative_val.replace(t);
                    cumulative_vec.push(t);
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
        min, max, sum, cumulative, median, pct10, pct25, pct75, pct90
    );

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn compute_aggregations_nblock_window<I, T, A>(
    max_from: I,
    source: &(impl ReadableVec<A, T> + Sized),
    first_indexes: &impl ReadableVec<I, A>,
    count_indexes: &impl ReadableVec<I, StoredU64>,
    n_blocks: usize,
    exit: &Exit,
    min: &mut EagerVec<PcoVec<I, T>>,
    max: &mut EagerVec<PcoVec<I, T>>,
    median: &mut EagerVec<PcoVec<I, T>>,
    pct10: &mut EagerVec<PcoVec<I, T>>,
    pct25: &mut EagerVec<PcoVec<I, T>>,
    pct75: &mut EagerVec<PcoVec<I, T>>,
    pct90: &mut EagerVec<PcoVec<I, T>>,
) -> Result<()>
where
    I: VecIndex,
    T: ComputedVecValue + CheckedSub + JsonSchema,
    A: VecIndex + VecValue + CheckedSub<A>,
{
    let combined_version = source.version() + first_indexes.version() + count_indexes.version();

    let mut idx = max_from;
    for vec in [
        &mut *min,
        &mut *max,
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

    let batch_start = start.saturating_sub(n_blocks - 1);
    let first_indexes_batch: Vec<A> = first_indexes.collect_range_at(batch_start, fi_len);
    let count_indexes_all: Vec<StoredU64> = count_indexes.collect_range_at(batch_start, fi_len);

    let zero = T::from(0_usize);

    for vec in [
        &mut *min,
        &mut *max,
        &mut *median,
        &mut *pct10,
        &mut *pct25,
        &mut *pct75,
        &mut *pct90,
    ] {
        vec.truncate_if_needed_at(start)?;
    }

    // Persistent sorted window: O(n) merge-insert for new block, O(n) merge-filter
    // for expired block. Avoids re-sorting every block. Cursor reads only the new
    // block (~1 page decompress vs original's ~4). Ring buffer caches per-block
    // sorted values for O(1) expiry.
    // Peak memory: 2 × ~15k window elements + n_blocks × ~2500 cached ≈ 360 KB.
    let mut block_ring: VecDeque<Vec<T>> = VecDeque::with_capacity(n_blocks + 1);
    let mut cursor = source.cursor();
    let mut sorted_window: Vec<T> = Vec::new();
    let mut merge_buf: Vec<T> = Vec::new();

    // Pre-fill initial window blocks [window_start_of_first..start)
    let window_start_of_first = start.saturating_sub(n_blocks - 1);
    for block_idx in window_start_of_first..start {
        let fi = first_indexes_batch[block_idx - batch_start].to_usize();
        let count = u64::from(count_indexes_all[block_idx - batch_start]) as usize;
        if cursor.position() < fi {
            cursor.advance(fi - cursor.position());
        }
        let mut bv = Vec::with_capacity(count);
        cursor.for_each(count, |v: T| bv.push(v));
        bv.sort_unstable();
        sorted_window.extend_from_slice(&bv);
        block_ring.push_back(bv);
    }
    // Initial sorted_window was built by extending individually sorted blocks —
    // stable sort detects these sorted runs and merges in O(n × log(k)) instead of O(n log n).
    sorted_window.sort();

    for j in 0..(fi_len - start) {
        let idx = start + j;

        // Read and sort new block's values
        let fi = first_indexes_batch[idx - batch_start].to_usize();
        let count = u64::from(count_indexes_all[idx - batch_start]) as usize;
        if cursor.position() < fi {
            cursor.advance(fi - cursor.position());
        }
        let mut new_block = Vec::with_capacity(count);
        cursor.for_each(count, |v: T| new_block.push(v));
        new_block.sort_unstable();

        // Merge-insert new sorted block into sorted_window: O(n+m)
        merge_buf.clear();
        merge_buf.reserve(sorted_window.len() + new_block.len());
        let (mut si, mut ni) = (0, 0);
        while si < sorted_window.len() && ni < new_block.len() {
            if sorted_window[si] <= new_block[ni] {
                merge_buf.push(sorted_window[si]);
                si += 1;
            } else {
                merge_buf.push(new_block[ni]);
                ni += 1;
            }
        }
        merge_buf.extend_from_slice(&sorted_window[si..]);
        merge_buf.extend_from_slice(&new_block[ni..]);
        std::mem::swap(&mut sorted_window, &mut merge_buf);

        block_ring.push_back(new_block);

        // Expire oldest block: merge-filter its sorted values from sorted_window in O(n)
        if block_ring.len() > n_blocks {
            let expired = block_ring.pop_front().unwrap();

            merge_buf.clear();
            merge_buf.reserve(sorted_window.len());
            let mut ei = 0;
            for &v in &sorted_window {
                if ei < expired.len() && v == expired[ei] {
                    ei += 1;
                } else {
                    merge_buf.push(v);
                }
            }
            std::mem::swap(&mut sorted_window, &mut merge_buf);
        }

        if sorted_window.is_empty() {
            for vec in [
                &mut *min,
                &mut *max,
                &mut *median,
                &mut *pct10,
                &mut *pct25,
                &mut *pct75,
                &mut *pct90,
            ] {
                vec.push(zero);
            }
        } else {
            max.push(*sorted_window.last().unwrap());
            pct90.push(get_percentile(&sorted_window, 0.90));
            pct75.push(get_percentile(&sorted_window, 0.75));
            median.push(get_percentile(&sorted_window, 0.50));
            pct25.push(get_percentile(&sorted_window, 0.25));
            pct10.push(get_percentile(&sorted_window, 0.10));
            min.push(*sorted_window.first().unwrap());
        }
    }

    let _lock = exit.lock();
    for vec in [min, max, median, pct10, pct25, pct75, pct90] {
        vec.write()?;
    }

    Ok(())
}
