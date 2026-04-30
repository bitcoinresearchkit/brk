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

use std::{sync::Arc, thread, time::Duration};

use brk_error::Result;
use brk_rpc::Client;
use brk_types::{AddrBytes, MempoolInfo, TxOut, Txid, Vout};
use parking_lot::RwLockReadGuard;
use tracing::error;

mod cpfp;
pub(crate) mod steps;
pub(crate) mod stores;
#[cfg(test)]
mod tests;

use steps::{Applier, Fetcher, Preparer, Rebuilder, Resolver};
pub use steps::{BlkIndex, BlockStats, RecommendedFees, Snapshot, TxEntry, TxRemoval};
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

    pub fn txs(&self) -> RwLockReadGuard<'_, TxStore> {
        self.0.state.txs.read()
    }

    pub fn entries(&self) -> RwLockReadGuard<'_, EntryPool> {
        self.0.state.entries.read()
    }

    pub fn addrs(&self) -> RwLockReadGuard<'_, AddrTracker> {
        self.0.state.addrs.read()
    }

    pub fn graveyard(&self) -> RwLockReadGuard<'_, TxGraveyard> {
        self.0.state.graveyard.read()
    }

    /// Infinite update loop with a 1 second interval.
    pub fn start(&self) {
        self.start_with(|| {});
    }

    /// Variant of `start` that runs `after_update` after every cycle.
    pub fn start_with(&self, mut after_update: impl FnMut()) {
        loop {
            if let Err(e) = self.update() {
                error!("update failed: {e}");
            }
            after_update();
            thread::sleep(Duration::from_secs(1));
        }
    }

    /// Fill remaining `prevout == None` inputs via an external
    /// resolver (typically the brk indexer for confirmed parents).
    /// Same-cycle in-mempool parents are filled automatically by
    /// `Resolver::resolve_in_mempool` after each `Applier::apply`.
    pub fn fill_prevouts<F>(&self, resolver: F) -> bool
    where
        F: Fn(&Txid, Vout) -> Option<TxOut>,
    {
        Resolver::resolve_external(&self.0.state, resolver)
    }

    /// One sync cycle: fetch, prepare, apply, resolve, maybe rebuild.
    pub fn update(&self) -> Result<()> {
        let Inner { client, state, rebuilder } = &*self.0;

        let fetched = Fetcher::fetch(client, state)?;
        let pulled = Preparer::prepare(fetched, state);
        let changed = Applier::apply(state, pulled);
        Resolver::resolve_in_mempool(state);
        rebuilder.tick(client, state, changed);

        Ok(())
    }
}
