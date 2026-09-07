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

pub mod tx_entry;

use brk_error::{Error, Result};
use brk_types::{BlockHash, MempoolInfo, Timestamp, Txid};
pub use tx_entry::TxEntry;

use crate::{
    Snapshot,
    stores::{AddrTracker, OutpointSpends, TxGraveyard, TxStore},
};

#[derive(Default)]
pub struct State {
    /// Present only after a complete address view was built between matching
    /// best-chain observations. Cleared before live mutations begin.
    pub published_tip: Option<BlockHash>,
    pub info: MempoolInfo,
    pub txs: TxStore,
    pub addrs: AddrTracker,
    pub outpoint_spends: OutpointSpends,
    pub graveyard: TxGraveyard,
}

#[cfg(test)]
#[path = "../../tests/unit/state.rs"]
mod tests;

impl State {
    /// Aggregate pool reads need a completed observation, but do not join it
    /// with an indexed-chain snapshot and therefore accept any published tip.
    pub fn ensure_published(&self) -> Result<()> {
        self.published_tip.ok_or(Error::StateUpdating).map(|_| ())
    }

    pub fn ensure_published_at(&self, tip: &BlockHash) -> Result<()> {
        if self.published_tip.as_ref() != Some(tip) {
            return Err(Error::StateUpdating);
        }
        Ok(())
    }

    /// The graph projection and live fields must describe the same completed
    /// transaction revision, not merely contain the same requested txid.
    pub fn ensure_snapshot_at(&self, tip: &BlockHash, snapshot: &Snapshot) -> Result<()> {
        self.ensure_published_at(tip)?;
        if snapshot.content_revision() != self.txs.content_revision() {
            return Err(Error::StateUpdating);
        }
        Ok(())
    }

    /// Publish the address view when inputs are complete. Return whether the
    /// membership is complete, which is sufficient for aggregate statistics.
    pub fn publish_at(&mut self, tip: BlockHash, live_txids: &[Txid]) -> bool {
        let complete = self.txs.len() == live_txids.len()
            && live_txids.iter().all(|txid| self.txs.contains(txid));
        self.published_tip = (complete && self.txs.unresolved().is_empty()).then_some(tip);
        complete
    }

    /// Smooths the flicker between drop and indexer catch-up. `Replaced`
    /// tombstones are excluded since the tx will not confirm.
    pub fn first_seen(&self, txid: &Txid) -> Option<Timestamp> {
        if let Some(e) = self.txs.entry(txid) {
            return Some(e.first_seen);
        }
        self.graveyard
            .get_vanished(txid)
            .map(|t| t.entry.first_seen)
    }
}
