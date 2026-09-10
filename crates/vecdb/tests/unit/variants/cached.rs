use std::{
    sync::{
        Arc, Barrier,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering::SeqCst},
    },
    thread,
};

use super::*;
use crate::{AnyVec, PrintableIndex, ReadableVec, TypedVec, short_type_name};

struct TestBudget {
    remaining: AtomicUsize,
    reservations: AtomicUsize,
}

impl TestBudget {
    const fn new(bytes: usize) -> Self {
        Self {
            remaining: AtomicUsize::new(bytes),
            reservations: AtomicUsize::new(0),
        }
    }
}

impl CachedVecBudget for TestBudget {
    fn record_access(&self) -> u64 {
        0
    }

    fn try_reserve(&self, bytes: usize) -> bool {
        let reserved = self
            .remaining
            .fetch_update(SeqCst, SeqCst, |remaining| remaining.checked_sub(bytes))
            .is_ok();
        if reserved {
            self.reservations.fetch_add(1, SeqCst);
        }
        reserved
    }

    fn release(&self, bytes: usize) {
        self.remaining.fetch_add(bytes, SeqCst);
    }
}

#[derive(Clone)]
struct BlockingVec {
    values: Arc<RwLock<Vec<u32>>>,
    started: Arc<Barrier>,
    resume: Arc<Barrier>,
    block_once: Arc<AtomicBool>,
    panic_once: Arc<AtomicBool>,
}

impl BlockingVec {
    fn new(values: impl IntoIterator<Item = u32>) -> Self {
        Self {
            values: Arc::new(RwLock::new(values.into_iter().collect())),
            started: Arc::new(Barrier::new(2)),
            resume: Arc::new(Barrier::new(2)),
            block_once: Arc::new(AtomicBool::new(true)),
            panic_once: Arc::new(AtomicBool::new(false)),
        }
    }

    fn replace(&self, index: usize, value: u32) {
        self.values.write()[index] = value;
    }

    fn values(&self, from: usize, to: usize) -> Vec<u32> {
        let values = self.values.read();
        values[from.min(values.len())..to.min(values.len())].to_vec()
    }
}

impl AnyVec for BlockingVec {
    fn version(&self) -> Version {
        Version::ONE
    }

    fn name(&self) -> &str {
        "blocking"
    }

    fn len(&self) -> usize {
        self.values.read().len()
    }

    fn index_type_to_string(&self) -> &'static str {
        <usize as PrintableIndex>::to_string()
    }

    fn region_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn value_type_to_size_of(&self) -> usize {
        size_of::<u32>()
    }

    fn value_type_to_string(&self) -> &'static str {
        short_type_name::<u32>()
    }
}

impl TypedVec for BlockingVec {
    type I = usize;
    type T = u32;
}

impl ReadableVec<usize, u32> for BlockingVec {
    fn cursor_chunk_size(&self) -> usize {
        2
    }

    fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<u32>) {
        assert!(!self.panic_once.swap(false, SeqCst), "failed snapshot read");
        let values = self.values(from, to);
        if self.block_once.swap(false, SeqCst) {
            self.started.wait();
            self.resume.wait();
        }
        buf.extend(values);
    }

    fn for_each_range_dyn_at(&self, from: usize, to: usize, each: &mut dyn FnMut(u32)) {
        self.values(from, to).into_iter().for_each(each);
    }

    fn fold_range_at<B, F: FnMut(B, u32) -> B>(
        &self,
        from: usize,
        to: usize,
        init: B,
        fold: F,
    ) -> B {
        self.values(from, to).into_iter().fold(init, fold)
    }

    fn try_fold_range_at<B, E, F: FnMut(B, u32) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        fold: F,
    ) -> Result<B, E> {
        self.values(from, to).into_iter().try_fold(init, fold)
    }
}

