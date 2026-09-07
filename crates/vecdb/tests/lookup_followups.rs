use std::sync::Arc;

use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, BytesVec, Database, DeltaAvg, DeltaChange, DeltaRate, DeltaSub, ImportableVec,
    LazyDeltaVec, ReadableCloneableVec, ReadableVec, Version, WritableVec,
};

#[test]
fn delta_merge_preserves_operators_duplicates_nonmonotonic_starts_and_rewrites() {
    let dir = tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let mut source = BytesVec::<usize, u32>::import(&db, "source", Version::ONE).unwrap();
    for i in 0..5000 {
        source.push((i + 1) * 3);
    }
    source.write().unwrap();
    for rewrite in [false, true] {
        if rewrite {
            source.truncate_if_needed_at(4000).unwrap();
            for i in 4000..5000 {
                source.push((i + 1) * 5);
            }
            source.write().unwrap();
        }
        for monotonic in [true, false] {
            let starts = Arc::new(
                (0..4993usize)
                    .map(|i| {
                        if monotonic {
                            i.saturating_sub(300)
                        } else {
                            i % 311
                        }
                    })
                    .collect::<Vec<_>>(),
            );
            let sub = LazyDeltaVec::<usize, u32, u32, DeltaSub>::new(
                "sub",
                Version::ONE,
                source.read_only_boxed_clone(),
                Version::ONE,
                {
                    let starts = starts.clone();
                    move || starts.clone()
                },
            );
            let avg = LazyDeltaVec::<usize, u32, f64, DeltaAvg>::new(
                "avg",
                Version::ONE,
                source.read_only_boxed_clone(),
                Version::ONE,
                {
                    let starts = starts.clone();
                    move || starts.clone()
                },
            );
            let change = LazyDeltaVec::<usize, u32, f64, DeltaChange>::new(
                "change",
                Version::ONE,
                source.read_only_boxed_clone(),
                Version::ONE,
                {
                    let starts = starts.clone();
                    move || starts.clone()
                },
            );
            let rate = LazyDeltaVec::<usize, u32, f64, DeltaRate>::new(
                "rate",
                Version::ONE,
                source.read_only_boxed_clone(),
                Version::ONE,
                {
                    let starts = starts.clone();
                    move || starts.clone()
                },
            );
            let values = source.collect_range_at(0, 5000);
            for indices in [
                vec![],
                vec![
                    0,
                    0,
                    1,
                    255,
                    256,
                    2047,
                    2048,
                    4096,
                    4992,
                    4993,
                    5000,
                    usize::MAX,
                ],
                (0..5000).flat_map(|i| [i, i]).collect(),
            ] {
                let valid: Vec<_> = indices
                    .iter()
                    .copied()
                    .filter(|&i| i < starts.len())
                    .collect();
                let sums: Vec<_> = valid
                    .iter()
                    .map(|&i| values[i] - starts[i].checked_sub(1).map(|j| values[j]).unwrap_or(0))
                    .collect();
                let mut actual = vec![99];
                sub.read_sorted_into_at(&indices, &mut actual);
                assert_eq!(&actual[1..], sums);
                assert_eq!(
                    avg.read_sorted_at(&indices),
                    valid
                        .iter()
                        .zip(&sums)
                        .map(|(&i, &sum)| f64::from(sum) / (i - starts[i] + 1) as f64)
                        .collect::<Vec<_>>()
                );
                assert_eq!(
                    change.read_sorted_at(&indices),
                    valid
                        .iter()
                        .map(|&i| f64::from(values[i]) - f64::from(values[starts[i]]))
                        .collect::<Vec<_>>()
                );
                assert_eq!(
                    rate.read_sorted_at(&indices),
                    valid
                        .iter()
                        .map(|&i| (f64::from(values[i]) - f64::from(values[starts[i]]))
                            / f64::from(values[starts[i]]))
                        .collect::<Vec<_>>()
                );
            }
        }
    }
}
