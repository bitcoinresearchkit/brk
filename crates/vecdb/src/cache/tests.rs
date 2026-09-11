use std::{
    ops::Range,
    panic::{AssertUnwindSafe, catch_unwind},
    sync::{
        Arc, Barrier,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread,
};

use super::{Cache, CacheBudget, CachePolicy, NoCache, Request, Table, Value};
use crate::{Error, READ_CHUNK_SIZE};

fn load(ranges: &[Range<usize>]) -> Vec<(usize, Vec<u64>)> {
    ranges
        .iter()
        .map(|range| (range.start, range.clone().map(|i| i as u64).collect()))
        .collect()
}

fn test_budget() -> &'static CacheBudget {
    Box::leak(Box::new(CacheBudget::new(1024 * 1024)))
}

#[test]
fn inline_values_and_no_cache_stay_small() {
    assert_eq!(size_of::<Value<u64>>(), 16);
    assert_eq!(size_of::<<NoCache as CachePolicy>::State<u64>>(), 0);
    assert!(NoCache::cache::<u64>(&NoCache).is_none());
}

#[test]
fn partial_ranges_sorted_duplicates_and_gaps_preserve_outputs() {
    let cache = Cache::new(test_budget());
    cache.read_scope(|| {
        let mut out = vec![999];
        cache.read(Request::Range(10, 20), &mut out, load);
        assert_eq!(out, [vec![999], (10..20).collect()].concat());
        for indices in [&[1, 10, 10, 19, 22][..], &[1, 21], &[0, 9, 20, 99]] {
            out.clear();
            out.push(999);
            cache.read(Request::Sorted(indices), &mut out, load);
            assert_eq!(
                &out[1..],
                indices.iter().map(|&i| i as u64).collect::<Vec<_>>()
            );
        }
        out.clear();
        cache.read(Request::Range(0, 100), &mut out, load);
        assert_eq!(out, (0..100).collect::<Vec<_>>());
        out.clear();
        cache.read(Request::Range(0, 100), &mut out, |_| {
            panic!("warm range decoded")
        });
        assert_eq!(out, (0..100).collect::<Vec<_>>());
    });
}

#[test]
fn generic_fills_and_source_rewrites_match_a_model() {
    let cache = Cache::<[u64; 3]>::new(test_budget());
    let mut model: Vec<_> = (0..1024).map(|i| [0, i, 0]).collect();
    let mut seed = 17_u64;
    for turn in 0..1500 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let from = seed as usize % model.len();
        let to = (from + 1 + (seed >> 32) as usize % 180).min(model.len());
        if turn % 17 == 0 {
            cache
                .update(from, true, || {
                    for (at, value) in model.iter_mut().enumerate().take(to).skip(from) {
                        *value = [turn, at as u64, seed];
                    }
                    Ok(())
                })
                .unwrap();
        }
        let mut out = Vec::new();
        cache.read_scope(|| {
            cache.read(Request::Range(from, to), &mut out, |ranges| {
                ranges
                    .iter()
                    .map(|range| (range.start, model[range.clone()].to_vec()))
                    .collect()
            })
        });
        assert_eq!(out, model[from..to], "turn {turn}");
        let table = cache.shared.table.read();
        let mut previous_end = 0;
        for (&at, value) in table.iter() {
            assert!(at >= previous_end);
            previous_end = at + value.len();
            assert_eq!(value.slice(), &model[at..previous_end], "turn {turn}");
        }
    }
}

#[test]
fn eviction_keeps_borrowed_buffers_charged_until_callback_finishes() {
    let budget = Box::leak(Box::new(CacheBudget::new(128 * 1024)));
    let cache = Cache::new(budget);
    cache.read_scope(|| cache.read(Request::Range(0, 4096), &mut Vec::new(), load));
    let cached = budget.used();
    assert!(cached > 4096 * 8);
    cache
        .read_scope(|| {
            cache.try_for_each_chunk(0, 4096, load, |_, values| {
                budget.clear();
                assert!(budget.used() >= 4096 * 8);
                assert!(budget.used() < cached);
                assert_eq!(values, (0..4096).collect::<Vec<_>>());
                Ok::<_, ()>(())
            })
        })
        .unwrap();
    assert_eq!(budget.used(), 0);
}

#[test]
fn active_reads_exclude_publication_and_callbacks_can_reenter() {
    let cache = Cache::new(test_budget());
    cache.read_scope(|| cache.read(Request::Range(0, 16), &mut Vec::new(), load));
    let barrier = Arc::new(Barrier::new(2));
    let (started_tx, started_rx) = mpsc::channel();
    let (done_tx, done_rx) = mpsc::channel();
    thread::scope(|scope| {
        let writer = cache.clone();
        let start = barrier.clone();
        scope.spawn(move || {
            start.wait();
            started_tx.send(()).unwrap();
            writer
                .update(0, true, || {
                    done_tx.send(()).unwrap();
                    Ok(())
                })
                .unwrap();
        });
        cache.read_scope(|| {
            barrier.wait();
            started_rx.recv().unwrap();
            cache
                .try_for_each_chunk(0, 16, load, |_, values| {
                    assert_eq!(values.len(), 16);
                    assert!(done_rx.try_recv().is_err());
                    cache.read_scope(|| {
                        cache.read(Request::Range(3, 4), &mut Vec::new(), |_| {
                            panic!("nested miss")
                        })
                    });
                    Ok::<_, ()>(())
                })
                .unwrap();
        });
        done_rx.recv().unwrap();
    });
}

