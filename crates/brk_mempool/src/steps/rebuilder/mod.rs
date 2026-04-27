pub mod block_builder;
pub mod projected_blocks;

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};

use brk_rpc::Client;
use brk_types::FeeRate;
use parking_lot::RwLock;
use tracing::warn;

#[cfg(debug_assertions)]
use self::projected_blocks::verify::Verifier;
use self::{
    block_builder::build_projected_blocks,
    projected_blocks::{BlockStats, RecommendedFees, Snapshot},
};
use crate::stores::EntryPool;

/// Minimum interval between rebuilds (milliseconds).
const MIN_REBUILD_INTERVAL_MS: u64 = 1000;

/// Owns the projected-blocks `Snapshot` and the scheduling around its
/// rebuild.
///
/// Internally stateful: a `dirty` flag the Applier nudges after each
/// state change, a `last_rebuild_ms` throttle so we rebuild at most
/// once per `MIN_REBUILD_INTERVAL_MS` regardless of churn, and the
/// `Snapshot` itself swapped behind a cheap `Arc` so readers clone a
/// pointer, not the vectors inside.
#[derive(Default)]
pub struct Rebuilder {
    snapshot: RwLock<Arc<Snapshot>>,
    dirty: AtomicBool,
    last_rebuild_ms: AtomicU64,
}

impl Rebuilder {
    /// Signal that state has changed and a rebuild is eventually needed.
    pub fn mark_dirty(&self) {
        self.dirty.store(true, Ordering::Release);
    }

    /// Rebuild iff dirty and enough time has passed since the last
    /// run. Takes a short read lock on `entries` while building and
    /// a short write lock on the internal snapshot at swap time.
    pub fn tick(&self, client: &Client, entries: &RwLock<EntryPool>) {
        if !self.dirty.load(Ordering::Acquire) {
            return;
        }

        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let last = self.last_rebuild_ms.load(Ordering::Acquire);
        if now_ms.saturating_sub(last) < MIN_REBUILD_INTERVAL_MS {
            return;
        }

        if self
            .last_rebuild_ms
            .compare_exchange(last, now_ms, Ordering::AcqRel, Ordering::Relaxed)
            .is_err()
        {
            return;
        }

        self.dirty.store(false, Ordering::Release);

        let min_fee = client.get_mempool_min_fee().unwrap_or_else(|e| {
            warn!("getmempoolinfo failed, falling back to FeeRate::MIN: {e}");
            FeeRate::MIN
        });

        let built = {
            let entries = entries.read();
            let entries_slice = entries.entries();
            let blocks = build_projected_blocks(entries_slice);

            #[cfg(debug_assertions)]
            Verifier::check(client, &blocks, entries_slice);
            #[cfg(not(debug_assertions))]
            let _ = client;

            Snapshot::build(blocks, entries_slice, min_fee)
        };

        *self.snapshot.write() = Arc::new(built);
    }

    /// Cheap: reader clones an `Arc` pointer and releases the lock.
    fn current(&self) -> Arc<Snapshot> {
        self.snapshot.read().clone()
    }

    pub fn snapshot(&self) -> Arc<Snapshot> {
        self.current()
    }

    pub fn fees(&self) -> RecommendedFees {
        self.current().fees.clone()
    }

    pub fn block_stats(&self) -> Vec<BlockStats> {
        self.current().block_stats.clone()
    }

    pub fn next_block_hash(&self) -> u64 {
        self.current().next_block_hash
    }
}
