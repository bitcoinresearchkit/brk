#[allow(dead_code)]
mod common;

use std::{hint::black_box, time::Instant};

use bitview_compute::{DailyView, LastDay, RepeatDay};
use brk_types::{Day1, Height, StoredU64};
use tempfile::tempdir;
use vecdb::{
    AnyVec, CachedVec, Cursor, Database, ReadOnlyClone, ReadableBoxedVec, ReadableCloneableVec,
    ReadableVec, VecIndex, Version,
};

// Retained pre-change repeat algorithm for same-process comparisons when the
// machine's load changes between the before and after runs.
fn baseline_repeated(
    source: &ReadableBoxedVec<Day1, StoredU64>,
    mapping: &ReadableBoxedVec<Height, Day1>,
    from: usize,
    to: usize,
) -> Vec<Option<StoredU64>> {
    let source_len = source.visible_len();
    let mut mapping = Cursor::new(&**mapping);
    let end = mapping
        .get(to - 1)
        .map(|day| day.to_usize().saturating_add(1));
    let start = mapping
        .get(from)
        .map(|day| day.to_usize())
        .unwrap_or(source_len)
        .min(source_len);
    let end = end.unwrap_or(start).min(source_len);
    let values = if start < end {
        source.collect_range_dyn(start, end)
    } else {
        Vec::new()
    };
    let mut out = Vec::with_capacity(to - from);
    mapping.advance(from);
    mapping.fold(to - from, (), |(), day| {
        out.push(
            day.to_usize()
                .checked_sub(start)
                .and_then(|i| values.get(i))
                .copied(),
        );
    });
    out
}

fn repeated(
    source: &ReadableBoxedVec<Day1, StoredU64>,
    mapping: &ReadableBoxedVec<Height, Day1>,
    from: usize,
    to: usize,
    mode: usize,
) -> Vec<Option<StoredU64>> {
    let mut out = Vec::with_capacity(to - from);
    let collected = (mode >= 4).then(|| mapping.collect_range_dyn(from, to));
    let start = collected
        .as_ref()
        .and_then(|days| days.first().copied())
        .or_else(|| mapping.collect_one_at(from))
        .unwrap()
        .to_usize()
        .min(source.visible_len());
    let end = (collected
        .as_ref()
        .and_then(|days| days.last().copied())
        .or_else(|| mapping.collect_one_at(to - 1))
        .unwrap()
        .to_usize()
        + 1)
    .min(source.visible_len());
    let values = source.collect_range_dyn(start, end);
    let lookup = |day: Day1| {
        day.to_usize()
            .checked_sub(start)
            .and_then(|i| values.get(i))
            .copied()
    };
    let mut append = |days: &[Day1]| {
        if mode == 3 || mode == 5 {
            let mut offset = 0;
            while offset < days.len() {
                let day = days[offset];
                let count = days[offset..].partition_point(|value| *value == day);
                out.resize(out.len() + count, lookup(day));
                offset += count;
            }
        } else {
            out.extend(days.iter().copied().map(lookup));
        }
    };
    if let Some(days) = collected {
        append(&days);
    } else if mode == 1 {
        append(&mapping.collect_range_dyn(from, to));
    } else {
        mapping.for_each_chunk_at(from, to, &mut |_, days| append(days));
    }
    out
}

fn sparse(
    source: &ReadableBoxedVec<Day1, StoredU64>,
    mapping: &ReadableBoxedVec<Height, Day1>,
    from: usize,
    to: usize,
) -> Vec<Option<StoredU64>> {
    let days = mapping.collect_range_dyn(from, (to + 1).min(mapping.len()));
    let end = source.visible_len();
    let indices: Vec<_> = (0..to - from)
        .map(|i| {
            let first = days[i].to_usize();
            let next = days
                .get(i + 1)
                .map(|day| day.to_usize())
                .unwrap_or(end)
                .min(end);
            (first < next).then(|| next - 1)
        })
        .collect();
    let requested: Vec<_> = indices.iter().flatten().copied().collect();
    let values = source.read_sorted_at(&requested);
    let mut values = values.into_iter();
    indices
        .into_iter()
        .map(|index| index.map(|_| values.next().unwrap()))
        .collect()
}

