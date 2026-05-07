//! Live mempool monitor for the brk indexer.
//!
//! One pull cycle, five pipeline steps:
//!
//! 1. [`steps::fetcher::Fetcher`] - three batched RPCs (verbose
//!    listing, raw txs for new entries, raw txs for confirmed parents).
//! 2. [`steps::preparer::Preparer`] - decode and classify into
//!    `TxsPulled { added, removed }`. Pure CPU.
//! 3. [`steps::applier::Applier`] - apply the diff to
//!    [`stores::state::MempoolState`] under brief write locks.
//! 4. [`steps::resolver::Resolver`] - fill prevouts from the live
//!    mempool, or via a caller-supplied external resolver.
//! 5. [`steps::rebuilder::Rebuilder`] - throttled rebuild of the
//!    projected-blocks `Snapshot`.

use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    sync::Arc,
    thread,
    time::Duration,
};

use brk_error::Result;
use brk_rpc::Client;
use brk_types::{
    AddrBytes, AddrMempoolStats, FeeRate, MempoolInfo, MempoolRecentTx, OutpointPrefix, OutputType,
    Sats, Timestamp, Transaction, TxOut, Txid, TxidPrefix, Vin, Vout,
};
use parking_lot::RwLockReadGuard;
use tracing::error;

pub mod cluster;
mod cpfp;
mod rbf;
mod stats;
pub(crate) mod steps;
pub(crate) mod stores;
#[cfg(test)]
mod tests;

pub use rbf::{RbfForTx, RbfNode};
pub use stats::MempoolStats;
use steps::{Applier, Fetcher, Preparer, Rebuilder, Resolver};
pub use steps::{BlockStats, RecommendedFees, Snapshot, TxEntry, TxRemoval};
use stores::{AddrTracker, MempoolState};
pub use stores::{EntryPool, TxGraveyard, TxStore, TxTombstone};

/// Cheaply cloneable: clones share one live mempool via `Arc`.
#[derive(Clone)]
pub struct Mempool(Arc<Inner>);

struct Inner {
    client: Client,
    state: MempoolState,
    rebuilder: Rebuilder,
}

impl Mempool {
    pub fn new(client: &Client) -> Self {
        Self(Arc::new(Inner {
            client: client.clone(),
            state: MempoolState::default(),
            rebuilder: Rebuilder::default(),
        }))
    }

    pub fn info(&self) -> MempoolInfo {
        self.0.state.info.read().clone()
    }

    pub fn snapshot(&self) -> Arc<Snapshot> {
        self.0.rebuilder.snapshot()
    }

    pub fn rebuild_count(&self) -> u64 {
        self.0.rebuilder.rebuild_count()
    }

    pub fn skip_counts(&self) -> (u64, u64) {
        self.0.rebuilder.skip_counts()
    }

    pub fn fees(&self) -> RecommendedFees {
        self.snapshot().fees.clone()
    }

    pub fn block_stats(&self) -> Vec<BlockStats> {
        self.snapshot().block_stats.clone()
    }

    pub fn next_block_hash(&self) -> u64 {
        self.snapshot().next_block_hash
    }

    pub fn addr_state_hash(&self, addr: &AddrBytes) -> u64 {
        self.0.state.addrs.read().stats_hash(addr)
    }

    /// Mempool tx spending `(txid, vout)`, or `None`. The spender's
    /// input list is walked to rule out `TxidPrefix` collisions.
    pub fn lookup_spender(&self, txid: &Txid, vout: Vout) -> Option<(Txid, Vin)> {
        let key = OutpointPrefix::new(TxidPrefix::from(txid), vout);
        let txs = self.txs();
        let entries = self.entries();
        let outpoint_spends = self.0.state.outpoint_spends.read();
        let idx = outpoint_spends.get(&key)?;
        let spender_txid = entries.slot(idx)?.txid;
        let spender_tx = txs.get(&spender_txid)?;
        let vin_pos = spender_tx
            .input
            .iter()
            .position(|inp| inp.txid == *txid && inp.vout == vout)?;
        Some((spender_txid, Vin::from(vin_pos)))
    }

