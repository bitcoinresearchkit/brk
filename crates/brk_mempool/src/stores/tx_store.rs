use brk_types::{MempoolRecentTx, Transaction, TxOut, Txid, Vin};
use derive_more::Deref;
use rustc_hash::{FxHashMap, FxHashSet};

const RECENT_CAP: usize = 10;

/// Store of full transaction data for API access.
#[derive(Default, Deref)]
pub struct TxStore {
    #[deref]
    txs: FxHashMap<Txid, Transaction>,
    recent: Vec<MempoolRecentTx>,
    /// Txids whose tx has at least one input with `prevout == None`.
    /// Maintained on every `extend` / `remove` / `apply_fills` so the
    /// post-update prevout filler can early-exit when this set is empty.
    unresolved: FxHashSet<Txid>,
}

impl TxStore {
    pub fn contains(&self, txid: &Txid) -> bool {
        self.txs.contains_key(txid)
    }

    /// Insert each `(Txid, Transaction)` yielded by `items`, and push
    /// up to `RECENT_CAP` of them onto the front of `recent` as the
    /// newest-seen window (older entries fall off the end).
    pub fn extend<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = (Txid, Transaction)>,
    {
        let mut new_recent: Vec<MempoolRecentTx> = Vec::with_capacity(RECENT_CAP);
        for (txid, tx) in items {
            if new_recent.len() < RECENT_CAP {
                new_recent.push(MempoolRecentTx::from((&txid, &tx)));
            }
            if tx.input.iter().any(|i| i.prevout.is_none()) {
                self.unresolved.insert(txid.clone());
            }
            self.txs.insert(txid, tx);
        }

        let keep = RECENT_CAP.saturating_sub(new_recent.len());
        new_recent.extend(self.recent.drain(..keep.min(self.recent.len())));
        self.recent = new_recent;
    }

    pub fn recent(&self) -> &[MempoolRecentTx] {
        &self.recent
    }

    /// Remove a single tx and return its stored data if present. `recent`
    /// isn't touched: it's an "added" window, not a live-set mirror.
    pub fn remove(&mut self, txid: &Txid) -> Option<Transaction> {
        self.unresolved.remove(txid);
        self.txs.remove(txid)
    }

    /// Set of txids with at least one unfilled prevout. Used by the
    /// prevout filler as a cheap "is there any work?" gate.
    pub fn unresolved(&self) -> &FxHashSet<Txid> {
        &self.unresolved
    }

    /// Apply resolved prevouts to a tx in place. `fills` is `(vin, prevout)`.
    /// Returns the prevouts that were actually written (so the caller can
    /// fold them into `AddrTracker`). Updates `unresolved` if the tx is
    /// fully resolved after the fill, and recomputes `total_sigop_cost`
    /// since the P2SH and witness components depend on prevouts.
    pub fn apply_fills(&mut self, txid: &Txid, fills: Vec<(Vin, TxOut)>) -> Vec<TxOut> {
        let Some(tx) = self.txs.get_mut(txid) else {
            return Vec::new();
        };
        let mut applied = Vec::with_capacity(fills.len());
        for (vin, prevout) in fills {
            if let Some(txin) = tx.input.get_mut(usize::from(vin))
                && txin.prevout.is_none()
            {
                txin.prevout = Some(prevout.clone());
                applied.push(prevout);
            }
        }
        if !applied.is_empty() {
            tx.total_sigop_cost = tx.total_sigop_cost();
        }
        if !tx.input.iter().any(|i| i.prevout.is_none()) {
            self.unresolved.remove(txid);
        }
        applied
    }
}
