use std::{
    collections::hash_map::Entry as MapEntry,
    hash::{DefaultHasher, Hash, Hasher},
};

use brk_types::{AddrBytes, AddrMempoolStats, Transaction, TxOut, Txid};
use derive_more::Deref;
use rustc_hash::{FxHashMap, FxHashSet};

/// Per-address stats with associated transaction set.
pub type AddrStats = (AddrMempoolStats, FxHashSet<Txid>);

/// Tracks per-address mempool statistics.
#[derive(Default, Deref)]
pub struct AddrTracker(FxHashMap<AddrBytes, AddrStats>);

impl AddrTracker {
    /// Add a transaction to address tracking.
    pub fn add_tx(&mut self, tx: &Transaction, txid: &Txid) {
        self.update(tx, txid, true);
    }

    /// Remove a transaction from address tracking.
    pub fn remove_tx(&mut self, tx: &Transaction, txid: &Txid) {
        self.update(tx, txid, false);
    }

    /// Hash of an address's per-mempool stats. Stable while the address
    /// is unchanged; cheaper to recompute than to track invalidation.
    /// Returns 0 for unknown addresses (collision with a real hash is
    /// astronomically unlikely and only costs one ETag false-hit if it
    /// ever happens).
    pub fn stats_hash(&self, addr: &AddrBytes) -> u64 {
        let Some((stats, _)) = self.0.get(addr) else {
            return 0;
        };
        let mut hasher = DefaultHasher::new();
        stats.hash(&mut hasher);
        hasher.finish()
    }

    /// Fold a single newly-resolved input into the per-address stats.
    /// Called by the Resolver after a prevout that was previously
    /// `None` has been filled. Inputs whose prevout doesn't resolve
    /// to an addr are no-ops.
    pub fn add_input(&mut self, txid: &Txid, prevout: &TxOut) {
        let Some(bytes) = prevout.addr_bytes() else {
            return;
        };
        let (stats, txids) = self.0.entry(bytes).or_default();
        txids.insert(txid.clone());
        stats.sending(prevout);
        stats.update_tx_count(txids.len() as u32);
    }

    fn update(&mut self, tx: &Transaction, txid: &Txid, is_addition: bool) {
        for txin in &tx.input {
            let Some(prevout) = txin.prevout.as_ref() else {
                continue;
            };
            let Some(bytes) = prevout.addr_bytes() else {
                continue;
            };
            self.apply(bytes, txid, is_addition, |stats| {
                if is_addition {
                    stats.sending(prevout);
                } else {
                    stats.sent(prevout);
                }
            });
        }

        for txout in &tx.output {
            let Some(bytes) = txout.addr_bytes() else {
                continue;
            };
            self.apply(bytes, txid, is_addition, |stats| {
                if is_addition {
                    stats.receiving(txout);
                } else {
                    stats.received(txout);
                }
            });
        }
    }

    fn apply(
        &mut self,
        bytes: AddrBytes,
        txid: &Txid,
        is_addition: bool,
        update_stats: impl FnOnce(&mut AddrMempoolStats),
    ) {
        let mut entry = match self.0.entry(bytes) {
            MapEntry::Occupied(e) => e,
            MapEntry::Vacant(v) => {
                if !is_addition {
                    return;
                }
                v.insert_entry(Default::default())
            }
        };
        let (stats, txids) = entry.get_mut();
        if is_addition {
            txids.insert(txid.clone());
        } else {
            txids.remove(txid);
        }
        update_stats(stats);
        let len = txids.len();
        if len == 0 {
            entry.remove();
        } else {
            stats.update_tx_count(len as u32);
        }
    }
}