    pub(crate) fn txs(&self) -> RwLockReadGuard<'_, TxStore> {
        self.0.state.txs.read()
    }

    pub(crate) fn entries(&self) -> RwLockReadGuard<'_, EntryPool> {
        self.0.state.entries.read()
    }

    pub(crate) fn addrs(&self) -> RwLockReadGuard<'_, AddrTracker> {
        self.0.state.addrs.read()
    }

    pub(crate) fn graveyard(&self) -> RwLockReadGuard<'_, TxGraveyard> {
        self.0.state.graveyard.read()
    }

    pub fn contains_txid(&self, txid: &Txid) -> bool {
        self.txs().contains(txid)
    }

    /// Apply `f` to the live tx body if present.
    pub fn with_tx<R>(&self, txid: &Txid, f: impl FnOnce(&Transaction) -> R) -> Option<R> {
        self.txs().get(txid).map(f)
    }

    /// Apply `f` to a `Vanished` tombstone's tx body if present.
    /// `Replaced` tombstones return `None` because the tx will not confirm.
    pub fn with_vanished_tx<R>(
        &self,
        txid: &Txid,
        f: impl FnOnce(&Transaction) -> R,
    ) -> Option<R> {
        let graveyard = self.graveyard();
        let tomb = graveyard.get(txid)?;
        matches!(tomb.reason(), TxRemoval::Vanished).then(|| f(&tomb.tx))
    }

    /// Snapshot of all live mempool txids.
    pub fn txids(&self) -> Vec<Txid> {
        self.txs().keys().cloned().collect()
    }

    /// Snapshot of recent live txs.
    pub fn recent_txs(&self) -> Vec<MempoolRecentTx> {
        self.txs().recent().to_vec()
    }

    /// Per-address mempool stats. `None` if the address has no live mempool activity.
    pub fn addr_stats(&self, addr: &AddrBytes) -> Option<AddrMempoolStats> {
        self.addrs().get(addr).map(|e| e.stats.clone())
    }

    /// Live mempool txs touching `addr`, newest first by `first_seen`,
    /// capped at `limit`. Returns owned `Transaction`s.
    pub fn addr_txs(&self, addr: &AddrBytes, limit: usize) -> Vec<Transaction> {
        let txs = self.txs();
        let addrs = self.addrs();
        let entries = self.entries();
        let Some(entry) = addrs.get(addr) else {
            return vec![];
        };
        let mut ordered: Vec<(Timestamp, &Txid)> = entry
            .txids
            .iter()
            .map(|txid| {
                let first_seen = entries
                    .get(&TxidPrefix::from(txid))
                    .map(|e| e.first_seen)
                    .unwrap_or_default();
                (first_seen, txid)
            })
            .collect();
        ordered.sort_unstable_by_key(|b| std::cmp::Reverse(b.0));
        ordered
            .into_iter()
            .filter_map(|(_, txid)| txs.get(txid).cloned())
            .take(limit)
            .collect()
    }

    /// Apply `f` to an iterator over `(value, output_type)` for every output
    /// of every live mempool tx. The lock is held for the duration of the call.
    pub fn process_live_outputs<R>(
        &self,
        f: impl FnOnce(&mut dyn Iterator<Item = (Sats, OutputType)>) -> R,
    ) -> R {
        let txs = self.txs();
        let mut iter = txs
            .values()
            .flat_map(|tx| &tx.output)
            .map(|txout| (txout.value, txout.type_()));
        f(&mut iter)
    }

    /// Effective fee rate for a live tx: seed's snapshot chunk rate,
    /// falling back to the entry's `fee/vsize` if not yet in the snapshot.
    pub fn live_effective_fee_rate(&self, prefix: &TxidPrefix) -> Option<FeeRate> {
        let entries = self.entries();
        if let Some(seed_idx) = entries.idx_of(prefix)
            && let Some(rate) = self.snapshot().chunk_rate_of(seed_idx)
        {
            return Some(rate);
        }
        entries.get(prefix).map(|e| e.fee_rate())
    }

    /// Fee rate snapshotted into a graveyard tomb at burial.
    pub fn graveyard_fee_rate(&self, txid: &Txid) -> Option<FeeRate> {
        self.graveyard()
            .get(txid)
            .map(|tomb| tomb.entry.fee_rate())
    }

    /// `first_seen` Unix-second timestamps for `txids`, in input order.
    /// Returns 0 for unknown txids. `Vanished` tombstones fall back to
    /// the buried entry's `first_seen` to avoid flicker between drop
    /// and indexer catch-up.
    pub fn transaction_times(&self, txids: &[Txid]) -> Vec<u64> {
        let entries = self.entries();
        let graveyard = self.graveyard();
        txids
            .iter()
            .map(|txid| {
                if let Some(e) = entries.get(&TxidPrefix::from(txid)) {
                    return u64::from(e.first_seen);
                }
                if let Some(tomb) = graveyard.get(txid)
                    && matches!(tomb.reason(), TxRemoval::Vanished)
                {
                    return u64::from(tomb.entry.first_seen);
                }
                0
            })
            .collect()
    }

    /// Infinite update loop with a 1 second interval.
    pub fn start(&self) {
        self.start_with(|| {});
    }

    /// Variant of `start` that runs `after_update` after every cycle.
    /// Both steps are wrapped in `catch_unwind` so a panic doesn't
    /// freeze the snapshot; `parking_lot` locks don't poison.
    pub fn start_with(&self, mut after_update: impl FnMut()) {
        loop {
            let outcome = catch_unwind(AssertUnwindSafe(|| {
                if let Err(e) = self.update() {
                    error!("update failed: {e}");
                }
                after_update();
            }));
            if let Err(payload) = outcome {
                let msg = if let Some(s) = payload.downcast_ref::<&'static str>() {
                    (*s).to_string()
                } else if let Some(s) = payload.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "<non-string panic payload>".to_string()
                };
                error!("mempool update panicked, continuing loop: {msg}");
            }
            thread::sleep(Duration::from_secs(1));
        }
    }

    /// Fill remaining `prevout == None` inputs via an external
    /// resolver (typically the indexer for confirmed parents).
    /// In-mempool parents are filled automatically each cycle.
    pub fn fill_prevouts<F>(&self, resolver: F) -> bool
    where
        F: Fn(&Txid, Vout) -> Option<TxOut>,
    {
        Resolver::resolve_external(&self.0.state, resolver)
    }

    /// One sync cycle: fetch, prepare, apply, resolve, maybe rebuild.
    pub fn update(&self) -> Result<()> {
        let Inner {
            client,
            state,
            rebuilder,
        } = &*self.0;

        let fetched = Fetcher::fetch(client, state)?;
        let pulled = Preparer::prepare(fetched, state);
        let changed = Applier::apply(state, pulled);
        Resolver::resolve_in_mempool(state);
        rebuilder.tick(client, state, changed);

        Ok(())
    }

    pub(crate) fn state(&self) -> &MempoolState {
        &self.0.state
    }
}