#[test]
#[ignore = "DailyView baseline and alternative algorithms, with output validation"]
fn benchmark_daily_view() {
    const N: usize = 262_144;
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let source = common::stored::<Day1, _>(
        &db,
        "daily",
        (0..N / 144 - 1).map(|i| StoredU64::from(i as u64 * 7)),
    );
    let repeated_mapping =
        common::stored::<Height, _>(&db, "repeated", (0..N).map(|i| Day1::from(i / 144)));
    let sparse_mapping =
        common::stored::<Height, _>(&db, "sparse", (0..512usize).map(|i| Day1::from(i * 7)));
    for cached in [false, true] {
        let source: ReadableBoxedVec<Day1, StoredU64> = if cached {
            let v = CachedVec::wrap(source.read_only_clone());
            v.snapshot();
            v.read_only_boxed_clone()
        } else {
            source.read_only_boxed_clone()
        };
        let mapping: ReadableBoxedVec<Height, Day1> = if cached {
            let v = CachedVec::wrap(repeated_mapping.read_only_clone());
            v.snapshot();
            v.read_only_boxed_clone()
        } else {
            repeated_mapping.read_only_boxed_clone()
        };
        let view = DailyView::<Height, StoredU64, RepeatDay>::new(
            "repeat",
            Version::ONE,
            source.clone(),
            mapping.clone(),
        );
        for from in [N - 1024, 0] {
            let expected: Vec<_> = (from..N)
                .map(|i| (i / 144 < source.len()).then(|| StoredU64::from((i / 144) as u64 * 7)))
                .collect();
            let mut times = vec![Vec::new(); 7];
            for round in 0..12 {
                for offset in 0..7 {
                    let mode = (round + offset) % 7;
                    let start = Instant::now();
                    let actual = if mode == 0 {
                        view.collect_range_at(black_box(from), black_box(N))
                    } else if mode == 6 {
                        baseline_repeated(&source, &mapping, black_box(from), black_box(N))
                    } else {
                        repeated(&source, &mapping, black_box(from), black_box(N), mode)
                    };
                    let elapsed = start.elapsed();
                    assert_eq!(actual, expected);
                    black_box(actual);
                    if round > 0 {
                        times[mode].push(elapsed);
                    }
                }
            }
            for (mode, samples) in [
                "view",
                "collected",
                "chunks",
                "runs",
                "one_read",
                "one_read_runs",
                "baseline_cursor",
            ]
            .into_iter()
            .zip(&mut times)
            {
                samples.sort();
                eprintln!(
                    "repeat/cached={cached}/from={from}/{mode}: median={:?} min={:?} max={:?}",
                    samples[5], samples[0], samples[10]
                );
            }
        }
        let mapping: ReadableBoxedVec<Height, Day1> = if cached {
            let v = CachedVec::wrap(sparse_mapping.read_only_clone());
            v.snapshot();
            v.read_only_boxed_clone()
        } else {
            sparse_mapping.read_only_boxed_clone()
        };
        let view = DailyView::<Height, StoredU64, LastDay>::new(
            "last",
            Version::ONE,
            source.clone(),
            mapping.clone(),
        );
        for from in [240, 0] {
            let to = if from == 0 { 512 } else { 260 };
            let expected: Vec<_> = (from..to)
                .map(|i| {
                    let end = ((i + 1) * 7).min(source.len());
                    (i * 7 < end).then(|| StoredU64::from((end - 1) as u64 * 7))
                })
                .collect();
            let mut times = vec![Vec::new(); 2];
            for round in 0..12 {
                for offset in 0..2 {
                    let mode = (round + offset) % 2;
                    let start = Instant::now();
                    let actual = if mode == 0 {
                        view.collect_range_at(black_box(from), black_box(to))
                    } else {
                        sparse(&source, &mapping, black_box(from), black_box(to))
                    };
                    let elapsed = start.elapsed();
                    assert_eq!(actual, expected);
                    black_box(actual);
                    if round > 0 {
                        times[mode].push(elapsed);
                    }
                }
            }
            for (mode, samples) in ["view", "direct_indices"].into_iter().zip(&mut times) {
                samples.sort();
                eprintln!(
                    "last/cached={cached}/from={from}/{mode}: median={:?} min={:?} max={:?}",
                    samples[5], samples[0], samples[10]
                );
            }
        }
    }
}

#[test]
#[ignore = "Check whether run filling generalizes to less repetitive mappings"]
fn benchmark_repeat_density() {
    const N: usize = 32_768;
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let source = CachedVec::wrap(common::stored::<Day1, _>(
        &db,
        "density_source",
        (0..N).map(|i| StoredU64::from(i as u64 * 7)),
    ));
    source.snapshot();
    let source = source.read_only_boxed_clone();
    for repeat in [1, 6, 144] {
        let mapping = CachedVec::wrap(common::stored::<Height, _>(
            &db,
            &format!("density_{repeat}"),
            (0..N).map(|i| Day1::from(i / repeat)),
        ));
        mapping.snapshot();
        let mapping = mapping.read_only_boxed_clone();
        let expected: Vec<_> = (0..N)
            .map(|i| Some(StoredU64::from((i / repeat) as u64 * 7)))
            .collect();
        let mut times = vec![Vec::new(); 2];
        for round in 0..20 {
            for offset in 0..2 {
                let mode = (round + offset) % 2;
                let start = Instant::now();
                let actual = repeated(&source, &mapping, black_box(0), black_box(N), mode + 4);
                let elapsed = start.elapsed();
                assert_eq!(actual, expected);
                black_box(actual);
                if round > 0 {
                    times[mode].push(elapsed);
                }
            }
        }
        for (mode, samples) in ["one_read", "one_read_runs"].into_iter().zip(&mut times) {
            samples.sort();
            eprintln!("density/repeat={repeat}/{mode}: median={:?}", samples[9]);
        }
    }
}
