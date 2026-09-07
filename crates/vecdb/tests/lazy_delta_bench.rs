#![cfg(feature = "pco")]

use std::{hint::black_box, sync::Arc, time::Instant};

use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, CachedVec, Database, DeltaSub, ImportableVec, LazyDeltaVec, LazyVec, PcoVec,
    ReadableBoxedVec, ReadableCloneableVec, ReadableVec, StoredVec, UnaryTransform, Version,
    WritableVec,
};

struct Twice;
impl UnaryTransform<u64> for Twice {
    fn apply(value: u64) -> u64 {
        value * 2
    }
}

#[test]
#[ignore = "synthetic delta range benchmark; run unchanged before and after"]
fn benchmark_delta_ranges() {
    const N: usize = 262_144;
    const WINDOW: usize = 131_072;
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut source = PcoVec::<usize, u64>::import(&db, "source", Version::ONE).unwrap();
    for i in 0..N {
        source.push((i as u64 + 1) * 3);
    }
    source.write().unwrap();
    let cached = CachedVec::wrap(source.read_only_clone());
    cached.snapshot();
    let sources: [(&str, ReadableBoxedVec<usize, u64>); 2] = [
        ("pco", source.read_only_boxed_clone()),
        ("cached", cached.read_only_boxed_clone()),
    ];
    for (name, source) in sources {
        let starts = Arc::new((0..N).map(|i| i.saturating_sub(WINDOW)).collect::<Vec<_>>());
        let delta = LazyDeltaVec::<usize, u64, u64, DeltaSub>::new(
            "delta",
            Version::ONE,
            source,
            Version::ONE,
            move || starts.clone(),
        );
        let nested = LazyVec::<usize, u64, usize, u64>::transformed::<Twice>(
            "nested",
            Version::ONE,
            delta.read_only_boxed_clone(),
        );
        let expected: Vec<_> = (0..N).map(|i| (i + 1).min(WINDOW + 1) as u64 * 3).collect();
        assert_eq!(delta.collect_range_at(0, N), expected);
        let mut times = vec![Vec::new(); 4];
        for round in 0..10 {
            for offset in 0..4 {
                let mode = (round + offset) % 4;
                let from = if mode < 2 { N - 64 } else { 0 };
                let start = Instant::now();
                let actual = if mode % 2 == 0 {
                    delta.collect_range_at(black_box(from), black_box(N))
                } else {
                    nested.collect_range_at(black_box(from), black_box(N))
                };
                let elapsed = start.elapsed();
                if mode % 2 == 0 {
                    assert_eq!(actual, expected[from..]);
                } else {
                    assert_eq!(
                        actual,
                        expected[from..].iter().map(|v| v * 2).collect::<Vec<_>>()
                    );
                }
                black_box(actual);
                if round > 0 {
                    times[mode].push(elapsed);
                }
            }
        }
        for (label, samples) in ["short_direct", "short_nested", "full_direct", "full_nested"]
            .into_iter()
            .zip(&mut times)
        {
            samples.sort();
            eprintln!(
                "{name}/{label}: median={:?} min={:?} max={:?}",
                samples[4], samples[0], samples[8]
            );
        }
    }
}
