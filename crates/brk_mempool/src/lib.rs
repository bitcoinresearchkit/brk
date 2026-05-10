//! Live mempool monitor for the brk indexer.
//!
//! One pull cycle, five steps:
//!
//! 1. [`steps::fetcher::Fetcher`] - one mixed batched RPC for
//!    `getblocktemplate` + `getrawmempool false` + `getmempoolinfo`,
//!    then a single mixed `getmempoolentry`+`getrawtransaction` batch
//!    on new txids only. GBT-only txs are synthesized inline from the
//!    GBT payload so block 0 matches Core's selection exactly without
//!    a follow-up entry fetch that could race the listing.
//! 2. [`steps::preparer::Preparer`] - decode and classify into
//!    `TxsPulled { added, removed }`. Pure CPU.
//! 3. [`steps::applier::Applier`] - apply the diff to
//!    [`state::State`] under a single write lock.
//! 4. [`steps::Prevouts::fill`] - fills `prevout: None` inputs in one
//!    pass, using same-cycle in-mempool parents directly and the
//!    caller-supplied resolver (default: `getrawtransaction`) for
//!    confirmed parents.
//! 5. [`steps::rebuilder::Rebuilder`] - throttled rebuild of the
//!    projected-blocks `Snapshot` from the same-cycle GBT and min fee.

