//! Single-locked container for the live mempool. All cycle steps and
//! read-side accessors take a guard on this one lock.
//!
//! # Concurrency
//!
//! `State` is held under one `RwLock` at the crate root. The cycle
//! takes the write guard for `Applier` and `Prevouts`, then drops it
//! before the [`crate::snapshot::Rebuilder`] runs. No code path holds
//! a `State` guard at the same time as a `Rebuilder` lock, so the two
//! domains are independent and lock-ordering between them is moot.

mod tx_entry;

pub use tx_entry::TxEntry;

use brk_types::{MempoolInfo, Timestamp, Txid};

use crate::stores::{AddrTracker, OutpointSpends, TxGraveyard, TxStore};

#[derive(Default)]
pub struct State {
    pub info: MempoolInfo,
    pub txs: TxStore,
    pub addrs: AddrTracker,
    pub outpoint_spends: OutpointSpends,
    pub graveyard: TxGraveyard,
}

impl State {
    /// Smooths the flicker between drop and indexer catch-up. `Replaced`
    /// tombstones are excluded since the tx will not confirm.
    pub fn first_seen(&self, txid: &Txid) -> Option<Timestamp> {
        if let Some(e) = self.txs.entry(txid) {
            return Some(e.first_seen);
        }
        self.graveyard.get_vanished(txid).map(|t| t.entry.first_seen)
    }
}