#[test]
fn invalidation_rejects_an_in_flight_stale_materialization() {
    let source = BlockingVec::new([0, 1]);
    let cached = CachedVec::wrap(source.clone());
    let reader = cached.clone();
    let handle = thread::spawn(move || reader.snapshot());

    source.started.wait();
    source.replace(1, 2);
    cached.invalidate();
    source.resume.wait();

    assert_eq!(handle.join().unwrap().as_slice(), [0, 2]);
    assert_eq!(cached.snapshot().as_slice(), [0, 2]);
}

#[test]
fn budget_tracks_resident_bytes_across_invalidation_and_resize() {
    static BUDGET: TestBudget = TestBudget::new(16);

    let source = BlockingVec::new([0, 1]);
    source.block_once.store(false, SeqCst);
    let resident_bytes = Arc::new(AtomicUsize::new(0));
    let cached = CachedVec::wrap_budgeted(
        source.clone(),
        &BUDGET,
        Arc::new(AtomicU64::new(0)),
        resident_bytes.clone(),
    );

    assert_eq!(cached.collect_range_at(0, 2), [0, 1]);
    assert_eq!(resident_bytes.load(SeqCst), 8);
    assert_eq!(BUDGET.remaining.load(SeqCst), 8);

    source.values.write().push(2);
    assert_eq!(cached.collect_range_at(0, 3), [0, 1, 2]);
    assert_eq!(resident_bytes.load(SeqCst), 12);
    assert_eq!(BUDGET.remaining.load(SeqCst), 4);

    cached.invalidate();
    assert_eq!(resident_bytes.load(SeqCst), 0);
    assert_eq!(BUDGET.remaining.load(SeqCst), 16);
}

#[test]
fn partial_reads_fall_through_until_every_chunk_is_touched() {
    static BUDGET: TestBudget = TestBudget::new(32);

    let source = BlockingVec::new([0, 1, 2, 3, 4, 5]);
    source.block_once.store(false, SeqCst);
    let resident_bytes = Arc::new(AtomicUsize::new(0));
    let cached = CachedVec::wrap_budgeted(
        source,
        &BUDGET,
        Arc::new(AtomicU64::new(0)),
        resident_bytes.clone(),
    );

    assert_eq!(cached.collect_one_at(0), Some(0));
    assert_eq!(cached.collect_range_at(0, 2), [0, 1]);
    assert_eq!(resident_bytes.load(SeqCst), 0);
    assert_eq!(BUDGET.reservations.load(SeqCst), 0);

    assert_eq!(cached.read_sorted_at(&[0, 4]), [0, 4]);
    assert_eq!(resident_bytes.load(SeqCst), 0);
    assert_eq!(BUDGET.reservations.load(SeqCst), 0);

    assert_eq!(cached.read_sorted_at(&[0, 2, 4]), [0, 2, 4]);
    assert_eq!(resident_bytes.load(SeqCst), 24);
    assert_eq!(BUDGET.reservations.load(SeqCst), 1);

    cached.invalidate();
    assert_eq!(BUDGET.remaining.load(SeqCst), 32);
}

#[test]
fn snapshot_explicitly_materializes_a_budgeted_vec() {
    static BUDGET: TestBudget = TestBudget::new(32);

    let source = BlockingVec::new([0, 1, 2, 3, 4, 5]);
    source.block_once.store(false, SeqCst);
    let resident_bytes = Arc::new(AtomicUsize::new(0));
    let cached = CachedVec::wrap_budgeted(
        source,
        &BUDGET,
        Arc::new(AtomicU64::new(0)),
        resident_bytes.clone(),
    );

    assert_eq!(cached.collect_one_at(0), Some(0));
    assert_eq!(resident_bytes.load(SeqCst), 0);

    assert_eq!(cached.snapshot().as_slice(), [0, 1, 2, 3, 4, 5]);
    assert_eq!(resident_bytes.load(SeqCst), 24);
    assert_eq!(BUDGET.reservations.load(SeqCst), 1);

    cached.invalidate();
    assert_eq!(BUDGET.remaining.load(SeqCst), 32);
}

