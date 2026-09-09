use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, BytesVec, Database, ImportableVec, LazyAggVec, LazyVec, READ_CHUNK_SIZE,
    ReadBounds, ReadableBoxedVec, ReadableCloneableVec, ReadableVec, Version, WritableVec,
};

#[cfg(feature = "pco")]
use vecdb::PcoVec;

static READS: AtomicUsize = AtomicUsize::new(0);

#[test]
fn aggregation_sorted_reads_are_selective_and_preserve_empty_buckets() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut source = BytesVec::<usize, u64>::import(&db, "source", Version::ONE).unwrap();
    for i in 0..20_000u64 {
        source.push(i);
    }
    source.write().unwrap();
    let counted = LazyVec::<usize, u64, usize, u64>::init(
        "counted",
        Version::ONE,
        ReadableBoxedVec::new(source.read_only_clone()),
        |_, value| {
            READS.fetch_add(1, Ordering::Relaxed);
            value
        },
    );
    let mapping = Arc::new((0..10_000usize).map(|i| i * 2).collect::<Vec<_>>());
    let mapping_reads = Arc::new(AtomicUsize::new(0));
    let counter = mapping_reads.clone();
    let agg = LazyAggVec::<usize, Option<u64>, usize, usize, u64>::new(
        "agg",
        Version::ONE,
        Version::ONE,
        counted.read_only_boxed_clone(),
        move || {
            counter.fetch_add(1, Ordering::Relaxed);
            mapping.clone()
        },
    );
    READS.store(0, Ordering::Relaxed);
    mapping_reads.store(0, Ordering::Relaxed);
    assert_eq!(
        agg.read_sorted_at(&[0, 3, 3, 9_999, 10_000, usize::MAX]),
        [Some(1), Some(7), Some(7), Some(19_999)]
    );
    assert_eq!(READS.load(Ordering::Relaxed), 3);
    assert_eq!(mapping_reads.load(Ordering::Relaxed), 1);
    mapping_reads.store(0, Ordering::Relaxed);
    let mut actual = Vec::new();
    agg.for_each_chunk_at(3, 11_000, &mut |at, values| {
        assert_eq!(at, 3 + actual.len());
        assert!(values.len() <= READ_CHUNK_SIZE);
        actual.extend_from_slice(values);
    });
    assert_eq!(
        actual,
        (3..10_000u64).map(|i| Some(i * 2 + 1)).collect::<Vec<_>>()
    );
    let mapping = Arc::new(vec![0, 0, 2, 2, 4, 20_001]);
    let agg = LazyAggVec::<usize, Option<u64>, usize, usize, u64>::new(
        "empty",
        Version::ONE,
        Version::ONE,
        ReadableBoxedVec::new(source.read_only_clone()),
        move || mapping.clone(),
    );
    let mut bounds = ReadBounds::new();
    bounds.set("usize", 3);
    bounds.scope(|| {
        assert_eq!(
            agg.read_sorted_at(&[0, 1, 1, 2, 3, 4, 5, usize::MAX]),
            [None, Some(1), Some(1), None, Some(2), None, None]
        );
    });
    assert!(agg.read_sorted_at(&[]).is_empty());
    assert!(agg.read_sorted_at(&[usize::MAX]).is_empty());
    let mut seen = 0;
    assert_eq!(
        agg.try_fold_range_at(0, 6, (), |(), _| {
            seen += 1;
            if seen == 2 { Err("stop") } else { Ok(()) }
        }),
        Err("stop")
    );
    assert_eq!(seen, 2);
}

#[test]
#[cfg(feature = "pco")]
fn compressed_sorted_gather_handles_raw_tail_pushes_rewrite_and_reopen() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut source = PcoVec::<usize, u64>::import(&db, "compressed", Version::ONE).unwrap();
    let mut expected: Vec<_> = (0..20_137u64)
        .map(|i| i.wrapping_mul(6364136223846793005))
        .collect();
    for &value in &expected {
        source.push(value);
    }
    source.write().unwrap();
    let stored = expected.len();
    for value in 0..333u64 {
        source.push(value);
        expected.push(value);
    }
    let indices = [
        0,
        1,
        1,
        1023,
        1024,
        1025,
        2047,
        4096,
        19_999,
        20_136,
        20_137,
        20_300,
        20_469,
        20_470,
        usize::MAX,
    ];
    assert_eq!(
        source.read_sorted_at(&indices),
        indices
            .iter()
            .filter_map(|&i| expected.get(i).copied())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        source.read_only_clone().read_sorted_at(&indices),
        indices
            .iter()
            .filter_map(|&i| expected[..stored].get(i).copied())
            .collect::<Vec<_>>()
    );
    source.write().unwrap();
    let mut appended = vec![99];
    source
        .read_only_clone()
        .read_sorted_into_at(&indices, &mut appended);
    assert_eq!(
        &appended[1..],
        indices
            .iter()
            .filter_map(|&i| expected.get(i).copied())
            .collect::<Vec<_>>()
    );
    source.truncate_if_needed_at(10_011).unwrap();
    expected.truncate(10_011);
    for value in 0..2000u64 {
        source.push(value + 7);
        expected.push(value + 7);
    }
    source.write().unwrap();
    drop(source);
    let source = PcoVec::<usize, u64>::import(&db, "compressed", Version::ONE).unwrap();
    let indices: Vec<_> = (0..expected.len())
        .step_by(13)
        .flat_map(|i| [i, i])
        .collect();
    assert_eq!(
        source.read_sorted_at(&indices),
        indices.iter().map(|&i| expected[i]).collect::<Vec<_>>()
    );
    assert!(source.read_sorted_at(&[]).is_empty());
    assert!(source.read_sorted_at(&[usize::MAX]).is_empty());
}
