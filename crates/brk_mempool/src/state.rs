//! Single-locked container for the live mempool.
//!
//! All cycle steps and read-side accessors take a guard on this one
//! lock. The substructures are plain owned types — they used to each
//! own a RwLock, but the canonical lock-order discipline disappears
//! when there's nothing to order.

use brk_types::{MempoolInfo, Timestamp, Txid};

use crate::{
    TxRemoval,
    stores::{AddrTracker, OutpointSpends, TxGraveyard, TxStore},
};

#[derive(Default)]
pub struct State {
    pub info: MempoolInfo,
    pub txs: TxStore,
    pub addrs: AddrTracker,
    pub outpoint_spends: OutpointSpends,
    pub graveyard: TxGraveyard,
}

impl State {
    /// `first_seen` for a tx that's live or in a `Vanished` tombstone.
    /// Smooths the flicker between drop and indexer catch-up; `Replaced`
    /// tombstones are excluded since the tx will not confirm.
    pub fn first_seen(&self, txid: &Txid) -> Option<Timestamp> {
        if let Some(e) = self.txs.entry(txid) {
            return Some(e.first_seen);
        }
        let tomb = self.graveyard.get(txid)?;
        matches!(tomb.reason(), TxRemoval::Vanished).then_some(tomb.entry.first_seen)
    }
}