#[test]
fn concurrent_miss_reserves_once() {
    static BUDGET: TestBudget = TestBudget::new(16);

    let source = BlockingVec::new([0, 1]);
    let cached = CachedVec::wrap_budgeted(
        source.clone(),
        &BUDGET,
        Arc::new(AtomicU64::new(0)),
        Arc::new(AtomicUsize::new(0)),
    );
    let first = cached.clone();
    let second = cached.clone();
    let first_handle = thread::spawn(move || first.collect_range_at(0, 2));

    source.started.wait();
    let second_handle = thread::spawn(move || second.collect_range_at(0, 2));
    source.resume.wait();

    assert_eq!(first_handle.join().unwrap(), [0, 1]);
    assert_eq!(second_handle.join().unwrap(), [0, 1]);
    assert_eq!(BUDGET.reservations.load(SeqCst), 1);
    assert_eq!(BUDGET.remaining.load(SeqCst), 8);

    cached.invalidate();
    assert_eq!(BUDGET.remaining.load(SeqCst), 16);
}

#[test]
fn empty_vec_materializes_once() {
    static BUDGET: TestBudget = TestBudget::new(16);

    let source = BlockingVec::new([]);
    source.block_once.store(false, SeqCst);
    let cached = CachedVec::wrap_budgeted(
        source,
        &BUDGET,
        Arc::new(AtomicU64::new(0)),
        Arc::new(AtomicUsize::new(0)),
    );

    assert!(cached.snapshot().is_empty());
    assert!(cached.snapshot().is_empty());
    assert_eq!(BUDGET.reservations.load(SeqCst), 0);
}

#[test]
fn failed_materialization_releases_its_reservation() {
    use std::panic::{AssertUnwindSafe, catch_unwind};

    static BUDGET: TestBudget = TestBudget::new(8);
    let source = BlockingVec::new([10, 20]);
    source.block_once.store(false, SeqCst);
    source.panic_once.store(true, SeqCst);
    let resident_bytes = Arc::new(AtomicUsize::new(0));
    let cached = CachedVec::wrap_budgeted(
        source,
        &BUDGET,
        Arc::new(AtomicU64::new(0)),
        resident_bytes.clone(),
    );

    assert!(catch_unwind(AssertUnwindSafe(|| cached.snapshot())).is_err());
    assert_eq!(BUDGET.remaining.load(SeqCst), 8);
    assert_eq!(resident_bytes.load(SeqCst), 0);
    assert!(cached.cached_snapshot().is_none());

    assert_eq!(cached.snapshot().as_slice(), [10, 20]);
    assert_eq!(BUDGET.remaining.load(SeqCst), 0);
    assert_eq!(resident_bytes.load(SeqCst), 8);
    cached.invalidate();
    assert_eq!(BUDGET.remaining.load(SeqCst), 8);

    assert!(BUDGET.try_reserve(4));
    assert_eq!(cached.snapshot().as_slice(), [10, 20]);
    assert!(cached.cached_snapshot().is_none());
    assert_eq!(BUDGET.remaining.load(SeqCst), 4);
    assert_eq!(resident_bytes.load(SeqCst), 0);
    BUDGET.release(4);
}

#[test]
fn invalidated_fill_returns_its_reservation_before_retrying() {
    static BUDGET: TestBudget = TestBudget::new(8);
    let source = BlockingVec::new([10, 20]);
    let cached = CachedVec::wrap_budgeted(
        source.clone(),
        &BUDGET,
        Arc::new(AtomicU64::new(0)),
        Arc::new(AtomicUsize::new(0)),
    );
    let reader = cached.clone();
    let handle = thread::spawn(move || reader.snapshot());

    source.started.wait();
    source.replace(1, 30);
    cached.invalidate();
    source.resume.wait();

    let snapshot = handle.join().unwrap();
    assert_eq!(snapshot.as_slice(), [10, 30]);
    assert!(Arc::ptr_eq(&snapshot, &cached.cached_snapshot().unwrap()));
    assert_eq!(BUDGET.reservations.load(SeqCst), 2);
    assert_eq!(BUDGET.remaining.load(SeqCst), 0);
    cached.invalidate();
    assert_eq!(BUDGET.remaining.load(SeqCst), 8);
}
