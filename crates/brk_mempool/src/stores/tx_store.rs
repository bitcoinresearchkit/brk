use brk_types::{MempoolRecentTx, Transaction, TxOut, Txid, Vin};
use derive_more::Deref;
use rustc_hash::{FxHashMap, FxHashSet};

const RECENT_CAP: usize = 10;

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
            Self::sample_recent(&mut new_recent, &txid, &tx);
            self.track_unresolved(&txid, &tx);
            self.txs.insert(txid, tx);
        }
        self.promote_recent(new_recent);
    }

    /// Append to the cap-bounded sample buffer if there's room. The
    /// pre-cap window becomes the next `recent()` value.
    fn sample_recent(buf: &mut Vec<MempoolRecentTx>, txid: &Txid, tx: &Transaction) {
        if buf.len() < RECENT_CAP {
            buf.push(MempoolRecentTx::from((txid, tx)));
        }
    }

    /// Record `txid` in the unresolved set if any input lacks a
    /// prevout. Cleared later by `apply_fills` once all inputs fill.
    fn track_unresolved(&mut self, txid: &Txid, tx: &Transaction) {
        if tx.input.iter().any(|i| i.prevout.is_none()) {
            self.unresolved.insert(txid.clone());
        }
    }

    fn promote_recent(&mut self, mut new_recent: Vec<MempoolRecentTx>) {
        if new_recent.is_empty() {
            return;
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
        let applied = Self::write_prevouts(tx, fills);
        if applied.is_empty() {
            return applied;
        }
        Self::recompute_sigop(tx);
        self.refresh_unresolved(txid);
        applied
    }

    /// Apply each `(vin, prevout)` to its empty input slot. Skips vins
    /// that are out of range or already filled. Returns the prevouts
    /// that were actually written.
    fn write_prevouts(tx: &mut Transaction, fills: Vec<(Vin, TxOut)>) -> Vec<TxOut> {
        let mut applied = Vec::with_capacity(fills.len());
        for (vin, prevout) in fills {
            if let Some(txin) = tx.input.get_mut(usize::from(vin))
                && txin.prevout.is_none()
            {
                txin.prevout = Some(prevout.clone());
                applied.push(prevout);
            }
        }
        applied
    }

    /// `total_sigop_cost` depends on the P2SH and witness components
    /// of each prevout, so it must be recomputed after any fill.
    fn recompute_sigop(tx: &mut Transaction) {
        tx.total_sigop_cost = tx.total_sigop_cost();
    }

    /// Drop `txid` from the unresolved set if every input now has a
    /// prevout. Idempotent if the tx was removed between phases.
    fn refresh_unresolved(&mut self, txid: &Txid) {
        if self.txs.get(txid).is_some_and(Self::all_resolved) {
            self.unresolved.remove(txid);
        }
    }

    fn all_resolved(tx: &Transaction) -> bool {
        tx.input.iter().all(|i| i.prevout.is_some())
    }
}
