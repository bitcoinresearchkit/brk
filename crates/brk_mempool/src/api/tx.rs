//! Tx-keyed reads.

use brk_types::{
    MempoolRecentTx, OutpointPrefix, Transaction, Txid, TxidPrefix, Vin, Vout,
};

use crate::Mempool;

impl Mempool {
    pub fn contains_txid(&self, txid: &Txid) -> bool {
        self.read().txs.contains(txid)
    }

    /// Apply `f` to the live tx body if present.
    pub fn with_tx<R>(&self, txid: &Txid, f: impl FnOnce(&Transaction) -> R) -> Option<R> {
        self.read().txs.get(txid).map(f)
    }

    /// Apply `f` to a `Vanished` tombstone's tx body if present.
    /// `Replaced` tombstones return `None` because the tx will not confirm.
    pub fn with_vanished_tx<R>(&self, txid: &Txid, f: impl FnOnce(&Transaction) -> R) -> Option<R> {
        self.read().graveyard.get_vanished(txid).map(|t| f(&t.tx))
    }

    /// Mempool tx spending `(txid, vout)`, or `None`. The spender's
    /// input list is walked to rule out `TxidPrefix` collisions.
    pub fn lookup_spender(&self, txid: &Txid, vout: Vout) -> Option<(Txid, Vin)> {
        let key = OutpointPrefix::new(TxidPrefix::from(txid), vout);
        let state = self.read();
        let spender_prefix = state.outpoint_spends.get(&key)?;
        let spender = state.txs.record_by_prefix(&spender_prefix)?;
        let vin_pos = spender
            .tx
            .input
            .iter()
            .position(|inp| inp.txid == *txid && inp.vout == vout)?;
        Some((spender.entry.txid, Vin::from(vin_pos)))
    }

    /// Snapshot of all live mempool txids.
    ///
    /// Allocates `32 * len(mempool)` bytes under the read guard. Sized for
    /// diagnostics. Route layers serving large pools should paginate at
    /// their boundary rather than calling this per request.
    #[must_use]
    pub fn txids(&self) -> Vec<Txid> {
        self.read().txs.txids().copied().collect()
    }

    /// Snapshot of recent live txs.
    #[must_use]
    pub fn recent_txs(&self) -> Vec<MempoolRecentTx> {
        self.read().txs.recent().to_vec()
    }

    /// `first_seen` Unix-second timestamps for `txids`, in input order.
    /// Returns 0 for unknown txids. `Vanished` tombstones fall back to
    /// the buried entry's `first_seen` to avoid flicker between drop
    /// and indexer catch-up.
    #[must_use]
    pub fn transaction_times(&self, txids: &[Txid]) -> Vec<u64> {
        let state = self.read();
        txids
            .iter()
            .map(|txid| state.first_seen(txid).map_or(0, u64::from))
            .collect()
    }
}
