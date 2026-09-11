use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

use bitview_vecs::LazyAggVec;
use rangeindex::SharedRangeMap;
use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Database, ImportableVec, LazyVec, READ_CHUNK_SIZE, ReadBounds,
    ReadableBoxedVec, ReadableCloneableVec, ReadableVec, ValueWriter, Version, WritableVec,
};

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
    let mapping = SharedRangeMap::new((0..10_000usize).map(|i| i * 2).collect());
    let agg = LazyAggVec::<usize, Option<u64>, usize, u64>::new(
        "agg",
        Version::ONE,
        counted.read_only_boxed_clone(),
        mapping,
    );
    READS.store(0, Ordering::Relaxed);
    assert_eq!(
        agg.read_sorted_at(&[0, 3, 3, 9_999, 10_000, usize::MAX]),
        [Some(1), Some(7), Some(7), Some(19_999)]
    );
    assert_eq!(READS.load(Ordering::Relaxed), 3);
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
    let mapping = SharedRangeMap::new(vec![0, 0, 2, 2, 4, 20_001]);
    let agg = LazyAggVec::<usize, Option<u64>, usize, u64>::new(
        "empty",
        Version::ONE,
        ReadableBoxedVec::new(source.read_only_clone()),
        mapping,
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
fn nested_lazy_inputs_keep_bounds_without_an_ambient_scope_and_after_thread_handoff() {
    let temp = tempdir().unwrap();
    let db = Database::open(temp.path()).unwrap();
    let mut source: BytesVec<usize, u64> =
        BytesVec::forced_import(&db, "source", Version::ONE).unwrap();
    for value in [10, 20, 30] {
        source.push(value);
    }
    source.flush().unwrap();
    let last = LazyAggVec::<usize, Option<u64>, usize, u64>::new(
        "last",
        Version::ONE,
        source.read_only_boxed_clone(),
        SharedRangeMap::new(vec![0]),
    );
    assert_eq!(last.collect_one_at(0), Some(Some(30)));
    let mut bounds = ReadBounds::new();
    bounds.set("usize", 2);
    let bounded = bounds.bind(&last).unwrap();
    let read = || {
        let mut json = Vec::new();
        bounded.write_json(None, None, &mut json).unwrap();
        assert_eq!(
            json, b"[20]",
            "output length alone does not constrain the lazy source tail"
        );
        json.clear();
        bounded.write_json_value_at(0, &mut json).unwrap();
        assert_eq!(json, b"20");
        let mut csv = String::new();
        bounded.write_csv_column(None, None, &mut csv).unwrap();
        assert_eq!(csv, "20\n");
        csv.clear();
        bounded
            .create_writer(None, None)
            .write_next(&mut csv)
            .unwrap();
        assert_eq!(csv, "20");
    };
    read();
    thread::scope(|threads| threads.spawn(read).join().unwrap());
    let mut outer = ReadBounds::new();
    outer.set("usize", 1);
    outer.scope(|| {
        assert_eq!(source.visible_len(), 1);
        read();
        assert_eq!(source.visible_len(), 1);
    });
    assert_eq!(source.visible_len(), 3);
}
