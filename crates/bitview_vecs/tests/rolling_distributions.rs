use std::sync::atomic::{AtomicUsize, Ordering};

use bitview_collections::DistributionStats;
use bitview_compute::{ComputeRollingMedianFromStarts, compute_rolling_distribution_from_starts};
use brk_exit::Exit;
use brk_types::{Height, StoredF64, Version};
use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, AnyVec, Database, EagerVec, ImportableVec, LazyVec, PcoVec, PcoVecValue,
    ReadableCloneableVec, ReadableVec, WritableVec,
};

fn stored<T: PcoVecValue>(
    db: &Database,
    name: &str,
    values: impl IntoIterator<Item = T>,
) -> EagerVec<PcoVec<Height, T>> {
    let mut output = EagerVec::forced_import(db, name, Version::ONE).unwrap();
    for value in values {
        output.push(value);
    }
    output.write().unwrap();
    output
}

fn output_refs<T: PcoVecValue>(
    values: &mut DistributionStats<EagerVec<PcoVec<Height, T>>>,
) -> DistributionStats<&mut EagerVec<PcoVec<Height, T>>> {
    DistributionStats {
        min: &mut values.min,
        max: &mut values.max,
        pct10: &mut values.pct10,
        pct25: &mut values.pct25,
        median: &mut values.median,
        pct75: &mut values.pct75,
        pct90: &mut values.pct90,
    }
}

#[test]
fn rolling_outputs_preserve_interpolation_cache_reuse_and_restarts() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let values: Vec<_> = (0..48)
        .map(|i| StoredF64::from(((i * 7) % 13) as f64))
        .collect();
    let source = stored(&db, "values", values.iter().copied());
    let mut cache = None;
    let mut cached_pointer = None;
    for width in [40usize, 6, 1, 0] {
        let starts = stored(
            &db,
            &format!("starts_{width}"),
            (0..53usize).map(|i| Height::from((i + 1).saturating_sub(width))),
        );
        let mut outputs = DistributionStats::try_from_fn(|suffix| {
            EagerVec::<PcoVec<Height, StoredF64>>::forced_import(
                &db,
                &format!("output_{width}_{suffix}"),
                Version::ONE,
            )
        })
        .unwrap();
        let expected: Vec<_> = (0..values.len())
            .map(|i| {
                let mut population: Vec<f64> = values[(i + 1).saturating_sub(width)..=i]
                    .iter()
                    .copied()
                    .map(f64::from)
                    .collect();
                if population.is_empty() {
                    return [StoredF64::from(0.0); 7];
                }
                population.sort_by(f64::total_cmp);
                [0.0, 1.0, 0.10, 0.25, 0.50, 0.75, 0.90].map(|p| {
                    let rank = p * (population.len() - 1) as f64;
                    let fraction = rank - rank.floor();
                    StoredF64::from(
                        population[rank.floor() as usize] * (1.0 - fraction)
                            + population[rank.ceil() as usize] * fraction,
                    )
                })
            })
            .collect();
        for phase in 0..5 {
            match phase {
                2 => {
                    outputs.pct25.truncate_if_needed_at(13).unwrap();
                }
                4 => {
                    outputs
                        .median
                        .validate_computed_version_or_reset(Version::ZERO)
                        .unwrap();
                }
                _ => {}
            }
            compute_rolling_distribution_from_starts(
                Height::from(if phase == 3 { 7usize } else { values.len() }),
                &starts,
                &source,
                output_refs(&mut outputs),
                &Exit::new(),
                &mut cache,
            )
            .unwrap();
            let pointer = cache.as_ref().unwrap().1.as_ptr();
            assert_eq!(
                *cached_pointer.get_or_insert(pointer),
                pointer,
                "covered cache was reallocated"
            );
            for (column, output) in [
                &outputs.min,
                &outputs.max,
                &outputs.pct10,
                &outputs.pct25,
                &outputs.median,
                &outputs.pct75,
                &outputs.pct90,
            ]
            .into_iter()
            .enumerate()
            {
                assert_eq!(
                    output.collect_range_at(0, output.len()),
                    expected.iter().map(|row| row[column]).collect::<Vec<_>>(),
                    "width={width} phase={phase} column={column}"
                );
            }
        }
    }
}

#[test]
fn median_matches_sorted_windows_through_resume_rewind_and_reset() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let population: Vec<_> = (0..53)
        .map(|i| StoredF64::from(((i * 17) % 23) as f64 - 10.0))
        .collect();
    let source = stored(&db, "source", population.iter().copied());
    let exit = Exit::new();

    for width in [0usize, 1, 2, 7, 40, 100] {
        let starts = stored(
            &db,
            &format!("starts_{width}"),
            (0..60usize).map(|i| Height::from((i + 1).saturating_sub(width))),
        );
        let mut output = EagerVec::<PcoVec<Height, StoredF64>>::forced_import(
            &db,
            &format!("median_{width}"),
            Version::ONE,
        )
        .unwrap();
        let expected: Vec<_> = (0..population.len())
            .map(|i| {
                let start = (i + 1).saturating_sub(width);
                let mut values: Vec<f64> = population[start..i + 1]
                    .iter()
                    .copied()
                    .map(f64::from)
                    .collect();
                values.sort_by(f64::total_cmp);
                let middle = values.len() / 2;
                StoredF64::from(if values.is_empty() {
                    0.0
                } else if values.len() % 2 == 1 {
                    values[middle]
                } else {
                    values[middle - 1] * 0.5 + values[middle] * 0.5
                })
            })
            .collect();
        for phase in 0..5 {
            if phase == 2 {
                output.truncate_if_needed_at(19).unwrap();
            } else if phase == 4 {
                output
                    .validate_computed_version_or_reset(Version::ZERO)
                    .unwrap();
            }
            output
                .compute_rolling_median_from_starts(
                    Height::from(if phase == 3 { 7usize } else { population.len() }),
                    &starts,
                    &source,
                    &exit,
                )
                .unwrap();
            assert_eq!(output.collect(), expected, "width={width}, phase={phase}");
        }
        drop(output);
        let mut output = EagerVec::<PcoVec<Height, StoredF64>>::forced_import(
            &db,
            &format!("median_{width}"),
            Version::ONE,
        )
        .unwrap();
        output
            .compute_rolling_median_from_starts(
                Height::from(population.len()),
                &starts,
                &source,
                &exit,
            )
            .unwrap();
        assert_eq!(output.collect(), expected);
    }
}

#[test]
fn completed_median_does_not_reread_or_sort_its_historical_window() {
    static READS: AtomicUsize = AtomicUsize::new(0);
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let source = stored(
        &db,
        "source",
        (0..100u32).map(|value| StoredF64::from(f64::from(value))),
    );
    let source = LazyVec::init(
        "counted_source",
        Version::ONE,
        source.read_only_boxed_clone(),
        |_: Height, value| {
            READS.fetch_add(1, Ordering::Relaxed);
            value
        },
    );
    let starts = stored(&db, "starts", [Height::ZERO; 100]);
    let mut output =
        EagerVec::<PcoVec<Height, StoredF64>>::forced_import(&db, "median", Version::ONE).unwrap();
    let exit = Exit::new();
    output
        .compute_rolling_median_from_starts(Height::ZERO, &starts, &source, &exit)
        .unwrap();
    assert!(READS.swap(0, Ordering::Relaxed) >= 100);
    output
        .compute_rolling_median_from_starts(Height::from(100usize), &starts, &source, &exit)
        .unwrap();
    assert_eq!(READS.load(Ordering::Relaxed), 0);
    assert_eq!(output.len(), 100);
}
