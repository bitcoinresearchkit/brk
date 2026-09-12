use std::{
    hint::black_box,
    sync::LazyLock,
    time::{Duration, Instant},
};

use tempfile::tempdir;
#[cfg(feature = "pco")]
use vecdb::PcoVec;
use vecdb::{Budgeted, BytesVec, CacheBudget, Database, StoredVec, Version};

static BUDGET: LazyLock<&CacheBudget> =
    LazyLock::new(|| Budgeted::init_global(32 * 1024 * 1024).unwrap());

fn updates<V: StoredVec<I = usize, T = u64>>(
    source: &mut V,
    warm_all: bool,
    rewrite: bool,
) -> Duration {
    const ITERATIONS: u32 = 256;
    let mut times = Vec::new();
    for _ in 0..15 {
        BUDGET.clear();
        let from = if warm_all { 0 } else { source.len() - 1 };
        black_box(source.collect_range_at(from, source.len()));
        let start = Instant::now();
        for value in 0..ITERATIONS {
            if rewrite {
                source.truncate_if_needed_at(source.len() - 1).unwrap();
            }
            source.push(u64::from(value));
            source.write().unwrap();
            assert_eq!(black_box(source.collect_last()), Some(u64::from(value)));
        }
        times.push(start.elapsed() / ITERATIONS);
    }
    times.sort_unstable();
    times[7]
}

fn compare<C, N>(name: &str)
where
    C: StoredVec<I = usize, T = u64>,
    N: StoredVec<I = usize, T = u64>,
{
    LazyLock::force(&BUDGET);
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut cached = C::import(&db, "cached", Version::ONE).unwrap();
    let mut native = N::import(&db, "native", Version::ONE).unwrap();
    for index in 0..1_000_000_u64 {
        cached.push(index);
        native.push(index);
    }
    cached.write().unwrap();
    native.write().unwrap();
    for warm_all in [false, true] {
        for rewrite in [false, true] {
            let cached_time = updates(&mut cached, warm_all, rewrite);
            let retained = BUDGET.used();
            let native_time = updates(&mut native, warm_all, rewrite);
            eprintln!(
                "{name} warm_all={warm_all} rewrite={rewrite}: write+last cached={cached_time:?}, native={native_time:?}, retained={retained}"
            );
        }
    }
}

#[test]
#[ignore = "synthetic committed append and tail-rewrite benchmark"]
fn raw_committed_updates() {
    compare::<BytesVec<usize, u64, Budgeted>, BytesVec<usize, u64>>("raw");
}

#[test]
#[ignore = "synthetic committed append and tail-rewrite benchmark"]
#[cfg(feature = "pco")]
fn compressed_committed_updates() {
    compare::<PcoVec<usize, u64, Budgeted>, PcoVec<usize, u64>>("pco");
}
