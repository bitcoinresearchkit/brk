use std::{hint::black_box, time::Instant};

use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, BytesVec, Database, ImportableVec, MutableVec, ReadableVec, StoredVec, Version,
    WritableVec,
};

fn check(source: &impl ReadableVec<usize, u64>, expected: &[Option<u64>]) {
    for indices in [
        vec![],
        vec![0],
        vec![usize::MAX],
        (0..expected.len() + 3).collect(),
        (0..expected.len() + 3).flat_map(|i| [i, i]).collect(),
        (0..expected.len() + 3).step_by(173).collect(),
    ] {
        let mut actual = vec![123];
        source.read_sorted_into_at(&indices, &mut actual);
        let mut wanted = vec![123];
        wanted.extend(
            indices
                .iter()
                .filter_map(|&i| expected.get(i).copied().flatten()),
        );
        assert_eq!(actual, wanted);
    }
}

macro_rules! mutation_roundtrip {
    ($raw:ty) => {{
        type V = $raw;
        let dir = tempdir().unwrap();
        let db = Database::open(dir.path()).unwrap();
        let mut source = MutableVec::<V>::import(&db, "values", Version::ONE).unwrap();
        let mut expected: Vec<_> = (0..8192).map(|i| Some(i * 7)).collect();
        for value in &expected {
            source.push(value.unwrap());
        }
        source.write().unwrap();
        let reader = source.read_only_clone();
        for i in (0..expected.len()).step_by(17) {
            source.delete_at(i);
            expected[i] = None;
        }
        for i in (0..expected.len()).step_by(23) {
            source.update_at(i, 999).unwrap();
            expected[i] = Some(999);
        }
        for i in 0..99 {
            source.push(i);
            expected.push(Some(i));
        }
        source.delete_at(8200);
        expected[8200] = None;
        check(&source, &expected);
        check(&reader, &(0..8192).map(|i| Some(i * 7)).collect::<Vec<_>>());
        source.write().unwrap();
        check(&source, &expected);
        check(&reader, &expected);
        source.truncate_if_needed_at(4100).unwrap();
        expected.truncate(4100);
        source.update_at(4099, 456).unwrap();
        expected[4099] = Some(456);
        check(&source, &expected);
        source.write().unwrap();
        check(&reader, &expected);
        drop(reader);
        drop(source);
        drop(db);
        let db = Database::open(dir.path()).unwrap();
        let source = MutableVec::<V>::import(&db, "values", Version::ONE).unwrap();
        check(&source, &expected);
    }};
}

#[test]
fn bytes_sorted_mutations_keep_physical_indices_duplicates_and_publication() {
    mutation_roundtrip!(BytesVec<usize, u64>);
}

#[cfg(feature = "zerocopy")]
#[test]
fn zerocopy_sorted_mutations_keep_physical_indices_duplicates_and_publication() {
    mutation_roundtrip!(vecdb::ZeroCopyVec<usize, u64>);
}

fn measure<T: PartialEq + std::fmt::Debug>(
    name: &str,
    expected: &[T],
    mut f: impl FnMut() -> Vec<T>,
) {
    let mut times = Vec::new();
    for round in 0..12 {
        let start = Instant::now();
        let actual = f();
        let elapsed = start.elapsed();
        assert_eq!(actual, expected);
        black_box(actual);
        if round > 0 {
            times.push(elapsed);
        }
    }
    times.sort();
    eprintln!("{name}: {:?}", times[5]);
}

#[test]
#[ignore = "Paired mutable lookup benchmark; checks all outputs"]
fn benchmark_mutable_lookup() {
    const N: usize = 262144;
    let dir = tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let mut source =
        MutableVec::<BytesVec<usize, u64>>::import(&db, "values", Version::ONE).unwrap();
    for i in 0..N {
        source.push(i as u64);
    }
    source.write().unwrap();
    for i in (0..N).step_by(17) {
        source.delete_at(i);
    }
    for i in (0..N).step_by(23) {
        source.update_at(i, 999).unwrap();
    }
    for (pattern, indices) in [
        ("one", vec![N - 3]),
        ("spread", (0..32).map(|i| i * (N - 1) / 31).collect()),
        ("dense", (N - 4096..N).collect()),
        ("duplicates", (0..4096).map(|i| N - 4096 + i / 4).collect()),
    ] {
        let expected: Vec<_> = indices
            .iter()
            .filter_map(|&i| source.collect_one_at(i))
            .collect();
        measure(&format!("writer/{pattern}/new"), &expected, || {
            source.read_sorted_at(black_box(&indices))
        });
        measure(&format!("writer/{pattern}/old"), &expected, || {
            let mut out = Vec::with_capacity(indices.len());
            out.extend(
                black_box(&indices)
                    .iter()
                    .filter_map(|&i| source.collect_one_at(i)),
            );
            out
        });
    }
    source.write().unwrap();
    let reader = source.read_only_clone();
    let raw = std::ops::Deref::deref(&source).read_only_clone();
    for (pattern, indices) in [
        ("one", vec![N - 3]),
        ("spread", (0..32).map(|i| i * (N - 1) / 31).collect()),
        ("dense", (N - 4096..N).collect()),
        ("duplicates", (0..4096).map(|i| N - 4096 + i / 4).collect()),
    ] {
        let expected: Vec<_> = indices
            .iter()
            .filter_map(|&i| reader.collect_one_at(i))
            .collect();
        measure(&format!("reader/{pattern}/new"), &expected, || {
            reader.read_sorted_at(black_box(&indices))
        });
        // Old loop, with a fixed hole set; excludes its one outer read lock.
        measure(&format!("reader/{pattern}/old"), &expected, || {
            let mut out = Vec::with_capacity(indices.len());
            out.extend(black_box(&indices).iter().filter_map(|&i| {
                if source.holes().contains(&i) {
                    None
                } else {
                    raw.collect_one_at(i)
                }
            }));
            out
        });
    }
}
