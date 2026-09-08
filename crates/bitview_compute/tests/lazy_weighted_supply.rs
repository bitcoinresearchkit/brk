mod common;

use std::{hint::black_box, time::Instant};

use bitview_cohort::{AgeRange, AgeRangeId};
use bitview_compute::{
    CACHE_BUDGET, ColumnarPerBlock, LazyIndexedVec, LazySpotValuePerBlock, WeightedCohortState,
};
use brk_types::{BoundedRatio, Cents, Day1, Height, Sats, Version};
use tempfile::tempdir;
use vecdb::{
    AnySerializableVec, AnyStoredVec, CachedColumnarVec, CachedReadableVec, CachedVec, Database,
    ImportableVec, PcoVec, ReadOnlyClone, ReadableCloneableVec, ReadableColumnarVec, ReadableVec,
    WritableVec,
};

#[test]
fn lazy_sides_preserve_stored_rounding_and_follow_source_rewrites() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut indexes = common::indexes(&db);
    indexes.cached_first_height.day1 = CachedVec::wrap(common::stored::<Day1, _>(
        &db,
        "daily_first_height",
        [0usize, 2, 4].map(Height::from),
    ))
    .read_only_cached_boxed_clone();
    let mut supply = ColumnarPerBlock::<Sats, AgeRangeId, ()>::forced_import(
        &db,
        "supply",
        Version::ONE,
        |_| (),
    )
    .unwrap();
    let mut weights = ColumnarPerBlock::<BoundedRatio, AgeRangeId, ()>::forced_import(
        &db,
        "weights",
        Version::ONE,
        |_| (),
    )
    .unwrap();
    let mut spot = PcoVec::<Height, Cents>::forced_import(&db, "spot", Version::ONE).unwrap();
    let inputs = [
        (Sats::from(123_456_789_u64), BoundedRatio::from(0.321)),
        (Sats::from(1_u64), BoundedRatio::from(0.5)),
        (Sats::from(99_u64), BoundedRatio::ONE),
        (Sats::from(99_u64), BoundedRatio::ZERO),
        (Sats::from(99_u64), BoundedRatio::NAN),
    ];
    for (s, w) in inputs {
        supply.push(AgeRange::from_fn(|_| s));
        weights.push(AgeRange::from_fn(|_| w));
        spot.push(Cents::from(1_000_000_u64));
    }
    supply.write().unwrap();
    weights.write().unwrap();
    spot.write().unwrap();
    let supply_cache = CachedColumnarVec::new(
        supply.height.read_only_clone(),
        Version::ONE,
        CachedVec::wrap,
    );
    let weight_cache = CachedColumnarVec::new(
        weights.height.read_only_clone(),
        Version::ONE,
        CachedVec::wrap,
    );
    let spot = CachedVec::wrap(spot.read_only_clone()).cached_boxed_clone();
    for &id in AgeRangeId::ALL {
        let raw = supply_cache
            .column("supply", Version::ONE, id)
            .read_only_boxed_clone();
        let weight = weight_cache.cached_column(id).cached_boxed_clone();
        let weighted = LazySpotValuePerBlock::from_weighted_supply::<false>(
            "awake_supply",
            Version::ONE,
            &raw,
            &weight,
            &indexes,
            &spot,
        );
        let complement = LazySpotValuePerBlock::from_weighted_supply::<true>(
            "dormant_supply",
            Version::ONE,
            &raw,
            &weight,
            &indexes,
            &spot,
        );
        for (height, (s, w)) in inputs.iter().copied().enumerate() {
            let expected = WeightedCohortState::split_supply(s, w);
            assert_eq!(
                weighted.sats.height.collect_one_at(height),
                Some(expected.0)
            );
            assert_eq!(
                complement.sats.height.collect_one_at(height),
                Some(expected.1)
            );
        }
        // Both floors are zero: subtraction would incorrectly return one sat.
        assert_eq!(weighted.sats.height.collect_one_at(1), Some(Sats::ZERO));
        assert_eq!(complement.sats.height.collect_one_at(1), Some(Sats::ZERO));
        // Stock resolutions select the final block, including the partial day.
        for (day, height) in [1usize, 3, 4].into_iter().enumerate() {
            let expected = WeightedCohortState::split_supply(inputs[height].0, inputs[height].1);
            assert_eq!(
                weighted.sats.day1.collect_one_at(day),
                Some(Some(expected.0))
            );
            assert_eq!(
                complement.sats.day1.collect_one_at(day),
                Some(Some(expected.1))
            );
        }
        supply_cache.invalidate();
        weight_cache.invalidate();
        // Production invalidates the shared budget before rewrites, including
        // derived resolution caches, not only the raw age inputs.
        CACHE_BUDGET.invalidate();
        supply.truncate_if_needed_at(4).unwrap();
        weights.truncate_if_needed_at(4).unwrap();
        supply.push(AgeRange::from_fn(|_| Sats::from(101_u64)));
        weights.push(AgeRange::from_fn(|_| BoundedRatio::ONE));
        supply.write().unwrap();
        weights.write().unwrap();
        assert_eq!(
            weighted.sats.height.collect_one_at(4),
            Some(Sats::from(101_u64))
        );
        assert_eq!(complement.sats.height.collect_one_at(4), Some(Sats::ZERO));
        assert_eq!(
            weighted.sats.day1.collect_one_at(2),
            Some(Some(Sats::from(101_u64)))
        );
        assert_eq!(
            complement.sats.day1.collect_one_at(2),
            Some(Some(Sats::ZERO))
        );
        // Restore before validating the next age column.
        supply_cache.invalidate();
        weight_cache.invalidate();
        CACHE_BUDGET.invalidate();
        supply.truncate_if_needed_at(4).unwrap();
        weights.truncate_if_needed_at(4).unwrap();
        supply.push(AgeRange::from_fn(|_| inputs[4].0));
        weights.push(AgeRange::from_fn(|_| inputs[4].1));
        supply.write().unwrap();
        weights.write().unwrap();
    }
}

