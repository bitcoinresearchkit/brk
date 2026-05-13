use std::{
    collections::VecDeque,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use brk_types::{FeeRate, NextBlockHash, Txid, TxidPrefix};
use parking_lot::RwLock;
use rustc_hash::FxHashSet;

use crate::State;

use partition::Partitioner;
use snapshot::build_txs;

mod partition;
mod snapshot;

pub use brk_types::RecommendedFees;
pub use snapshot::{BlockStats, SnapTx, Snapshot, TxIndex};

const NUM_BLOCKS: usize = 8;
const HISTORY: usize = 10;

#[derive(Default)]
pub struct Rebuilder {
    snapshot: RwLock<Arc<Snapshot>>,
    /// Past block-0 txid lists keyed by `next_block_hash`, oldest first.
    /// Ordered so `block_template_diff` can emit `Retained(prior_index)`
    /// entries that line up with the client's cached prior template.
    history: RwLock<VecDeque<(NextBlockHash, Vec<Txid>)>>,
    rebuild_count: AtomicU64,
}

impl Rebuilder {
    /// Rebuild the snapshot every cycle. The build is pure CPU on
    /// already-fetched data and `min_fee` participates in the result,
    /// so a "skip if no add/remove" gate would freeze the served fees
    /// when Core's `mempoolminfee` drifts on a quiet pool. Cycle pacing
    /// is the driver loop's job.
    pub fn tick(&self, lock: &RwLock<State>, gbt_txids: &[Txid], min_fee: FeeRate) {
        let snap = Self::build_snapshot(lock, gbt_txids, min_fee);
        let block0: Vec<Txid> = snap.block0_txids().collect();
        let next_hash = snap.next_block_hash;
        *self.snapshot.write() = Arc::new(snap);

        let mut hist = self.history.write();
        hist.retain(|(h, _)| *h != next_hash);
        hist.push_back((next_hash, block0));
        while hist.len() > HISTORY {
            hist.pop_front();
        }
        drop(hist);

        self.rebuild_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Past block-0 ordered txid list for `hash`, or `None` if it has
    /// aged out (or was never seen). Used by `block_template_diff` to
    /// decide 200 vs 404 and to resolve `Retained(prior_index)` entries.
    pub fn historical_block0(&self, hash: NextBlockHash) -> Option<Vec<Txid>> {
        self.history
            .read()
            .iter()
            .find(|(h, _)| *h == hash)
            .map(|(_, block0)| block0.clone())
    }

    pub fn rebuild_count(&self) -> u64 {
        self.rebuild_count.load(Ordering::Relaxed)
    }

    fn build_snapshot(
        lock: &RwLock<State>,
        gbt_txids: &[Txid],
        min_fee: FeeRate,
    ) -> Snapshot {
        let (txs, prefix_to_idx) = {
            let state = lock.read();
            build_txs(&state.txs)
        };

        // Block 0 from `getblocktemplate`: Core's actual selection.
        // The Fetcher synthesizes pool entries for GBT txs that aren't
        // already present (using GBT's inline body + stats), so this
        // lookup always resolves and block 0 matches Core exactly.
        // The `filter_map` only drops if a tx was concurrently evicted
        // from `txs` between `build_txs` and the rebuild, which the
        // partitioner backfills so callers still see `NUM_BLOCKS`.
        let block0: Vec<TxIndex> = gbt_txids
            .iter()
            .filter_map(|txid| prefix_to_idx.get(&TxidPrefix::from(txid)).copied())
            .collect();
        let excluded: FxHashSet<TxIndex> = block0.iter().copied().collect();
        let rest = Partitioner::partition(&txs, &excluded, NUM_BLOCKS.saturating_sub(1));

        let mut blocks = Vec::with_capacity(NUM_BLOCKS);
        blocks.push(block0);
        blocks.extend(rest);

        Snapshot::build(txs, blocks, prefix_to_idx, min_fee)
    }

    pub fn snapshot(&self) -> Arc<Snapshot> {
        self.snapshot.read().clone()
    }
}
