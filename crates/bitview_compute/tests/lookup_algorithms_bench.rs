#[allow(dead_code)]
mod common;

use std::{fmt::Debug, hint::black_box, sync::Arc, time::Instant};

use bitview_compute::{
    CachedBlockCountReader, DailyView, LastDay, LazyLookbackVec, LazyRollingRatioVec,
    LazyRollingRatioWithCachedBlockCount, LazyWindowVec, RepeatDay,
};
use brk_types::{Day1, Height, StoredU16, StoredU64};
use tempfile::tempdir;
use vecdb::{
    BinaryTransform, CachedVec, Database, LazyAggVec, LazyVec, READ_CHUNK_SIZE, ReadOnlyClone,
    ReadableBoxedVec, ReadableCloneableVec, ReadableVec, UnaryTransform, Version,
};

struct Ratio;
impl BinaryTransform<StoredU64, StoredU64, StoredU64> for Ratio {
    fn apply(a: StoredU64, b: StoredU64) -> StoredU64 {
        StoredU64::from(u64::from(a) / u64::from(b).max(1))
    }
}
struct Twice;
impl UnaryTransform<Option<StoredU64>> for Twice {
    fn apply(value: Option<StoredU64>) -> Option<StoredU64> {
        value.map(|v| StoredU64::from(u64::from(v) * 2))
    }
}

fn measure<T: Debug + PartialEq>(name: &str, expected: &[T], mut read: impl FnMut() -> Vec<T>) {
    let mut samples = Vec::new();
    for round in 0..12 {
        let start = Instant::now();
        let actual = read();
        let elapsed = start.elapsed();
        assert_eq!(actual, expected, "{name}");
        black_box(actual);
        if round > 0 {
            samples.push(elapsed);
        }
    }
    samples.sort();
    eprintln!(
        "{name}: median={:?} min={:?} max={:?}",
        samples[5], samples[0], samples[10]
    );
}

fn requests(len: usize) -> Vec<(&'static str, Vec<usize>)> {
    vec![
        ("one", vec![len - 3]),
        ("clustered", (len - 64..len - 32).collect()),
        ("spread", (0..32).map(|i| i * (len - 1) / 31).collect()),
        ("dense", (len - len.min(4096)..len).collect()),
    ]
}

