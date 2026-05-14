//! Address-keyed reads.

use std::cmp::Reverse;

use brk_types::{AddrBytes, AddrMempoolStats, Timestamp, Transaction, TxidPrefix};

use crate::Mempool;

impl Mempool {
    /// Hash of the address's mempool state, `None` if the address has
    /// no live mempool activity. Used as an `ETag` for address-keyed
    /// mempool responses. Route handlers may fall back to a 0 sentinel.
    pub fn addr_state_hash(&self, addr: &AddrBytes) -> Option<u64> {
        self.read().addrs.stats_hash(addr)
    }

    /// Per-address mempool stats. `None` if the address has no live mempool activity.
    pub fn addr_stats(&self, addr: &AddrBytes) -> Option<AddrMempoolStats> {
        self.read().addrs.get(addr).map(|e| e.stats.clone())
    }

    /// Live mempool txs touching `addr`, newest first by `first_seen`,
    /// capped at `limit`. Returns owned `Transaction`s.
    #[must_use]
    pub fn addr_txs(&self, addr: &AddrBytes, limit: usize) -> Vec<Transaction> {
        let state = self.read();
        let Some(entry) = state.addrs.get(addr) else {
            return vec![];
        };
        let mut ordered: Vec<(Timestamp, &Transaction)> = entry
            .txids
            .iter()
            .filter_map(|txid| {
                let record = state.txs.record_by_prefix(&TxidPrefix::from(txid))?;
                Some((record.entry.first_seen, &record.tx))
            })
            .collect();
        ordered.sort_unstable_by_key(|b| Reverse(b.0));
        ordered
            .into_iter()
            .take(limit)
            .map(|(_, tx)| tx.clone())
            .collect()
    }
}
