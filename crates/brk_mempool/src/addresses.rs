use std::ops::Deref;

use brk_types::{AddressBytes, AddressMempoolStats, Transaction, Txid};
use rustc_hash::{FxHashMap, FxHashSet};

/// Per-address stats with associated transaction set.
pub type AddressStats = (AddressMempoolStats, FxHashSet<Txid>);

/// Tracks per-address mempool statistics.
#[derive(Default)]
pub struct AddressTracker(FxHashMap<AddressBytes, AddressStats>);

impl Deref for AddressTracker {
    type Target = FxHashMap<AddressBytes, AddressStats>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AddressTracker {
    /// Add a transaction to address tracking.
    pub fn add_tx(&mut self, tx: &Transaction, txid: &Txid) {
        self.update(tx, txid, true);
    }

    /// Remove a transaction from address tracking.
    pub fn remove_tx(&mut self, tx: &Transaction, txid: &Txid) {
        self.update(tx, txid, false);
    }

    fn update(&mut self, tx: &Transaction, txid: &Txid, is_addition: bool) {
        // Inputs: track sending
        for txin in &tx.input {
            let Some(prevout) = txin.prevout.as_ref() else {
                continue;
            };
            let Some(bytes) = prevout.address_bytes() else {
                continue;
            };

            let (stats, txids) = self.0.entry(bytes).or_default();
            if is_addition {
                txids.insert(txid.clone());
                stats.sending(prevout);
            } else {
                txids.remove(txid);
                stats.sent(prevout);
            }
            stats.update_tx_count(txids.len() as u32);
        }

        // Outputs: track receiving
        for txout in &tx.output {
            let Some(bytes) = txout.address_bytes() else {
                continue;
            };

            let (stats, txids) = self.0.entry(bytes).or_default();
            if is_addition {
                txids.insert(txid.clone());
                stats.receiving(txout);
            } else {
                txids.remove(txid);
                stats.received(txout);
            }
            stats.update_tx_count(txids.len() as u32);
        }
    }
}
