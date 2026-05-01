use brk_types::{OutpointPrefix, Transaction, TxidPrefix};
use derive_more::Deref;
use rustc_hash::FxHashMap;

use super::TxIndex;

/// Mempool index from spent outpoint to spending mempool tx.
///
/// Keys are `OutpointPrefix` (8 bytes txid + 2 bytes vout); prefix
/// collisions are possible, so callers must verify the candidate
/// spender's input list. Values are slot indices into `EntryPool`,
/// stable for the lifetime of an entry.
#[derive(Default, Deref)]
pub struct OutpointSpends(FxHashMap<OutpointPrefix, TxIndex>);

impl OutpointSpends {
    pub fn insert_spends(&mut self, tx: &Transaction, idx: TxIndex) {
        for input in &tx.input {
            if input.is_coinbase {
                continue;
            }
            let key = OutpointPrefix::new(TxidPrefix::from(&input.txid), input.vout);
            self.0.insert(key, idx);
        }
    }

    /// Only removes entries whose stored `TxIndex` still matches `idx`,
    /// so a slot already recycled by a later insert is left alone.
    pub fn remove_spends(&mut self, tx: &Transaction, idx: TxIndex) {
        for input in &tx.input {
            if input.is_coinbase {
                continue;
            }
            let key = OutpointPrefix::new(TxidPrefix::from(&input.txid), input.vout);
            if self.0.get(&key) == Some(&idx) {
                self.0.remove(&key);
            }
        }
    }

    #[inline]
    pub fn get(&self, key: &OutpointPrefix) -> Option<TxIndex> {
        self.0.get(key).copied()
    }
}
