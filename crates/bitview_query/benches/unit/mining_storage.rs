//! Full-window storage/grouping/serialization cost, not an HTTP latency claim.

use std::{hint::black_box, time::Instant};

use brk_types::{BlockSizeEntry, BlockSizesWeights, BlockWeightEntry, StoredU64, Version, Weight};
use serde_json::to_vec;
use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, Budgeted, CacheBudget, Database, ImportOptions, ImportableVec, PcoVec,
    WritableVec,
};

use super::*;
use crate::RepresentationId;

#[test]
#[ignore = "one-million-row persisted mining data path; warm OS storage, excludes HTTP and plugin lookup"]
fn benchmark_full_history_storage() {
    const ROWS: u32 = 1_000_000;
    let directory = tempdir().unwrap();
    let database = Database::open(directory.path()).unwrap();
    let mut times: PcoVec<Height, Timestamp> =
        PcoVec::forced_import(&database, "timestamps", Version::ONE).unwrap();
    let mut sizes: PcoVec<Height, StoredU64> =
        PcoVec::forced_import(&database, "sizes", Version::ONE).unwrap();
    let mut weights: PcoVec<Height, Weight> =
        PcoVec::forced_import(&database, "weights", Version::ONE).unwrap();
    for height in 0..ROWS {
        // Include clock regressions and varying, realistic-scale block values.
        let time = 1_231_006_505 + height * 600 - (height % 17) * 61;
        let size = 100_000 + u64::from(height % 1_500_000);
        times.push(Timestamp::new(time));
        sizes.push(StoredU64::from(size));
        weights.push(Weight::from(size * 3));
    }
    times.write().unwrap();
    sizes.write().unwrap();
    weights.write().unwrap();
    // Reopen so persisted compressed sources, not pending write buffers, feed
    // the production window reader. Timestamp retention matches the indexer.
    drop((times, sizes, weights));
    static CACHE: CacheBudget = CacheBudget::new(16 * 1024 * 1024);
    let times = PcoVec::<Height, Timestamp, Budgeted>::forced_import_with(
        ImportOptions::new(&database, "timestamps", Version::ONE).with_cache_budget(&CACHE),
    )
    .unwrap();
    let sizes: PcoVec<Height, StoredU64> =
        PcoVec::forced_import(&database, "sizes", Version::ONE).unwrap();
    let weights: PcoVec<Height, Weight> =
        PcoVec::forced_import(&database, "weights", Version::ONE).unwrap();

    let mut expected = None;
    for resident_times in [false, true] {
        let mut samples = Vec::new();
        for round in 0..24 {
            if !resident_times {
                CACHE.clear();
            }
            let started = Instant::now();
            let timestamps = times.collect_range(Height::ZERO, Height::new(ROWS));
            let window = BlockWindow::from_timestamps(
                Height::ZERO,
                Height::new(ROWS),
                TimePeriod::All,
                &timestamps,
            )
            .unwrap();
            let values = window.read(&sizes).unwrap();
            let weighted = window.read(&weights).unwrap();
            // Same output projection as block_sizes_weights; the production
            // grouping, bounded source reads and integer means are called above
            // and below. Query/plugin resolution and HTTP are excluded.
            let (sizes, weights) = window
                .buckets
                .iter()
                .map(|bucket| {
                    (
                        BlockSizeEntry {
                            avg_height: bucket.avg_height,
                            timestamp: bucket.avg_timestamp,
                            avg_size: u64::from(bucket.mean_rounded(&values)),
                        },
                        BlockWeightEntry {
                            avg_height: bucket.avg_height,
                            timestamp: bucket.avg_timestamp,
                            avg_weight: bucket.mean_rounded(&weighted),
                        },
                    )
                })
                .unzip();
            let body = to_vec(&BlockSizesWeights { sizes, weights }).unwrap();
            black_box(RepresentationId::content_hash(&body));
            let elapsed = started.elapsed();
            if let Some(expected) = &expected {
                assert_eq!(&body, expected);
            } else {
                assert!(window.buckets.len() > 6_000);
                expected = Some(body);
            }
            if round >= 4 {
                samples.push(elapsed);
            }
        }
        samples.sort_unstable();
        eprintln!(
            "mining rows={ROWS}, resident timestamps={resident_times}: p50={:?}, p95={:?}, body={} bytes, samples={}",
            samples[10],
            samples[19],
            expected.as_ref().unwrap().len(),
            samples.len()
        );
    }
}
