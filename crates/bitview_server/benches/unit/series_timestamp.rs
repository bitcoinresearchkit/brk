use std::{hint::black_box, time::Instant};

use brk_types::{Height, RangeMap, Timestamp};
use parking_lot::RwLock;
use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, AnyVec, CachedVec, Database, EagerVec, ImportableVec, PcoVec, ReadableVec,
    StoredVec, Version, WritableVec,
};

/// Storage-level comparison, not an HTTP/query benchmark. Mirrors the lookup
/// operations over the same concrete vector used by timestamp mappings.
#[test]
#[ignore = "million-row timestamp storage comparison; temporary local data only"]
fn benchmark_timestamp_lookup() {
    const LEN: usize = 1_000_000;
    let directory = tempdir().unwrap();
    let database = Database::open(directory.path()).unwrap();
    let mut stored: EagerVec<PcoVec<Height, Timestamp>> =
        EagerVec::import(&database, "timestamps", Version::ONE).unwrap();
    for index in 0..LEN {
        // Duplicates exercise first-equal semantics.
        stored.push(Timestamp::new(1_200_000_000 + (index / 2) as u32 * 600));
    }
    stored.write().unwrap();
    let values = CachedVec::wrap(stored.read_only_clone());
    let start = Instant::now();
    let copied = RwLock::new(RangeMap::<Timestamp, Height>::from(values.collect()));
    eprintln!(
        "timestamp copied-map initialization {:?}, extra timestamp bytes {}",
        start.elapsed(),
        LEN * size_of::<Timestamp>()
    );
    let targets = [
        0,
        1_200_000_000,
        1_200_000_001,
        1_350_000_000,
        1_499_999_400,
        u32::MAX,
    ];
    let mut times = [Vec::new(), Vec::new(), Vec::new()];
    for round in 0..24 {
        for offset in 0..3 {
            let variant = (round + offset) % 3;
            let start = Instant::now();
            for sample in 0..6000 {
                let target = Timestamp::new(black_box(targets[sample % targets.len()]));
                let position = match variant {
                    0 => copied
                        .read()
                        .ceil(target)
                        .map(usize::from)
                        .unwrap_or(LEN - 1),
                    1 => {
                        let (mut low, mut high) = (0, values.visible_len());
                        while low < high {
                            let middle = low + (high - low) / 2;
                            if values.collect_one(Height::from(middle)).unwrap() < target {
                                low = middle + 1;
                            } else {
                                high = middle;
                            }
                        }
                        if low == LEN { LEN - 1 } else { low }
                    }
                    _ => {
                        let len = values.visible_len();
                        let snapshot = values.snapshot();
                        let visible = snapshot.get(..len).unwrap();
                        let position = visible.partition_point(|value| *value < target);
                        if position == len { LEN - 1 } else { position }
                    }
                };
                if round == 0 {
                    assert_eq!(
                        position,
                        copied
                            .read()
                            .ceil(target)
                            .map(usize::from)
                            .unwrap_or(LEN - 1)
                    );
                }
                black_box(position);
            }
            if round >= 4 {
                times[variant].push(start.elapsed() / 6000);
            }
        }
    }
    for samples in &mut times {
        samples.sort_unstable();
    }
    eprintln!(
        "timestamp warm lookup: copied map {:?}, scalar probes {:?}, shared snapshot {:?}",
        times[0][10], times[1][10], times[2][10]
    );
}
