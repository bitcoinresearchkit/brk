use brk_types::{OutpointPrefix, Transaction, TxidPrefix};
use derive_more::Deref;
use rustc_hash::FxHashMap;

/// Mempool index from spent outpoint to spending mempool tx.
///
/// Keys are `OutpointPrefix` (8 bytes txid + 2 bytes vout); prefix
/// collisions are possible, so callers must verify the candidate
/// spender's input list. Values are the spender's `TxidPrefix`,
/// looked up against `TxStore` to recover the full spender record.
#[derive(Default, Deref)]
pub struct OutpointSpends(FxHashMap<OutpointPrefix, TxidPrefix>);

impl OutpointSpends {
    pub fn insert_spends(&mut self, tx: &Transaction, spender: TxidPrefix) {
        for input in &tx.input {
            if input.is_coinbase {
                continue;
            }
            let key = OutpointPrefix::new(TxidPrefix::from(&input.txid), input.vout);
            self.0.insert(key, spender);
        }
    }

    /// Only removes entries whose stored prefix still matches `spender`,
    /// so an outpoint already re-claimed by a later spender is left alone.
    pub fn remove_spends(&mut self, tx: &Transaction, spender: TxidPrefix) {
        for input in &tx.input {
            if input.is_coinbase {
                continue;
            }
            let key = OutpointPrefix::new(TxidPrefix::from(&input.txid), input.vout);
            if self.0.get(&key) == Some(&spender) {
                self.0.remove(&key);
            }
        }
    }

    #[inline]
    pub fn get(&self, key: &OutpointPrefix) -> Option<TxidPrefix> {
        self.0.get(key).copied()
    }
}