#[test]
#[ignore = "Before/after lookup and aggregation benchmark; validates all outputs"]
fn benchmark_lookup_algorithms() {
    const N: usize = 262_144;
    const DAYS: usize = 28_672;
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let source = common::stored::<Height, _>(
        &db,
        "source",
        (0..N).map(|i| StoredU64::from((i as u64 + 1) * 3)),
    );
    let daily = common::stored::<Day1, _>(
        &db,
        "daily",
        (0..DAYS).map(|i| StoredU64::from((i as u64 + 1) * 3)),
    );
    let repeat_mapping =
        common::stored::<Height, _>(&db, "repeat_mapping", (0..N).map(|i| Day1::from(i / 144)));
    let last_mapping = common::stored::<Height, _>(
        &db,
        "last_mapping",
        (0..4096usize).map(|i| Day1::from(i * 7)),
    );
    let starts = CachedVec::wrap(common::stored::<Height, _>(
        &db,
        "starts",
        (0..N).map(|i| Height::from(i.saturating_sub(2016))),
    ));
    let cached = CachedVec::wrap(common::stored::<Height, _>(
        &db,
        "cached",
        (0..N).map(|i| StoredU64::from(i as u64 + 1)),
    ));
    let counts = CachedVec::wrap(common::stored::<Height, _>(
        &db,
        "counts",
        (0..N).map(|_| StoredU16::from(1u16)),
    ));
    starts.snapshot();
    cached.snapshot();
    counts.snapshot();
    for resident in [false, true] {
        let source: ReadableBoxedVec<Height, StoredU64> = if resident {
            let v = CachedVec::wrap(source.read_only_clone());
            v.snapshot();
            v.read_only_boxed_clone()
        } else {
            source.read_only_boxed_clone()
        };
        let daily: ReadableBoxedVec<Day1, StoredU64> = if resident {
            let v = CachedVec::wrap(daily.read_only_clone());
            v.snapshot();
            v.read_only_boxed_clone()
        } else {
            daily.read_only_boxed_clone()
        };
        let repeat_mapping = if resident {
            let v = CachedVec::wrap(repeat_mapping.read_only_clone());
            v.snapshot();
            v.read_only_boxed_clone()
        } else {
            repeat_mapping.read_only_boxed_clone()
        };
        let last_mapping = if resident {
            let v = CachedVec::wrap(last_mapping.read_only_clone());
            v.snapshot();
            v.read_only_boxed_clone()
        } else {
            last_mapping.read_only_boxed_clone()
        };
        let repeat = DailyView::<Height, StoredU64, RepeatDay>::new(
            "repeat",
            Version::ONE,
            daily.clone(),
            &repeat_mapping,
        );
        let last = DailyView::<Height, StoredU64, LastDay>::new(
            "last",
            Version::ONE,
            daily,
            &last_mapping,
        );
        let mapping = Arc::new(
            (0..16_384usize)
                .map(|i| Height::from(i * 16))
                .collect::<Vec<_>>(),
        );
        let agg = LazyAggVec::<Height, Option<StoredU64>, Height, Height, StoredU64>::new(
            "agg",
            Version::ONE,
            Version::ONE,
            source.clone(),
            move || mapping.clone(),
        );
        for (name, view, len, expected) in [
            (
                "repeat",
                repeat.read_only_boxed_clone(),
                N,
                (0..N)
                    .map(|i| Some(StoredU64::from((i as u64 / 144 + 1) * 3)))
                    .collect::<Vec<_>>(),
            ),
            (
                "last",
                last.read_only_boxed_clone(),
                4096,
                (0..4096u64)
                    .map(|i| Some(StoredU64::from((i + 1) * 7 * 3)))
                    .collect(),
            ),
            (
                "agg",
                agg.read_only_boxed_clone(),
                16_384,
                (0..16_384u64)
                    .map(|i| Some(StoredU64::from((i + 1) * 16 * 3)))
                    .collect(),
            ),
        ] {
            for (pattern, indices) in requests(len) {
                let expected: Vec<_> = indices.iter().map(|&i| expected[i]).collect();
                measure(
                    &format!("{name}/cached={resident}/{pattern}"),
                    &expected,
                    || view.read_sorted_at(black_box(&indices)),
                );
            }
            if name == "agg" {
                let nested =
                    LazyVec::<Height, Option<StoredU64>, Height, Option<StoredU64>>::transformed::<
                        Twice,
                    >("nested", Version::ONE, view.clone());
                measure(
                    &format!("agg/cached={resident}/full_direct"),
                    &expected,
                    || view.collect_range_at(black_box(0), black_box(len)),
                );
                let expected: Vec<_> = expected.into_iter().map(Twice::apply).collect();
                measure(
                    &format!("agg/cached={resident}/full_nested"),
                    &expected,
                    || nested.collect_range_at(black_box(0), black_box(len)),
                );
                measure(
                    &format!("agg/cached={resident}/experimental_single_fold_nested"),
                    &expected,
                    || {
                        let mut out = Vec::with_capacity(len);
                        let mut values = Vec::with_capacity(READ_CHUNK_SIZE);
                        agg.fold_range_at(black_box(0), black_box(len), (), |(), value| {
                            values.push(value);
                            if values.len() == READ_CHUNK_SIZE {
                                out.extend(values.iter().copied().map(Twice::apply));
                                values.clear();
                            }
                        });
                        out.extend(values.iter().copied().map(Twice::apply));
                        out
                    },
                );
                measure(
                    &format!("agg/cached={resident}/legacy_nested"),
                    &expected,
                    || {
                        let mut out = Vec::with_capacity(len);
                        let mut values = Vec::with_capacity(READ_CHUNK_SIZE);
                        for at in (0..len).step_by(READ_CHUNK_SIZE) {
                            values.clear();
                            view.read_into_at(
                                black_box(at),
                                (at + READ_CHUNK_SIZE).min(len),
                                &mut values,
                            );
                            out.extend(values.iter().copied().map(Twice::apply));
                        }
                        out
                    },
                );
            }
        }
        let denominator = CachedBlockCountReader::new(counts.read_only_cached_boxed_clone());
        let lookback = LazyLookbackVec::new(
            "lookback",
            Version::ONE,
            source.clone(),
            2016,
            |a: StoredU64, b: Option<StoredU64>| {
                StoredU64::from(u64::from(a) - u64::from(b.unwrap_or_default()))
            },
        );
        let window = LazyWindowVec::new(
            "window",
            Version::ONE,
            source.clone(),
            starts.read_only_cached_boxed_clone(),
            true,
            |a: StoredU64, b: StoredU64, _| StoredU64::from(u64::from(a) - u64::from(b)),
        );
        let ratio = LazyRollingRatioVec::<StoredU64, StoredU64, StoredU64, Ratio>::new(
            "ratio",
            Version::ONE,
            source.clone(),
            cached.read_only_cached_boxed_clone(),
            starts.read_only_cached_boxed_clone(),
        );
        let rolling = LazyRollingRatioWithCachedBlockCount::<StoredU64, Ratio>::new(
            "rolling",
            Version::ONE,
            source.clone(),
            denominator,
            starts.read_only_cached_boxed_clone(),
        );
        for (name, view) in [
            ("source", source),
            ("lookback", lookback.read_only_boxed_clone()),
            ("window", window.read_only_boxed_clone()),
            ("ratio", ratio.read_only_boxed_clone()),
            ("rolling_block", rolling.read_only_boxed_clone()),
        ] {
            for (pattern, indices) in requests(N) {
                let expected: Vec<_> = indices
                    .iter()
                    .map(|&i| {
                        StoredU64::from(match name {
                            "source" => (i as u64 + 1) * 3,
                            "lookback" => (i + 1).min(2016) as u64 * 3,
                            "window" => (i + 1).min(2017) as u64 * 3,
                            _ => 3,
                        })
                    })
                    .collect();
                measure(
                    &format!("{name}/cached={resident}/{pattern}"),
                    &expected,
                    || view.read_sorted_at(black_box(&indices)),
                );
            }
        }
    }
}
