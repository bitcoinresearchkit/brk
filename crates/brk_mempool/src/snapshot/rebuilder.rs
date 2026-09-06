//! # Locking
//!
//! Two locks live on `Rebuilder`: `history` and `snapshot`. Writes always
//! land on `history` first, then `snapshot`, so any `next_block_hash` a
//! reader sees in the published snapshot is already recorded in
//! `historical_block0`. No read path ever holds both, and no path holds
//! a `State` guard together with either Rebuilder lock - the cycle reads
//! `State` once to build the snapshot, then drops it before touching
//! these locks.

use std::{
    collections::VecDeque,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use brk_types::{FeeRate, NextBlockHash, Transaction, Txid, TxidPrefix};
use parking_lot::RwLock;

use crate::State;

use super::{Partitioner, Snapshot, TxIndex};

const NUM_BLOCKS: usize = 8;
const HISTORY: usize = 10;

#[derive(Default)]
pub struct Rebuilder {
    snapshot: RwLock<Arc<Snapshot>>,
    /// Past block-0 body references keyed by content hash, oldest first.
    /// Shared immutable bodies let diffs verify retained entries without
    /// duplicating transaction payloads for every history entry.
    history: RwLock<VecDeque<(NextBlockHash, Arc<[Arc<Transaction>]>)>>,
    rebuild_count: AtomicU64,
}

impl Rebuilder {
    /// Reuse the published projection only when all of its inputs are
    /// unchanged. History is updated before the snapshot Arc is swapped,
    /// so a reader can never observe a hash that has not been recorded.
    pub fn tick(
        &self,
        lock: &RwLock<State>,
        gbt_txids: &[Txid],
        min_fee: FeeRate,
        membership_changed: bool,
    ) {
        let revision = lock.read().txs.content_revision();
        if self.can_reuse(gbt_txids, min_fee, membership_changed)
            && self.snapshot().content_revision == revision
        {
            return;
        }

        let snap = Self::build_snapshot(lock, gbt_txids, min_fee);
        let block0 = snap.template_transactions.clone();
        let next_hash = snap.next_block_hash;

        let mut hist = self.history.write();
        if !snap.template_missing {
            hist.retain(|(h, _)| *h != next_hash);
            hist.push_back((next_hash, block0));
        }
        while hist.len() > HISTORY {
            hist.pop_front();
        }
        drop(hist);

        *self.snapshot.write() = Arc::new(snap);

        self.rebuild_count.fetch_add(1, Ordering::Relaxed);
    }

    fn can_reuse(&self, gbt_txids: &[Txid], min_fee: FeeRate, membership_changed: bool) -> bool {
        if membership_changed {
            return false;
        }
        let snapshot = self.snapshot.read();
        !snapshot.blocks.is_empty()
            && snapshot.min_fee == min_fee
            && snapshot.block0_txids().eq(gbt_txids.iter().copied())
    }

    /// Past block-0 ordered txid list for `hash`, or `None` if it has
    /// aged out (or was never seen). Used by `block_template_diff` to
    /// decide 200 vs 404 and to resolve `Retained(prior_index)` entries.
    pub fn historical_block0(&self, hash: NextBlockHash) -> Option<Arc<[Arc<Transaction>]>> {
        self.history
            .read()
            .iter()
            .find(|(h, _)| *h == hash)
            .map(|(_, block0)| block0.clone())
    }

    pub fn rebuild_count(&self) -> u64 {
        self.rebuild_count.load(Ordering::Relaxed)
    }

    fn build_snapshot(lock: &RwLock<State>, gbt_txids: &[Txid], min_fee: FeeRate) -> Snapshot {
        let (txs, prefix_to_idx, bodies, revision) = {
            let state = lock.read();
            let (txs, prefix_to_idx) = Snapshot::build_txs(&state.txs);
            let bodies: Vec<_> = gbt_txids
                .iter()
                .filter_map(|txid| state.txs.record(txid).map(|record| record.tx.clone()))
                .collect();
            (txs, prefix_to_idx, bodies, state.txs.content_revision())
        };

        let block0: Vec<TxIndex> = gbt_txids
            .iter()
            .filter_map(|txid| {
                prefix_to_idx
                    .get(&TxidPrefix::from(txid))
                    .copied()
                    .filter(|index| txs[index.as_usize()].txid == *txid)
            })
            .collect();
        let mut excluded = vec![0; txs.len()];
        for index in &block0 {
            excluded[index.as_usize()] = 1;
        }
        let rest = Partitioner::partition(&txs, &excluded, NUM_BLOCKS.saturating_sub(1));

        let mut blocks = Vec::with_capacity(NUM_BLOCKS);
        blocks.push(block0);
        blocks.extend(rest);

        let missing = bodies.len() != gbt_txids.len();
        let mut snapshot = Snapshot::build(txs, blocks, prefix_to_idx, min_fee);
        snapshot.set_template(bodies, revision, missing);
        snapshot
    }

    pub fn snapshot(&self) -> Arc<Snapshot> {
        self.snapshot.read().clone()
    }
}

#[cfg(test)]
#[path = "../../tests/unit/snapshot/rebuilder.rs"]
mod tests;
