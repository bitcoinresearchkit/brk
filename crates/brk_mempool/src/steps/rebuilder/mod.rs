use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use brk_rpc::Client;
use brk_types::FeeRate;
use parking_lot::{Mutex, RwLock};
use tracing::warn;

use graph::Graph;
use linearize::Linearizer;
use partition::Partitioner;
#[cfg(debug_assertions)]
use verify::Verifier;
use crate::stores::MempoolState;

pub(crate) mod graph;
pub(crate) mod linearize;
mod partition;
mod snapshot;
#[cfg(debug_assertions)]
mod verify;

pub use brk_types::RecommendedFees;
pub use snapshot::{BlkIndex, BlockStats, Snapshot};

const MIN_REBUILD_INTERVAL: Duration = Duration::from_secs(1);
const NUM_BLOCKS: usize = 8;

#[derive(Default)]
pub struct Rebuilder {
    snapshot: RwLock<Arc<Snapshot>>,
    dirty: AtomicBool,
    last_rebuild: Mutex<Option<Instant>>,
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
    }

    fn build_snapshot(client: &Client, state: &MempoolState) -> Snapshot {
        let min_fee = Self::fetch_min_fee(client);
        let entries = state.entries.read();
        let entries_slice = entries.entries();

        let nodes = Graph::build(entries_slice);
        let packages = Linearizer::linearize(&nodes);
        let blocks = Partitioner::partition(packages, NUM_BLOCKS);

        #[cfg(debug_assertions)]
        Verifier::check(client, &blocks, entries_slice);

        Snapshot::build(blocks, entries_slice, min_fee)
    }

    pub fn snapshot(&self) -> Arc<Snapshot> {
        self.snapshot.read().clone()
    }

    fn mark_dirty(&self, changed: bool) {
        if changed {
            self.dirty.store(true, Ordering::Release);
        }
    }

    /// Returns true iff dirty and the throttle window has elapsed. On
    /// success, clears the dirty bit and starts a new throttle window;
    /// on failure, leaves all state untouched so the next cycle can
    /// retry.
    fn try_claim_rebuild(&self) -> bool {
        if !self.dirty.load(Ordering::Acquire) {
            return false;
        }
        let mut last = self.last_rebuild.lock();
        if last.is_some_and(|t| t.elapsed() < MIN_REBUILD_INTERVAL) {
            return false;
        }
        *last = Some(Instant::now());
        self.dirty.store(false, Ordering::Release);
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