#[test]
fn admission_preserves_hot_points_when_a_full_scan_cannot_fit() {
    let budget = Box::leak(Box::new(CacheBudget::new(16 * 1024)));
    let cache = Cache::new(budget);
    let indices = [1, 100, 2000, 10_000];
    cache.read_scope(|| {
        cache.read(Request::Sorted(&indices), &mut Vec::new(), load);
        let before = budget.used();
        cache.read(Request::Range(0, 65536), &mut Vec::new(), load);
        assert_eq!(budget.used(), before);
        cache.read(Request::Sorted(&indices), &mut Vec::new(), |_| {
            panic!("hot points evicted")
        });
        assert_eq!(before, Table::<u64>::charge_for(indices.len()));
    });
    drop(cache);
    assert_eq!(budget.used(), 0);
}

#[test]
fn fallible_cold_reads_stop_before_loading_the_rest() {
    let cache = Cache::new(test_budget());
    let mut loaded = 0;
    let result = cache.read_scope(|| {
        cache.try_for_each_chunk(
            0,
            1_000_000,
            |ranges| {
                loaded += ranges.iter().map(Range::len).sum::<usize>();
                load(ranges)
            },
            |_, _| Err::<(), _>("stop"),
        )
    });
    assert_eq!(result, Err("stop"));
    assert_eq!(loaded, READ_CHUNK_SIZE);
}

#[test]
fn concurrent_misses_fill_once_and_release_every_charge() {
    let budget = Box::leak(Box::new(CacheBudget::new(128 * 1024)));
    let cache = Cache::new(budget);
    let starts = Barrier::new(8);
    let fills = AtomicUsize::new(0);
    thread::scope(|threads| {
        for _ in 0..8 {
            threads.spawn(|| {
                starts.wait();
                let mut out = Vec::new();
                cache.read_scope(|| {
                    cache.read(Request::Range(0, 4096), &mut out, |ranges| {
                        fills.fetch_add(1, Ordering::Relaxed);
                        load(ranges)
                    })
                });
                assert_eq!(out, (0..4096).collect::<Vec<_>>());
            });
        }
    });
    assert_eq!(fills.load(Ordering::Relaxed), 1);
    assert!(budget.used() > 4096 * 8);
    drop(cache);
    assert_eq!(budget.used(), 0);
}

#[test]
fn empty_reads_and_panicking_loaders_leak_no_reservations() {
    let budget = Box::leak(Box::new(CacheBudget::new(4096)));
    let cache = Cache::new(budget);
    cache.read_scope(|| {
        cache.read(Request::Range(0, 0), &mut Vec::new(), |_| {
            panic!("empty read loaded")
        })
    });
    assert_eq!(budget.used(), 0);
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            cache.read_scope(|| {
                cache.read(Request::Range(0, 4), &mut Vec::new(), |_| {
                    panic!("load failed")
                })
            })
        }))
        .is_err()
    );
    assert_eq!(budget.used(), 0);
    let mut out = Vec::new();
    cache.read_scope(|| cache.read(Request::Range(0, 4), &mut out, load));
    assert_eq!(out, [0, 1, 2, 3]);
    assert!(budget.used() > 0);
    drop(cache);
    assert_eq!(budget.used(), 0);
}

#[test]
fn failed_updates_fail_closed_until_a_successful_update() {
    let cache = Cache::new(test_budget());
    cache.read_scope(|| cache.read(Request::Range(0, 4), &mut Vec::new(), load));
    assert!(
        cache
            .update(2, true, || Err::<(), _>(Error::InvalidArgument(
                "failed write"
            )))
            .is_err()
    );
    let mut out = vec![99];
    assert!(!cache.try_read_range(0, 2, || 4, &mut out));
    assert_eq!(out, [99]);
    assert!(catch_unwind(AssertUnwindSafe(|| cache.read_scope(|| ()))).is_err());
    cache.update(2, true, || Ok(())).unwrap();
    assert!(cache.try_read_range(0, 2, || 4, &mut out));
    assert_eq!(out, [99, 0, 1]);
    assert!(!cache.try_read_range(2, 4, || 4, &mut out));
}

#[test]
fn mixed_source_pressure_never_exceeds_the_shared_limit() {
    let budget = Box::leak(Box::new(CacheBudget::new(8192)));
    let caches: Vec<_> = (0..4).map(|_| Cache::new(budget)).collect();
    for turn in 0..300usize {
        let cache = &caches[turn % caches.len()];
        let from = turn * 113 % 4000;
        let count = if turn % 3 == 0 { 1 } else { 700 };
        let mut out = Vec::new();
        cache.read_scope(|| cache.read(Request::Range(from, from + count), &mut out, load));
        assert_eq!(
            out,
            (from..from + count).map(|i| i as u64).collect::<Vec<_>>()
        );
        assert!(budget.used() <= budget.limit());
    }
    drop(caches);
    assert_eq!(budget.used(), 0);
}
