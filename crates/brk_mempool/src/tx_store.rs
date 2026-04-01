use brk_types::{MempoolRecentTx, TxWithHex, Txid};
use derive_more::Deref;
use rustc_hash::FxHashMap;

const RECENT_CAP: usize = 10;

/// Store of full transaction data for API access.
#[derive(Default, Deref)]
pub struct TxStore {
    #[deref]
    txs: FxHashMap<Txid, TxWithHex>,
    recent: Vec<MempoolRecentTx>,
}

impl TxStore {
    /// Check if a transaction exists.
    pub fn contains(&self, txid: &Txid) -> bool {
        self.txs.contains_key(txid)
    }

    /// Add transactions in bulk.
    pub fn extend(&mut self, txs: FxHashMap<Txid, TxWithHex>) {
        let mut new: Vec<_> = txs
            .iter()
            .take(RECENT_CAP)
            .map(|(txid, tx_hex)| MempoolRecentTx::from((txid, tx_hex.tx())))
            .collect();
        let keep = RECENT_CAP.saturating_sub(new.len());
        new.extend(self.recent.drain(..keep.min(self.recent.len())));
        self.recent = new;
        self.txs.extend(txs);
    }

    /// Last 10 transactions to enter the mempool.
    pub fn recent(&self) -> &[MempoolRecentTx] {
        &self.recent
    }

    /// Keep items matching predicate, call `on_remove` for each removed item.
    pub fn retain_or_remove<K, R>(&mut self, mut keep: K, mut on_remove: R)
    where
        K: FnMut(&Txid) -> bool,
        R: FnMut(&Txid, &TxWithHex),
    {
        self.txs.retain(|txid, tx| {
            if keep(txid) {
                true
            } else {
                on_remove(txid, tx);
                false
            }
        });
    }
}
