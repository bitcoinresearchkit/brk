use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};

use brk_rpc::Client;
use brk_types::FeeRate;
use parking_lot::{Mutex, RwLock};
use tracing::warn;

use crate::stores::MempoolState;
use clusters::build_clusters;
use partition::Partitioner;
#[cfg(debug_assertions)]
use verify::Verifier;

pub(crate) mod clusters;
mod partition;
mod snapshot;
#[cfg(debug_assertions)]
mod verify;

pub use brk_types::RecommendedFees;
pub use snapshot::{BlockStats, Snapshot};

const MIN_REBUILD_INTERVAL: Duration = Duration::from_secs(1);
const NUM_BLOCKS: usize = 8;

#[derive(Default)]
pub struct Rebuilder {
    snapshot: RwLock<Arc<Snapshot>>,
    dirty: AtomicBool,
    last_rebuild: Mutex<Option<Instant>>,
    rebuild_count: AtomicU64,
    skip_throttled: AtomicU64,
    skip_clean: AtomicU64,
}

impl Rebuilder {
    /// Mark dirty if the cycle changed mempool state, then rebuild iff
    /// the throttle window has elapsed. Marking is sticky: a throttled
    /// `changed=true` cycle keeps the bit set so a later quiet cycle
    /// can still trigger the rebuild.
    pub fn tick(&self, client: &Client, state: &MempoolState, changed: bool) {
        self.mark_dirty(changed);
        if !self.try_claim_rebuild() {
            return;
        }
        self.publish(Self::build_snapshot(client, state));
        self.dirty.store(false, Ordering::Release);
        self.rebuild_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn rebuild_count(&self) -> u64 {
        self.rebuild_count.load(Ordering::Relaxed)
    }

    pub fn skip_counts(&self) -> (u64, u64) {
        (
            self.skip_clean.load(Ordering::Relaxed),
            self.skip_throttled.load(Ordering::Relaxed),
        )
    }

    fn build_snapshot(client: &Client, state: &MempoolState) -> Snapshot {
        let min_fee = Self::fetch_min_fee(client);
        let entries = state.entries.read();
        let entries_slice = entries.entries();

        let (clusters, cluster_of) = build_clusters(entries_slice);
        let blocks = Partitioner::partition(&clusters, NUM_BLOCKS);

        #[cfg(debug_assertions)]
        Verifier::check(client, &blocks, &clusters, &cluster_of, entries_slice);

        Snapshot::build(clusters, cluster_of, blocks, entries_slice, min_fee)
    }

    pub fn snapshot(&self) -> Arc<Snapshot> {
        self.snapshot.read().clone()
    }

    fn mark_dirty(&self, changed: bool) {
        if changed {
            self.dirty.store(true, Ordering::Release);
        }
    }

    /// True iff dirty and the throttle window has elapsed. The dirty
    /// bit is cleared in `tick` only after `publish` returns, so a
    /// panic in `build_snapshot` retries on the next cycle.
    fn try_claim_rebuild(&self) -> bool {
        if !self.dirty.load(Ordering::Acquire) {
            self.skip_clean.fetch_add(1, Ordering::Relaxed);
            return false;
        }
        let mut last = self.last_rebuild.lock();
        if last.is_some_and(|t| t.elapsed() < MIN_REBUILD_INTERVAL) {
            self.skip_throttled.fetch_add(1, Ordering::Relaxed);
            return false;
        }
        *last = Some(Instant::now());
        true
    }

    fn fetch_min_fee(client: &Client) -> FeeRate {
        client.get_mempool_min_fee().unwrap_or_else(|e| {
            warn!("getmempoolinfo failed, falling back to FeeRate::MIN: {e}");
            FeeRate::MIN
        })
    }

    fn publish(&self, snapshot: Snapshot) {
        *self.snapshot.write() = Arc::new(snapshot);
    }
}
