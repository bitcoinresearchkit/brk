use std::{
    collections::VecDeque,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
};

use brk_rpc::BlockTemplateTx;
use brk_types::{FeeRate, NextBlockHash, Txid, TxidPrefix};
use parking_lot::RwLock;
use rustc_hash::FxHashSet;

use crate::State;

use partition::Partitioner;
use snapshot::{PrefixIndex, builder};

mod partition;
mod snapshot;

pub use brk_types::RecommendedFees;
pub use snapshot::{BlockStats, SnapTx, Snapshot, TxIndex};

const NUM_BLOCKS: usize = 8;
const HISTORY: usize = 10;

#[derive(Default)]
pub struct Rebuilder {
    snapshot: RwLock<Arc<Snapshot>>,
    /// Past block-0 txid sets keyed by `next_block_hash`, oldest first.
    history: RwLock<VecDeque<(NextBlockHash, FxHashSet<Txid>)>>,
    dirty: AtomicBool,
    rebuild_count: AtomicU64,
    skip_clean: AtomicU64,
}

impl Rebuilder {
    /// Mark dirty if the cycle changed mempool state, then rebuild iff
    /// the dirty bit is set. Cycle pacing is the driver loop's job; the
    /// rebuild itself is pure CPU on already-fetched data. The dirty
    /// bit is cleared only after the snapshot is published, so a panic
    /// in `build_snapshot` retries on the next cycle.
    pub fn tick(
        &self,
        lock: &RwLock<State>,
        changed: bool,
        gbt: &[BlockTemplateTx],
        min_fee: FeeRate,
    ) {
        if changed {
            self.dirty.store(true, Ordering::Release);
        }
        if !self.dirty.load(Ordering::Acquire) {
            self.skip_clean.fetch_add(1, Ordering::Relaxed);
            return;
        }
        let snap = Self::build_snapshot(lock, gbt, min_fee);
        let block0_set: FxHashSet<Txid> = snap.blocks[0]
            .iter()
            .map(|idx| snap.txs[idx.as_usize()].txid)
            .collect();
        let next_hash = snap.next_block_hash;
        *self.snapshot.write() = Arc::new(snap);
        self.push_history(next_hash, block0_set);
        self.dirty.store(false, Ordering::Release);
        self.rebuild_count.fetch_add(1, Ordering::Relaxed);
    }

    fn push_history(&self, hash: NextBlockHash, set: FxHashSet<Txid>) {
        let mut hist = self.history.write();
        hist.retain(|(h, _)| *h != hash);
        hist.push_back((hash, set));
        while hist.len() > HISTORY {
            hist.pop_front();
        }
    }

    /// Past block-0 txid set for `hash`, or `None` if it has aged out
    /// (or was never seen). Used by `block_template_diff` to decide
    /// 200 vs 404.
    pub fn historical_block0(&self, hash: NextBlockHash) -> Option<FxHashSet<Txid>> {
        self.history
            .read()
            .iter()
            .find(|(h, _)| *h == hash)
            .map(|(_, set)| set.clone())
    }

    pub fn rebuild_count(&self) -> u64 {
        self.rebuild_count.load(Ordering::Relaxed)
    }

    pub fn skip_clean_count(&self) -> u64 {
        self.skip_clean.load(Ordering::Relaxed)
    }

    fn build_snapshot(
        lock: &RwLock<State>,
        gbt: &[BlockTemplateTx],
        min_fee: FeeRate,
    ) -> Snapshot {
        let (txs, prefix_to_idx) = {
            let state = lock.read();
            builder::build_txs(&state.txs)
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
}
