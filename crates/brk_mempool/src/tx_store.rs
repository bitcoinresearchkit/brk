use std::ops::Deref;

use brk_types::{TxWithHex, Txid};
use rustc_hash::FxHashMap;

/// Store of full transaction data for API access.
#[derive(Default)]
pub struct TxStore(FxHashMap<Txid, TxWithHex>);

impl Deref for TxStore {
    type Target = FxHashMap<Txid, TxWithHex>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TxStore {
    /// Check if a transaction exists.
    pub fn contains(&self, txid: &Txid) -> bool {
        self.0.contains_key(txid)
    }

    /// Add transactions in bulk.
    pub fn extend(&mut self, txs: FxHashMap<Txid, TxWithHex>) {
        self.0.extend(txs);
    }

    /// Keep items matching predicate, call `on_remove` for each removed item.
    pub fn retain_or_remove<K, R>(&mut self, mut keep: K, mut on_remove: R)
    where
        K: FnMut(&Txid) -> bool,
        R: FnMut(&Txid, &TxWithHex),
    {
        self.0.retain(|txid, tx| {
            if keep(txid) {
                true
            } else {
                on_remove(txid, tx);
                false
            }
        });
    }
}
