//! Live mempool monitor for the brk indexer.
//!
//! One pull cycle, five steps:
//!
//! 1. [`steps::fetcher::Fetcher`] - one mixed batched RPC for
//!    `getrawmempool verbose` + `getblocktemplate` + `getmempoolinfo`,
//!    then a second batch for `getrawtransaction` on new entries. The
//!    GBT is validated to be a subset of the verbose listing; on
//!    mismatch the cycle is skipped.
//! 2. [`steps::preparer::Preparer`] - decode and classify into
//!    `TxsPulled { added, removed }`. Pure CPU.
//! 3. [`steps::applier::Applier`] - apply the diff to
//!    [`inner::MempoolInner`] under a single write lock.
//! 4. [`prevouts::fill`] - fills `prevout: None` inputs in one pass,
//!    using same-cycle in-mempool parents directly and the
//!    caller-supplied resolver (default: `getrawtransaction`) for
//!    confirmed parents.
//! 5. [`steps::rebuilder::Rebuilder`] - throttled rebuild of the
//!    projected-blocks `Snapshot` from the same-cycle GBT and min fee.

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
use parking_lot::{RwLock, RwLockReadGuard};
use tracing::error;

mod cpfp;
mod inner;
mod prevouts;
mod rbf;
pub(crate) mod steps;
pub(crate) mod stores;

pub use rbf::{RbfForTx, RbfNode};
use steps::{Applier, Fetched, Fetcher, Preparer, Rebuilder};
pub use steps::{BlockStats, RecommendedFees, Snapshot, TxEntry, TxRemoval};
pub use stores::{TxGraveyard, TxStore, TxTombstone};

/// Confirmed-parent prevout resolver passed to [`Mempool::update_with`] /
/// [`Mempool::start_with`]. Receives `(parent_txid, vout)`, returns the
/// `TxOut` if the parent is reachable, `None` otherwise.
pub type PrevoutResolver = Box<dyn Fn(&Txid, Vout) -> Option<TxOut> + Send + Sync>;

pub(crate) use inner::MempoolInner;

/// Cheaply cloneable: clones share one live mempool via `Arc`.
#[derive(Clone)]
pub struct Mempool(Arc<Inner>);

struct Inner {
    client: Client,
    lock: RwLock<MempoolInner>,
    rebuilder: Rebuilder,
}

impl Mempool {
    pub fn new(client: &Client) -> Self {
        Self(Arc::new(Inner {
            client: client.clone(),
            lock: RwLock::new(MempoolInner::default()),
            rebuilder: Rebuilder::default(),
        }))
    }

    pub fn info(&self) -> MempoolInfo {
        self.read().info.clone()
    }

    pub fn snapshot(&self) -> Arc<Snapshot> {
        self.0.rebuilder.snapshot()
    }

    pub fn rebuild_count(&self) -> u64 {
        self.0.rebuilder.rebuild_count()
    }

    pub fn skip_clean_count(&self) -> u64 {
        self.0.rebuilder.skip_clean_count()
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
        self.read().addrs.stats_hash(addr)
    }

    /// Mempool tx spending `(txid, vout)`, or `None`. The spender's
    /// input list is walked to rule out `TxidPrefix` collisions.
    pub fn lookup_spender(&self, txid: &Txid, vout: Vout) -> Option<(Txid, Vin)> {
        let key = OutpointPrefix::new(TxidPrefix::from(txid), vout);
        let inner = self.read();
        let spender_prefix = inner.outpoint_spends.get(&key)?;
        let spender = inner.txs.record_by_prefix(&spender_prefix)?;
        let vin_pos = spender
            .tx
            .input
            .iter()
            .position(|inp| inp.txid == *txid && inp.vout == vout)?;
        Some((spender.entry.txid, Vin::from(vin_pos)))
    }