use std::{
    any::Any,
    cmp::Reverse,
    panic::{AssertUnwindSafe, catch_unwind},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

use brk_error::Result;
use brk_rpc::Client;
use brk_types::{
    AddrBytes, AddrMempoolStats, BlockTemplate, BlockTemplateDiff, FeeRate, MempoolBlock,
    MempoolInfo, MempoolRecentTx, NextBlockHash, OutpointPrefix, OutputType, Sats, Timestamp,
    Transaction, TxOut, Txid, TxidPrefix, Vin, Vout,
};
use parking_lot::{RwLock, RwLockReadGuard};
use rustc_hash::{FxHashMap, FxHashSet};
use tracing::error;

mod cluster;
mod cpfp;
mod diagnostics;
mod rbf;
mod state;
pub(crate) mod steps;
pub(crate) mod stores;

pub use diagnostics::MempoolStats;
pub use rbf::{RbfForTx, RbfNode};
pub use steps::Snapshot;
use steps::{Applier, Fetched, Fetcher, Preparer, Prevouts, Rebuilder};
pub(crate) use steps::{BlockStats, RecommendedFees, TxEntry, TxRemoval};
pub(crate) use stores::{TxStore, TxTombstone};

/// Confirmed-parent prevout resolver passed to [`Mempool::update_with`] /
/// [`Mempool::start_with`]. Receives a slice of `(parent_txid, vout)`
/// holes and returns the subset that resolved. Unresolved holes are
/// simply omitted from the map; the next cycle retries automatically.
///
/// Batched so the RPC implementation can pack one round-trip per cycle
/// (deduping by parent txid so a tx with N inputs from one parent costs
/// one fetch); the indexer implementation just loops over local reads.
pub type PrevoutResolver =
    Box<dyn Fn(&[(Txid, Vout)]) -> FxHashMap<(Txid, Vout), TxOut> + Send + Sync>;

pub(crate) use state::State;

/// Cheaply cloneable: clones share one live mempool via `Arc`.
#[derive(Clone)]
pub struct Mempool(Arc<Inner>);

struct Inner {
    client: Client,
    state: RwLock<State>,
    rebuilder: Rebuilder,
    started: AtomicBool,
}

impl Mempool {
    pub fn new(client: &Client) -> Self {
        Self(Arc::new(Inner {
            client: client.clone(),
            state: RwLock::new(State::default()),
            rebuilder: Rebuilder::default(),
            started: AtomicBool::new(false),
        }))
    }

    pub fn info(&self) -> MempoolInfo {
        self.read().info.clone()
    }

    pub fn snapshot(&self) -> Arc<Snapshot> {
        self.0.rebuilder.snapshot()
    }

    /// One-shot diagnostic counters captured under a single read guard.
    pub fn stats(&self) -> MempoolStats {
        MempoolStats::from(self)
    }

    pub(crate) fn rebuilder(&self) -> &Rebuilder {
        &self.0.rebuilder
    }

    pub fn fees(&self) -> RecommendedFees {
        self.snapshot().fees.clone()
    }

    pub fn block_stats(&self) -> Vec<BlockStats> {
        self.snapshot().block_stats.clone()
    }

    pub fn next_block_hash(&self) -> NextBlockHash {
        self.snapshot().next_block_hash
    }

    /// Full projected next block: Core's `getblocktemplate` selection
    /// (block 0) with aggregate stats and full tx bodies in GBT order.
    pub fn block_template(&self) -> BlockTemplate {
        let snap = self.snapshot();
        BlockTemplate {
            hash: snap.next_block_hash,
            stats: snap
                .block_stats
                .first()
                .map(MempoolBlock::from)
                .unwrap_or_default(),
            transactions: self.collect_txs(snap.block0_txids()),
        }
    }

    /// Delta of the projected next block since `since`. `None` when
    /// `since` has aged out of the rebuilder's history (server should
    /// 404 → client falls back to `block_template`). `removed` is just
    /// txids; `added` carries full bodies so clients can patch their
    /// local view in one round trip.
    pub fn block_template_diff(&self, since: NextBlockHash) -> Option<BlockTemplateDiff> {
        let past = self.0.rebuilder.historical_block0(since)?;
        let snap = self.snapshot();
        let current: FxHashSet<Txid> = snap.block0_txids().collect();
        Some(BlockTemplateDiff {
            hash: snap.next_block_hash,
            since,
            added: self.collect_txs(current.difference(&past).copied()),
            removed: past.difference(&current).copied().collect(),
        })
    }

    fn collect_txs(&self, txids: impl IntoIterator<Item = Txid>) -> Vec<Transaction> {
        let state = self.read();
        txids
            .into_iter()
            .filter_map(|txid| state.txs.get(&txid).cloned())
            .collect()
    }

    pub fn addr_state_hash(&self, addr: &AddrBytes) -> u64 {
        self.read().addrs.stats_hash(addr)
    }

    /// Mempool tx spending `(txid, vout)`, or `None`. The spender's
    /// input list is walked to rule out `TxidPrefix` collisions.
    pub fn lookup_spender(&self, txid: &Txid, vout: Vout) -> Option<(Txid, Vin)> {
        let key = OutpointPrefix::new(TxidPrefix::from(txid), vout);
        let state = self.read();
        let spender_prefix = state.outpoint_spends.get(&key)?;
        let spender = state.txs.record_by_prefix(&spender_prefix)?;
        let vin_pos = spender
            .tx
            .input
            .iter()
            .position(|inp| inp.txid == *txid && inp.vout == vout)?;
        Some((spender.entry.txid, Vin::from(vin_pos)))
    }

    pub(crate) fn read(&self) -> RwLockReadGuard<'_, State> {
        self.0.state.read()
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
        self.read().graveyard.get_vanished(txid).map(|t| f(&t.tx))
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
        let state = self.read();
        let Some(entry) = state.addrs.get(addr) else {
            return vec![];
        };
        let mut ordered: Vec<(Timestamp, &Transaction)> = entry
            .txids
            .iter()
            .filter_map(|txid| {
                let record = state.txs.record_by_prefix(&TxidPrefix::from(txid))?;
                Some((record.entry.first_seen, &record.tx))
            })
            .collect();
        ordered.sort_unstable_by_key(|b| Reverse(b.0));
        ordered
            .into_iter()
            .take(limit)
            .map(|(_, tx)| tx.clone())
            .collect()
    }

    /// Apply `f` to an iterator over `(value, output_type)` for every output
    /// of every live mempool tx. The lock is held for the duration of the call.
    pub fn process_live_outputs<R>(
        &self,
        f: impl FnOnce(&mut dyn Iterator<Item = (Sats, OutputType)>) -> R,
    ) -> R {
        let state = self.read();
        let mut iter = state
            .txs
            .values()
            .flat_map(|tx| &tx.output)
            .map(|txout| (txout.value, txout.type_()));
        f(&mut iter)
    }

    /// Effective fee rate for a live tx: snapshot's linearized chunk
    /// rate. Falls back to `fee/vsize` for txs added since the latest
    /// snapshot was built (apply -> same-cycle tick gap).
    pub fn live_effective_fee_rate(&self, prefix: &TxidPrefix) -> Option<FeeRate> {
        if let Some(rate) = self.snapshot().chunk_rate_for(prefix) {
            return Some(rate);
        }
        self.read()
            .txs
            .entry_by_prefix(prefix)
            .map(|e| e.fee_rate())
    }

    /// Linearized chunk rate captured at burial - same value
    /// `live_effective_fee_rate` returned while the tx was alive, so an
    /// evicted RBF predecessor reports the package-effective rate it
    /// had in the mempool, not a misleading isolated `fee/vsize`.
    pub fn graveyard_fee_rate(&self, txid: &Txid) -> Option<FeeRate> {
        self.read().graveyard.get(txid).map(|tomb| tomb.chunk_rate)
    }

    /// `first_seen` Unix-second timestamps for `txids`, in input order.
    /// Returns 0 for unknown txids. `Vanished` tombstones fall back to
    /// the buried entry's `first_seen` to avoid flicker between drop
    /// and indexer catch-up.
    pub fn transaction_times(&self, txids: &[Txid]) -> Vec<u64> {
        let state = self.read();
        txids
            .iter()
            .map(|txid| state.first_seen(txid).map_or(0, u64::from))
            .collect()
    }

    /// Infinite update loop with a 500ms interval. Resolves
    /// confirmed-parent prevouts via the default `getrawtransaction`
    /// resolver; requires bitcoind started with `txindex=1`.
    pub fn start(&self) {
        self.start_with(Prevouts::rpc_resolver(self.0.client.clone()));
    }

    /// Variant of `start` that uses a caller-supplied resolver for
    /// confirmed-parent prevouts (typically backed by an indexer).
    /// Each cycle is wrapped in `catch_unwind` so a panic doesn't
    /// freeze the snapshot; `parking_lot` locks don't poison.
    ///
    /// Sleep is `PERIOD - work_duration`, so a 350ms cycle followed by
    /// a 100ms cycle still ticks roughly every `PERIOD`. When work
    /// overruns `PERIOD`, the next cycle starts immediately.
    pub fn start_with<F>(&self, resolver: F)
    where
        F: Fn(&[(Txid, Vout)]) -> FxHashMap<(Txid, Vout), TxOut>,
    {
        if self
            .0
            .started
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            panic!("Mempool::start_with already running on this instance");
        }
        const PERIOD: Duration = Duration::from_millis(500);
        loop {
            let started = Instant::now();
            let outcome = catch_unwind(AssertUnwindSafe(|| {
                if let Err(e) = self.update_with(&resolver) {
                    error!("update failed: {e}");
                }
            }));
            if let Err(payload) = outcome {
                error!(
                    "mempool update panicked, continuing loop: {}",
                    panic_msg(&payload)
                );
            }
            if let Some(rest) = PERIOD.checked_sub(started.elapsed()) {
                thread::sleep(rest);
            }
        }
    }

    /// One sync cycle: fetch, prepare, apply, fill prevouts, maybe
    /// rebuild. The resolver MUST resolve confirmed prevouts only;
    /// mempool-to-mempool chains are wired internally and the
    /// resolver is never called for them.
    fn update_with<F>(&self, resolver: F) -> Result<()>
    where
        F: Fn(&[(Txid, Vout)]) -> FxHashMap<(Txid, Vout), TxOut>,
    {
        let Inner {
            client,
            state,
            rebuilder,
            ..
        } = &*self.0;

        let Fetched {
            live_txids,
            new_entries,
            new_txs,
            gbt_txids,
            min_fee,
        } = Fetcher::fetch(client, state)?;
        let pulled = Preparer::prepare(&live_txids, new_entries, new_txs, state);
        let changed = Applier::apply(state, rebuilder, pulled);
        Prevouts::fill(state, resolver);
        rebuilder.tick(state, changed, &gbt_txids, min_fee);

        Ok(())
    }
}

fn panic_msg(payload: &(dyn Any + Send)) -> &str {
    payload
        .downcast_ref::<&'static str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
        .unwrap_or("<non-string panic payload>")
}
