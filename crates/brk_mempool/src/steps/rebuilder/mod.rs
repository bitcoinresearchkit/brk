use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
};

use brk_rpc::BlockTemplateTx;
use brk_types::{FeeRate, TxidPrefix};
use parking_lot::RwLock;
use rustc_hash::FxHashSet;

use crate::inner::MempoolInner;

use partition::Partitioner;
use snapshot::{PrefixIndex, builder};

mod partition;
mod snapshot;

pub use brk_types::RecommendedFees;
pub use snapshot::{BlockStats, SnapTx, Snapshot, TxIndex};

const NUM_BLOCKS: usize = 8;

#[derive(Default)]
pub struct Rebuilder {
    snapshot: RwLock<Arc<Snapshot>>,
    dirty: AtomicBool,
    rebuild_count: AtomicU64,
    skip_clean: AtomicU64,
}

impl Rebuilder {
    /// Mark dirty if the cycle changed mempool state, then rebuild iff
    /// the dirty bit is set. Cycle pacing is the driver loop's job; the
    /// rebuild itself is pure CPU on already-fetched data.
    pub fn tick(
        &self,
        lock: &RwLock<MempoolInner>,
        changed: bool,
        gbt: &[BlockTemplateTx],
        min_fee: FeeRate,
    ) {
        if changed {
            self.dirty.store(true, Ordering::Release);
        }
        if !self.try_claim_rebuild() {
            return;
        }
        *self.snapshot.write() = Arc::new(Self::build_snapshot(lock, gbt, min_fee));
        self.dirty.store(false, Ordering::Release);
        self.rebuild_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn rebuild_count(&self) -> u64 {
        self.rebuild_count.load(Ordering::Relaxed)
    }

    pub fn skip_clean_count(&self) -> u64 {
        self.skip_clean.load(Ordering::Relaxed)
    }

    fn build_snapshot(
        lock: &RwLock<MempoolInner>,
        gbt: &[BlockTemplateTx],
        min_fee: FeeRate,
    ) -> Snapshot {
        let (txs, prefix_to_idx) = {
            let inner = lock.read();
            builder::build_txs(&inner.txs)
        };

        let block0 = Self::block_from_gbt(gbt, &prefix_to_idx);
        let excluded: FxHashSet<TxIndex> = block0.iter().copied().collect();
        let rest = Partitioner::partition(&txs, &excluded, NUM_BLOCKS.saturating_sub(1));

        let mut blocks = Vec::with_capacity(NUM_BLOCKS);
        blocks.push(block0);
        blocks.extend(rest);

        Snapshot::build(txs, blocks, prefix_to_idx, min_fee)
    }

    /// Block 0 from `getblocktemplate`: Core's actual selection. Maps
    /// each GBT txid back to its `TxIndex` via the per-build prefix
    /// index. Fetcher already validated GBT ⊆ verbose mempool, so any
    /// drop here is a same-cycle race and the partitioner picks up the
    /// slack so callers always see eight blocks.
    fn block_from_gbt(gbt: &[BlockTemplateTx], prefix_to_idx: &PrefixIndex) -> Vec<TxIndex> {
        gbt.iter()
            .filter_map(|t| prefix_to_idx.get(&TxidPrefix::from(&t.txid)).copied())
            .collect()
    }

    pub fn snapshot(&self) -> Arc<Snapshot> {
        self.snapshot.read().clone()
    }

    /// True iff dirty. The dirty bit is cleared in `tick` only after
    /// the snapshot is published, so a panic in `build_snapshot`
    /// retries on the next cycle.
    fn try_claim_rebuild(&self) -> bool {
        if !self.dirty.load(Ordering::Acquire) {
            self.skip_clean.fetch_add(1, Ordering::Relaxed);
            return false;
        }
        true
    }
}