    pub(crate) fn read(&self) -> RwLockReadGuard<'_, MempoolInner> {
        self.0.lock.read()
    }

    pub fn tx_count(&self) -> usize {
        self.read().txs.len()
    }

    pub fn unresolved_count(&self) -> usize {
        self.read().txs.unresolved().len()
    }

    pub fn addr_count(&self) -> usize {
        self.read().addrs.len()
    }

    pub fn outpoint_spend_count(&self) -> usize {
        self.read().outpoint_spends.len()
    }

    pub fn graveyard_tombstone_count(&self) -> usize {
        self.read().graveyard.tombstones_len()
    }

    pub fn graveyard_order_count(&self) -> usize {
        self.read().graveyard.order_len()
    }

    pub fn contains_txid(&self, txid: &Txid) -> bool {
        self.read().txs.contains(txid)
    }

    /// Apply `f` to the live tx body if present.
    pub fn with_tx<R>(&self, txid: &Txid, f: impl FnOnce(&Transaction) -> R) -> Option<R> {
        self.read().txs.get(txid).map(f)
    }

    /// Apply `f` to a `Vanished` tombstone's tx body if present.
    /// `Replaced` tombstones return `None` because the tx will not confirm.
    pub fn with_vanished_tx<R>(&self, txid: &Txid, f: impl FnOnce(&Transaction) -> R) -> Option<R> {
        let inner = self.read();
        let tomb = inner.graveyard.get(txid)?;
        matches!(tomb.reason(), TxRemoval::Vanished).then(|| f(&tomb.tx))
    }

    /// Snapshot of all live mempool txids.
    pub fn txids(&self) -> Vec<Txid> {
        self.read().txs.txids().copied().collect()
    }

    /// Snapshot of recent live txs.
    pub fn recent_txs(&self) -> Vec<MempoolRecentTx> {
        self.read().txs.recent().to_vec()
    }

    /// Per-address mempool stats. `None` if the address has no live mempool activity.
    pub fn addr_stats(&self, addr: &AddrBytes) -> Option<AddrMempoolStats> {
        self.read().addrs.get(addr).map(|e| e.stats.clone())
    }

    /// Live mempool txs touching `addr`, newest first by `first_seen`,
    /// capped at `limit`. Returns owned `Transaction`s.
    pub fn addr_txs(&self, addr: &AddrBytes, limit: usize) -> Vec<Transaction> {
        let inner = self.read();
        let Some(entry) = inner.addrs.get(addr) else {
            return vec![];
        };
        let mut ordered: Vec<(Timestamp, &Txid)> = entry
            .txids
            .iter()
            .map(|txid| {
                let first_seen = inner
                    .txs
                    .entry(txid)
                    .map(|e| e.first_seen)
                    .unwrap_or_default();
                (first_seen, txid)
            })
            .collect();
        ordered.sort_unstable_by_key(|b| std::cmp::Reverse(b.0));
        ordered
            .into_iter()
            .filter_map(|(_, txid)| inner.txs.get(txid).cloned())
            .take(limit)
            .collect()
    }

    /// Apply `f` to an iterator over `(value, output_type)` for every output
    /// of every live mempool tx. The lock is held for the duration of the call.
    pub fn process_live_outputs<R>(
        &self,
        f: impl FnOnce(&mut dyn Iterator<Item = (Sats, OutputType)>) -> R,
    ) -> R {
        let inner = self.read();
        let mut iter = inner
            .txs
            .values()
            .flat_map(|tx| &tx.output)
            .map(|txout| (txout.value, txout.type_()));
        f(&mut iter)
    }

    /// Effective fee rate for a live tx: snapshot's chunk rate when
    /// the tx is in the latest snapshot, falling back to the entry's
    /// `fee/vsize` if not yet ingested.
    pub fn live_effective_fee_rate(&self, prefix: &TxidPrefix) -> Option<FeeRate> {
        if let Some(rate) = self.snapshot().chunk_rate_for(prefix) {
            return Some(rate);
        }
        self.read()
            .txs
            .entry_by_prefix(prefix)
            .map(|e| e.fee_rate())
    }

    /// Fee rate snapshotted into a graveyard tomb at burial.
    pub fn graveyard_fee_rate(&self, txid: &Txid) -> Option<FeeRate> {
        self.read()
            .graveyard
            .get(txid)
            .map(|tomb| tomb.entry.fee_rate())
    }

    /// `first_seen` Unix-second timestamps for `txids`, in input order.
    /// Returns 0 for unknown txids. `Vanished` tombstones fall back to
    /// the buried entry's `first_seen` to avoid flicker between drop
    /// and indexer catch-up.
    pub fn transaction_times(&self, txids: &[Txid]) -> Vec<u64> {
        let inner = self.read();
        txids
            .iter()
            .map(|txid| {
                if let Some(e) = inner.txs.entry(txid) {
                    return u64::from(e.first_seen);
                }
                if let Some(tomb) = inner.graveyard.get(txid)
                    && matches!(tomb.reason(), TxRemoval::Vanished)
                {
                    return u64::from(tomb.entry.first_seen);
                }
                0
            })
            .collect()
    }

    /// Infinite update loop with a 1 second interval. Resolves
    /// confirmed-parent prevouts via the default `getrawtransaction`
    /// resolver; requires bitcoind started with `txindex=1`.
    pub fn start(&self) {
        self.start_with(prevouts::rpc_resolver(self.0.client.clone()));
    }

    /// Variant of `start` that uses a caller-supplied resolver for
    /// confirmed-parent prevouts (typically backed by an indexer).
    /// Each cycle is wrapped in `catch_unwind` so a panic doesn't
    /// freeze the snapshot; `parking_lot` locks don't poison.
    pub fn start_with<F>(&self, resolver: F)
    where
        F: Fn(&Txid, Vout) -> Option<TxOut>,
    {
        loop {
            let outcome = catch_unwind(AssertUnwindSafe(|| {
                if let Err(e) = self.update_with(&resolver) {
                    error!("update failed: {e}");
                }
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

    /// One sync cycle with the default RPC resolver. Equivalent to
    /// `update_with(rpc_resolver)`. Standalone consumers (Core +
    /// `txindex=1`) get a one-line driver loop.
    pub fn update(&self) -> Result<()> {
        self.update_with(prevouts::rpc_resolver(self.0.client.clone()))
    }

    /// One sync cycle: fetch, prepare, apply, fill prevouts, maybe
    /// rebuild. The resolver MUST resolve confirmed prevouts only;
    /// mempool-to-mempool chains are wired internally and the
    /// resolver is never called for them.
    pub fn update_with<F>(&self, resolver: F) -> Result<()>
    where
        F: Fn(&Txid, Vout) -> Option<TxOut>,
    {
        let Inner {
            client,
            lock,
            rebuilder,
        } = &*self.0;

        let Some(Fetched {
            entries_info,
            new_raws,
            gbt,
            min_fee,
        }) = Fetcher::fetch(client, lock)?
        else {
            return Ok(());
        };
        let pulled = Preparer::prepare(entries_info, new_raws, lock);
        let changed = Applier::apply(lock, pulled);
        prevouts::fill(lock, resolver);
        rebuilder.tick(lock, changed, &gbt, min_fee);

        Ok(())
    }
}
