use std::{hint::black_box, time::Instant};

use bitview_vecs::LazyIndexedVec;
use tempfile::tempdir;
use vecdb::ReverseOperands;

use super::*;

fn compare<T: VecValue>(
    name: &str,
    source: &impl ReadableVec<Height, T>,
    value: impl Fn(T) -> u64 + Copy,
) {
    let len = source.len();
    for (case, from, to, stop, fallible) in [
        ("full", 0, len, None, true),
        ("tail16", len - 16, len, None, true),
        ("early16", 0, len, Some(16), true),
        ("full_fold", 0, len, None, false),
        ("tail16_fold", len - 16, len, None, false),
    ] {
        let run = |buffered| {
            if !fallible {
                let fold = |sum: u64, item| sum.wrapping_add(value(item));
                return Ok(if buffered {
                    let mut values = Vec::with_capacity(to - from);
                    source.read_into_at(from, to, &mut values);
                    values.into_iter().fold(0, fold)
                } else {
                    source.fold_range_at(from, to, 0, fold)
                });
            }
            let mut seen = 0;
            let fold = |sum: u64, item| {
                seen += 1;
                let sum = sum.wrapping_add(value(item));
                if Some(seen) == stop {
                    Err(sum)
                } else {
                    Ok(sum)
                }
            };
            if buffered {
                // The original implementation materialized the entire result before folding.
                let mut values = Vec::with_capacity(to - from);
                source.read_into_at(from, to, &mut values);
                values.into_iter().try_fold(0, fold)
            } else {
                source.try_fold_range_at(from, to, 0, fold)
            }
        };
        assert_eq!(run(true), run(false));
        let measure = |buffered| {
            let start = Instant::now();
            for _ in 0..40 {
                let _ = black_box(run(buffered));
            }
            start.elapsed().as_nanos() / 40
        };
        let mut before = Vec::new();
        let mut after = Vec::new();
        for round in 0..13 {
            let (a, b) = if round % 2 == 0 {
                (measure(true), measure(false))
            } else {
                let b = measure(false);
                (measure(true), b)
            };
            if round > 1 {
                before.push(a);
                after.push(b);
            }
        }
        before.sort_unstable();
        after.sort_unstable();
        eprintln!(
            "fold {name} {case}: buffered={}ns direct={}ns ratio={:.3}",
            before[5],
            after[5],
            after[5] as f64 / before[5] as f64
        );
    }
}

#[test]
#[ignore = "same-input buffered versus direct folds; excludes import and server transport"]
fn benchmark_lazy_folds() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let len = 32_768_usize;
    let source = stored(
        &db,
        "source",
        (0..len as u64).map(|i| StoredU64::from(i * (i + 1))),
    );
    let starts = CachedVec::wrap(stored(
        &db,
        "starts",
        (0..len).map(|i| Height::from(i.saturating_sub(256))),
    ));
    let days = CachedVec::wrap(stored(&db, "days", (0..len).map(|i| Day1::from(i / 144))));
    let counts = CachedVec::wrap(stored(&db, "counts", (0..len).map(|_| StoredU16::new(100))));
    let counts = CumulativeCountVec::new(&counts);
    let cumulative = CachedVec::wrap(stored(
        &db,
        "cumulative",
        (0..len as u64).map(|i| StoredU64::from((i + 1) * 100)),
    ));
    let first = stored(&db, "first", (0..len).map(|i| Height::from(i / 2)));
    let count = LazyIndexCountVec::new("count", Version::ONE, &first, &source);
    let delta = LazyPreviousDeltaVec::new("delta", Version::ONE, &source);
    let lookback = LazyLookbackVec::new(
        "lookback",
        Version::ONE,
        &source,
        256,
        |current, previous| current - previous.unwrap_or_default(),
    );
    let since = LazySinceDayVec::new(
        "since",
        Version::ONE,
        &source,
        &days,
        Day1::from(2),
        |current, previous| current - previous,
    );
    let window = LazyWindowVec::new(
        "window",
        Version::ONE,
        &source,
        &starts,
        true,
        |current, previous, _| current - previous,
    );
    let ratio = LazyIndexedVec::new(
        "ratio",
        Version::ONE,
        &counts,
        &source,
        |_, count, numerator| RatioU64::<PartsPerMillion32>::apply(numerator, count),
    );
    let rolling = LazyRollingRatioVec::<
        StoredU64,
        StoredU64,
        PartsPerMillion32,
        ReverseOperands<RatioU64<PartsPerMillion32>>,
    >::new("rolling", Version::ONE, &counts, &source, &starts);
    let cached_rolling = LazyRollingRatioVec::<
        StoredU64,
        StoredU64,
        PartsPerMillion32,
        RatioU64<PartsPerMillion32>,
    >::new(
        "cached_rolling",
        Version::ONE,
        &source,
        &cumulative,
        &starts,
    );
    compare("delta", &delta, u64::from);
    compare("lookback", &lookback, u64::from);
    compare("since_day", &since, u64::from);
    compare("window", &window, u64::from);
    compare("index_count", &count, u64::from);
    let ppm = |value: PartsPerMillion32| u64::from(value.inner());
    compare("count_ratio", &ratio, ppm);
    compare("rolling_count_ratio", &rolling, ppm);
    compare("rolling_ratio", &cached_rolling, ppm);
}
