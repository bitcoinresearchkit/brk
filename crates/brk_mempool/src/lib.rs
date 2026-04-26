//! Live mempool monitor for the brk indexer.
//!
//! One pull cycle, five pipeline steps:
//!
//! 1. [`steps::fetcher::Fetcher`]: three batched RPCs against bitcoind
//!    (verbose listing + raw txs for new entries + raw txs for
//!    confirmed parents). Pure I/O.
//! 2. [`steps::preparer::Preparer`]: turn raw bytes into a typed diff
//!    (`Pulled { added, removed }`), classifying additions as
//!    Fresh or Revived and removals as Replaced or Vanished.
//!    Pure CPU, no locks.
//! 3. [`steps::applier::Applier`]: apply the diff to the five-bucket
//!    [`stores::state::MempoolState`] (info, txs, addrs, entries,
//!    graveyard) under brief write locks.
//! 4. [`steps::resolver::Resolver`]: fill prevouts whose parents are
//!    in the live mempool (run after every successful apply)
//!    or via an external resolver supplied by the caller
//!    (typically the brk indexer for confirmed parents).
//! 5. [`steps::rebuilder::Rebuilder`]: throttled rebuild of the
//!    projected-blocks `Snapshot` consumed by the API.
//!
//! [`Mempool`] is the public entry point. `Mempool::start` drives the
//! cycle on a 1-second tick.
//!
//! Source layout:
//!
//! - `steps/` - one file or folder per pipeline step.
//! - `stores/` - the state buckets held inside `MempoolState` plus
//!   the value types they contain.

mod steps;
mod stores;

pub use steps::preparer::Removal;
pub use steps::rebuilder::projected_blocks::{BlockStats, RecommendedFees, Snapshot};
pub use stores::{Entry, EntryPool, Tombstone, TxGraveyard, TxStore};

use std::{sync::Arc, thread, time::Duration};

use brk_error::Result;
use brk_rpc::Client;
use brk_types::{AddrBytes, MempoolInfo, TxOut, Txid, Vout};
use parking_lot::RwLockReadGuard;
use tracing::error;

use crate::{
    steps::{fetcher::Fetcher, preparer::Preparer, rebuilder::Rebuilder, resolver::Resolver},
    stores::{AddrTracker, MempoolState},
};

/// Public entry point to the mempool monitor.
///
/// Cheaply cloneable: wraps an `Arc` over the private state so clones
/// share a single live mempool. See the crate-level docs for the
/// pipeline shape.
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
        self.0.rebuilder.fees()
    }

    pub fn block_stats(&self) -> Vec<BlockStats> {
        self.0.rebuilder.block_stats()
    }

    pub fn next_block_hash(&self) -> u64 {
        self.0.rebuilder.next_block_hash()
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

    /// Start an infinite update loop with a 1 second interval.
    pub fn start(&self) {
        self.start_with(|| {});
    }

    /// Variant of `start` that runs `after_update` after every cycle.
    /// Used by `brk_cli` to drive `Query::fill_mempool_prevouts` so
    /// indexer-resolvable prevouts get filled in place each tick.
    pub fn start_with(&self, mut after_update: impl FnMut()) {
        loop {
            if let Err(e) = self.update() {
                error!("Error updating mempool: {}", e);
            }
            after_update();
            thread::sleep(Duration::from_secs(1));
        }
    }

    /// Fill any remaining `prevout == None` inputs on live mempool
    /// txs using `resolver`. Only call this if you have an external
    /// data source for confirmed parents (typically the brk indexer);
    /// in-mempool same-cycle parents are filled automatically by
    /// `MempoolState::apply` and don't need an external resolver.
    pub fn fill_prevouts<F>(&self, resolver: F) -> bool
    where
        F: Fn(&Txid, Vout) -> Option<TxOut>,
    {
        Resolver::resolve_external(&self.0.state, resolver)
    }

    /// One sync cycle: fetch -> prepare -> apply -> resolve -> (maybe) rebuild.
    /// The resolve step only runs when `apply` reported a change (no
    /// new txs means no new unresolved prevouts to fill); the rebuild
    /// step is throttled by `Rebuilder` regardless.
    pub fn update(&self) -> Result<()> {
        let inner = &*self.0;

        let fetched = Fetcher::fetch(
            &inner.client,
            &inner.state.txs.read(),
            &inner.state.graveyard.read(),
        )?;

        let pulled = Preparer::prepare(
            fetched,
            &inner.state.txs.read(),
            &inner.state.graveyard.read(),
        );

        if inner.state.apply(pulled) {
            Resolver::resolve_in_mempool(&inner.state);
            inner.rebuilder.mark_dirty();
        }

        inner.rebuilder.tick(&inner.client, &inner.state.entries);

        Ok(())
    }
}
