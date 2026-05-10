use std::{
    collections::hash_map::Entry as MapEntry,
    hash::{Hash, Hasher},
};

use brk_types::{AddrBytes, AddrMempoolStats, Transaction, TxOut, Txid};
use derive_more::Deref;
use rustc_hash::{FxHashMap, FxHasher};

mod addr_entry;

use addr_entry::AddrEntry;

#[derive(Default, Deref)]
pub struct AddrTracker(FxHashMap<AddrBytes, AddrEntry>);

impl AddrTracker {
    pub fn add_tx(&mut self, tx: &Transaction) {
        let txid = &tx.txid;
        for txin in &tx.input {
            if let Some(prevout) = txin.prevout.as_ref() {
                self.add_input(txid, prevout);
            }
        }
        for txout in &tx.output {
            if let Some(bytes) = txout.addr_bytes() {
                self.apply_add(bytes, txid, |stats| stats.receiving(txout));
            }
        }
    }

    pub fn remove_tx(&mut self, tx: &Transaction) {
        let txid = &tx.txid;
        for txin in &tx.input {
            if let Some(prevout) = txin.prevout.as_ref() {
                self.remove_input(txid, prevout);
            }
        }
        for txout in &tx.output {
            if let Some(bytes) = txout.addr_bytes() {
                self.apply_remove(bytes, txid, |stats| stats.received(txout));
            }
        }
    }

    /// Hash of an address's per-mempool stats. Stable while the address
    /// is unchanged. Cheaper to recompute than to track invalidation.
    /// Returns 0 for unknown addresses (collision with a real hash is
    /// astronomically unlikely and only costs one ETag false-hit if it
    /// ever happens).
    pub fn stats_hash(&self, addr: &AddrBytes) -> u64 {
        let Some(entry) = self.0.get(addr) else {
            return 0;
        };
        let mut hasher = FxHasher::default();
        entry.stats.hash(&mut hasher);
        hasher.finish()
    }

    /// Fold a single newly-resolved input into the per-address stats.
    /// Called by the prevout-fill paths after a prevout that was
    /// previously `None` has been filled, and by `add_tx` for each
    /// resolved input. Inputs whose prevout doesn't resolve to an addr
    /// are no-ops.
    pub fn add_input(&mut self, txid: &Txid, prevout: &TxOut) {
        let Some(bytes) = prevout.addr_bytes() else {
            return;
        };
        self.apply_add(bytes, txid, |stats| stats.sending(prevout));
    }

    fn remove_input(&mut self, txid: &Txid, prevout: &TxOut) {
        let Some(bytes) = prevout.addr_bytes() else {
            return;
        };
        self.apply_remove(bytes, txid, |stats| stats.sent(prevout));
    }

    fn apply_add(
        &mut self,
        bytes: AddrBytes,
        txid: &Txid,
        update_stats: impl FnOnce(&mut AddrMempoolStats),
    ) {
        let entry = self.0.entry(bytes).or_default();
        entry.txids.insert(*txid);
        update_stats(&mut entry.stats);
        entry.stats.update_tx_count(entry.txids.len() as u32);
    }

    fn apply_remove(
        &mut self,
        bytes: AddrBytes,
        txid: &Txid,
        update_stats: impl FnOnce(&mut AddrMempoolStats),
    ) {
        let MapEntry::Occupied(mut occupied) = self.0.entry(bytes) else {
            return;
        };
        let entry = occupied.get_mut();
        entry.txids.remove(txid);
        update_stats(&mut entry.stats);
        let len = entry.txids.len();
        if len == 0 {
            occupied.remove();
        } else {
            entry.stats.update_tx_count(len as u32);
        }
    }
}
