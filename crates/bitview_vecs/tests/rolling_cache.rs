#![cfg(feature = "diagnostics")]

use bitview_collections::Windows;
use bitview_vecs::{
    CachedWindowStartVec, LazyPerBlockCumulativeRolling, LazyWindowStartVec,
    PerBlockCumulativeRolling,
};
use brk_types::{Height, StoredF32, StoredU64, Timestamp, Version};
use tempfile::tempdir;
use vecdb::{AnyStoredVec, CachedVec, Database, ReadableVec, VecIndex, WritableVec, diagnostics};

use crate::common::CACHE_BUDGET;

#[allow(dead_code)]
mod common;

#[test]
fn rolling_resolutions_share_the_cumulative_cache_without_caching_derivations() {
    const N: usize = 32_768;
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut indexes = common::indexes(&db);
    let days: Vec<_> = (0..N).step_by(144).collect();
    indexes.first_height.day1 = CachedVec::wrap(common::stored(
        &db,
        "days",
        days.iter().copied().map(Height::from),
    ))
    .read_only_boxed_clone();
    // Warm metadata separately; only source-value decompression is counted below.
    indexes.first_height.day1.snapshot();
    let timestamps = CachedVec::wrap(common::stored::<Height, _>(
        &db,
        "timestamps",
        (0..N).map(|i| Timestamp::from((i * 600) as u32)),
    ));
    let starts = Windows {
        _24h: 1,
        _1w: 7,
        _1m: 30,
        _1y: 365,
    }
    .map_with_suffix(|suffix, &days| {
        CachedWindowStartVec::new(LazyWindowStartVec::days(
            suffix,
            Version::ONE,
            days,
            timestamps.read_only_cached_boxed_clone(),
        ))
    });
    let starts_ref = Windows {
        _24h: &starts._24h,
        _1w: &starts._1w,
        _1m: &starts._1m,
        _1y: &starts._1y,
    };
    for start in starts.as_array() {
        start.snapshot();
    }
    let mut metric = PerBlockCumulativeRolling::<StoredU64>::forced_import(
        &CACHE_BUDGET,
        &db,
        "metric",
        Version::ONE,
        &indexes,
        &starts_ref,
    )
    .unwrap();
    let mut total = 0u64;
    let truth: Vec<_> = (0..N)
        .map(|i| {
            let value = (i % 17 + 1) as u64;
            total += value;
            metric.push_block(StoredU64::from(value));
            total
        })
        .collect();
    metric.cumulative.height.write().unwrap();

    let ends: Vec<_> = days
        .iter()
        .enumerate()
        .map(|(day, _)| days.get(day + 1).copied().unwrap_or(N) - 1)
        .collect();
    CACHE_BUDGET.invalidate();
    diagnostics::take();
    for (slot, start) in starts.as_array().into_iter().enumerate() {
        let mapping = start.snapshot();
        let expected: Vec<_> = ends
            .iter()
            .map(|&end| {
                let start = mapping[end].to_usize();
                Some(StoredU64::from(
                    truth[end] - start.checked_sub(1).map_or(0, |i| truth[i]),
                ))
            })
            .collect();
        assert_eq!(
            metric.sum.as_array()[slot].resolutions.day1.collect(),
            expected
        );
        assert_eq!(diagnostics::take(), if slot == 0 { N / 1024 } else { 0 });
        let averages: Vec<_> = ends
            .iter()
            .zip(&expected)
            .map(|(&end, sum)| {
                Some(StoredF32::from(
                    f64::from(sum.unwrap()) / (end - mapping[end].to_usize() + 1) as f64,
                ))
            })
            .collect();
        assert_eq!(
            metric.average.as_array()[slot].resolutions.day1.collect(),
            averages
        );
        assert_eq!(diagnostics::take(), 0);
    }
    assert_eq!(
        metric.cumulative.day1.collect(),
        ends.iter()
            .map(|&i| Some(StoredU64::from(truth[i])))
            .collect::<Vec<_>>()
    );
    assert_eq!(diagnostics::take(), 0);
    // The block view uses the same cache too, including across page boundaries.
    assert_eq!(
        metric.block.collect_range_at(1020, 1030),
        (1020..1030)
            .map(|i| StoredU64::from((i % 17 + 1) as u64))
            .collect::<Vec<_>>()
    );
    assert_eq!(diagnostics::take(), 0);

    // A same-length replacement uses the existing publication-time invalidation.
    CACHE_BUDGET.invalidate();
    metric
        .cumulative
        .height
        .truncate_if_needed(Height::from(N - 1))
        .unwrap();
    metric
        .cumulative
        .height
        .push(StoredU64::from(truth[N - 1] + 123));
    metric.cumulative.height.write().unwrap();
    diagnostics::take(); // Exclude page reads needed to rewrite the compressed tail.
    let values = metric.sum._24h.resolutions.day1.collect();
    let start = starts._24h.snapshot()[N - 1].to_usize();
    assert_eq!(
        values.last().copied().flatten(),
        Some(StoredU64::from(truth[N - 1] + 123 - truth[start - 1]))
    );
    assert_eq!(diagnostics::take(), N / 1024);
    assert_eq!(
        metric.cumulative.day1.collect().last().copied().flatten(),
        Some(StoredU64::from(truth[N - 1] + 123))
    );
    assert_eq!(diagnostics::take(), 0);

    // The lazy cumulative constructor must share its root between its height
    // transform, cumulative resolutions, and rolling views as well.
    let lazy = LazyPerBlockCumulativeRolling::from_cumulative_source(
        "lazy",
        Version::ONE,
        &metric.cumulative.height,
        &starts_ref,
        &indexes,
    );
    CACHE_BUDGET.invalidate();
    diagnostics::take();
    assert_eq!(lazy.sum._24h.resolutions.day1.collect(), values);
    assert_eq!(diagnostics::take(), N / 1024);
    lazy.average._1w.resolutions.day1.collect();
    lazy.cumulative.resolutions.day1.collect();
    lazy.block.collect_range_at(1020, 1030);
    assert_eq!(diagnostics::take(), 0);
}