#[test]
#[ignore = "synthetic compressed-history benchmark; excludes indexing and HTTP"]
fn benchmark_shared_age_inputs() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let indexes = common::indexes(&db);
    let mut supply = ColumnarPerBlock::<Sats, AgeRangeId, ()>::forced_import(
        &db,
        "supply",
        Version::ONE,
        |_| (),
    )
    .unwrap();
    let mut weights = ColumnarPerBlock::<BoundedRatio, AgeRangeId, ()>::forced_import(
        &db,
        "weights",
        Version::ONE,
        |_| (),
    )
    .unwrap();
    let mut stored = ColumnarPerBlock::<Sats, AgeRangeId, ()>::forced_import(
        &db,
        "stored_weighted",
        Version::ONE,
        |_| (),
    )
    .unwrap();
    let rows = 131_072usize;
    let input = |height: usize, id: AgeRangeId| {
        let supply = Sats::from(
            10_000_000_000u64 + (height as u64 * 7919 + id.index() as u64 * 104729) % 9_000_000_000,
        );
        let weight = BoundedRatio::from(((height * 17 + id.index() * 31) % 10000) as f64 / 10000.0);
        (supply, weight)
    };
    for height in 0..rows {
        supply.push(AgeRange::from_fn(|id| input(height, id).0));
        weights.push(AgeRange::from_fn(|id| input(height, id).1));
        stored.push(AgeRange::from_fn(|id| {
            let (s, w) = input(height, id);
            WeightedCohortState::split_supply(s, w).0
        }));
    }
    supply.write().unwrap();
    weights.write().unwrap();
    stored.write().unwrap();
    let supplies = CachedColumnarVec::new(supply.height.read_only_clone(), Version::ONE, |v| {
        CACHE_BUDGET.wrap(v)
    });
    let weights = CachedColumnarVec::new(weights.height.read_only_clone(), Version::ONE, |v| {
        CACHE_BUDGET.wrap(v)
    });
    let spot = CachedVec::wrap(common::stored::<Height, Cents>(&db, "spot", []))
        .read_only_cached_boxed_clone();
    let lazy: Vec<_> = AgeRangeId::ALL
        .iter()
        .map(|&id| {
            LazySpotValuePerBlock::from_weighted_supply::<false>(
                "weighted",
                Version::ONE,
                &supplies.cached_column(id).read_only_boxed_clone(),
                &weights.cached_column(id).cached_boxed_clone(),
                &indexes,
                &spot,
            )
        })
        .collect();
    let mut timings = Vec::new();
    for round in 0..7 {
        CACHE_BUDGET.invalidate();
        let start = Instant::now();
        let reference: Vec<_> = AgeRangeId::ALL
            .iter()
            .map(|&id| {
                stored
                    .height
                    .read_only_clone()
                    .column("reference", Version::ONE, id)
                    .collect_range_at(0, rows)
            })
            .collect();
        let stored_time = start.elapsed();
        let start = Instant::now();
        let cold: Vec<_> = lazy
            .iter()
            .map(|v| v.sats.height.collect_range_at(0, rows))
            .collect();
        let cold_time = start.elapsed();
        let start = Instant::now();
        let warm: Vec<_> = lazy
            .iter()
            .map(|v| v.sats.height.collect_range_at(0, rows))
            .collect();
        let warm_time = start.elapsed();
        assert_eq!(cold, reference);
        assert_eq!(warm, reference);
        black_box(&warm);
        if round > 0 {
            timings.push((stored_time, cold_time, warm_time));
        }
    }
    let median = |column: usize| {
        let mut values: Vec<_> = timings.iter().map(|&(s, c, w)| [s, c, w][column]).collect();
        values.sort();
        values[values.len() / 2]
    };
    eprintln!(
        "synthetic {rows} rows x {} ages: stored={:?}, application-cache-cold lazy={:?}, warm lazy={:?}; all points equal",
        AgeRangeId::ALL.len(),
        median(0),
        median(1),
        median(2)
    );

    // Diagnostic ablations only: production code is unchanged. Keep identical
    // inputs and output values, rotate order, and exclude equality checks.
    let snapshots: Vec<_> = AgeRangeId::ALL
        .iter()
        .map(|&id| {
            (
                supplies
                    .cached_column(id)
                    .cached_snapshot()
                    .expect("supply cache is warm"),
                weights
                    .cached_column(id)
                    .cached_snapshot()
                    .expect("weight cache is warm"),
            )
        })
        .collect();
    let indexed: Vec<_> = AgeRangeId::ALL
        .iter()
        .map(|&id| {
            LazyIndexedVec::new(
                "diagnostic",
                Version::ONE,
                supplies.cached_column(id),
                weights.cached_column(id),
                |_: Height, s, w| WeightedCohortState::split_supply(s, w).0,
            )
        })
        .collect();
    let expected: Vec<_> = lazy
        .iter()
        .map(|v| v.sats.height.collect_range_at(0, rows))
        .collect();
    let compute = |s, w| WeightedCohortState::split_supply(s, w).0;
    let dynamic: &dyn Fn(Sats, BoundedRatio) -> Sats = black_box(&compute);
    let labels = [
        "production",
        "indexed_only",
        "slices_static",
        "slices_dynamic",
        "copied_static",
        "slices_reused_output",
        "copy_precomputed_output",
    ];
    let mut samples = vec![Vec::new(); labels.len()];
    let mut reusable: Vec<Vec<Sats>> = (0..snapshots.len())
        .map(|_| Vec::with_capacity(rows))
        .collect();
    for round in 0..15 {
        for offset in 0..labels.len() {
            let case = (round + offset) % labels.len();
            let start = Instant::now();
            if case == 5 {
                for ((s, w), output) in snapshots.iter().zip(&mut reusable) {
                    output.clear();
                    output.extend(s.iter().zip(w.iter()).map(|(&s, &w)| compute(s, w)));
                }
                let elapsed = start.elapsed();
                black_box(&reusable);
                assert_eq!(reusable, expected);
                if round >= 3 {
                    samples[case].push(elapsed);
                }
            } else {
                let result: Vec<Vec<Sats>> = match case {
                    0 => lazy
                        .iter()
                        .map(|v| v.sats.height.collect_range_at(0, rows))
                        .collect(),
                    1 => indexed
                        .iter()
                        .map(|v| v.collect_range_at(0, rows))
                        .collect(),
                    2 => snapshots
                        .iter()
                        .map(|(s, w)| {
                            s.iter()
                                .zip(w.iter())
                                .map(|(&s, &w)| compute(s, w))
                                .collect()
                        })
                        .collect(),
                    3 => snapshots
                        .iter()
                        .map(|(s, w)| {
                            s.iter()
                                .zip(w.iter())
                                .map(|(&s, &w)| dynamic(s, w))
                                .collect()
                        })
                        .collect(),
                    4 => snapshots
                        .iter()
                        .map(|(s, w)| {
                            let copied = s.as_ref().clone();
                            black_box(copied.as_slice())
                                .iter()
                                .zip(w.iter())
                                .map(|(&s, &w)| compute(s, w))
                                .collect()
                        })
                        .collect(),
                    6 => black_box(&expected).clone(),
                    _ => unreachable!(),
                };
                let elapsed = start.elapsed();
                black_box(&result);
                assert_eq!(result, expected);
                if round >= 3 {
                    samples[case].push(elapsed);
                }
            }
        }
    }
    for (label, samples) in labels.iter().zip(&mut samples) {
        samples.sort();
        eprintln!(
            "overhead {label}: median={:?}, min={:?}, max={:?}",
            samples[samples.len() / 2],
            samples[0],
            samples.last().unwrap()
        );
    }

    // Exercise the export boundary too, not just collect_range_at.
    let mut stored_json_times = Vec::new();
    let mut lazy_json_times = Vec::new();
    for round in 0..7 {
        let stored_json = || {
            AgeRangeId::ALL
                .iter()
                .map(|&id| {
                    let mut json = Vec::new();
                    stored
                        .height
                        .read_only_clone()
                        .column("reference", Version::ONE, id)
                        .write_json(Some(0), Some(rows), &mut json)
                        .unwrap();
                    json
                })
                .collect::<Vec<_>>()
        };
        let lazy_json = || {
            lazy.iter()
                .map(|v| {
                    let mut json = Vec::new();
                    v.sats
                        .height
                        .write_json(Some(0), Some(rows), &mut json)
                        .unwrap();
                    json
                })
                .collect::<Vec<_>>()
        };
        let (reference, actual, stored_time, lazy_time) = if round % 2 == 0 {
            let start = Instant::now();
            let reference = stored_json();
            let stored_time = start.elapsed();
            let start = Instant::now();
            let actual = lazy_json();
            (reference, actual, stored_time, start.elapsed())
        } else {
            let start = Instant::now();
            let actual = lazy_json();
            let lazy_time = start.elapsed();
            let start = Instant::now();
            let reference = stored_json();
            (reference, actual, start.elapsed(), lazy_time)
        };
        assert_eq!(actual, reference);
        black_box(&actual);
        if round > 0 {
            stored_json_times.push(stored_time);
            lazy_json_times.push(lazy_time);
        }
    }
    stored_json_times.sort();
    lazy_json_times.sort();
    eprintln!(
        "JSON export: stored={:?}, warm lazy={:?}; all bytes equal",
        stored_json_times[3], lazy_json_times[3]
    );
}
