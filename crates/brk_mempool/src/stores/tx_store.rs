use brk_types::{MempoolRecentTx, Transaction, TxOut, Txid, TxidPrefix, Vin};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::TxEntry;

const RECENT_CAP: usize = 10;

/// Per-tx record: live tx body and its mempool entry, kept under one
/// key so a single map probe returns both.
pub struct TxRecord {
    pub tx: Transaction,
    pub entry: TxEntry,
}

/// Live-pool index keyed by `TxidPrefix`. The full `Txid` lives in
/// `record.entry.txid`, so callers that only have a `Txid` derive the
/// prefix (an 8-byte truncation) at the callsite. `unresolved` is the
/// set of prefixes whose tx still has at least one `prevout: None`,
/// maintained on every `insert` / `remove_by_prefix` / `apply_fills`
/// so the post-update prevout filler can early-exit when empty.
#[derive(Default)]
pub struct TxStore {
    records: FxHashMap<TxidPrefix, TxRecord>,
    recent: Vec<MempoolRecentTx>,
    unresolved: FxHashSet<TxidPrefix>,
}

impl TxStore {
    pub fn contains(&self, txid: &Txid) -> bool {
        self.records.contains_key(&TxidPrefix::from(txid))
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn get(&self, txid: &Txid) -> Option<&Transaction> {
        self.records.get(&TxidPrefix::from(txid)).map(|r| &r.tx)
    }

    pub fn entry(&self, txid: &Txid) -> Option<&TxEntry> {
        self.records.get(&TxidPrefix::from(txid)).map(|r| &r.entry)
    }

    pub fn entry_by_prefix(&self, prefix: &TxidPrefix) -> Option<&TxEntry> {
        self.records.get(prefix).map(|r| &r.entry)
    }

    /// Tx + entry in one map probe. Used by the RBF builder and the
    /// snapshot builder which need both per visited tx.
    pub fn record_by_prefix(&self, prefix: &TxidPrefix) -> Option<&TxRecord> {
        self.records.get(prefix)
    }

    /// `(prefix, record)` pairs in HashMap iteration order. Used by
    /// the snapshot builder to assign a compact `TxIndex` to each
    /// live tx in one pass.
    pub fn records(&self) -> impl Iterator<Item = (&TxidPrefix, &TxRecord)> {
        self.records.iter()
    }

    pub fn txids(&self) -> impl Iterator<Item = &Txid> {
        self.records.values().map(|r| &r.entry.txid)
    }

    pub fn values(&self) -> impl Iterator<Item = &Transaction> {
        self.records.values().map(|r| &r.tx)
    }

    pub fn insert(&mut self, tx: Transaction, entry: TxEntry) {
        let prefix = entry.txid_prefix();
        debug_assert!(
            !self.records.contains_key(&prefix),
            "TxidPrefix collision: {prefix:?} already mapped. Birthday-rare on SHA-256d."
        );
        self.sample_recent(&entry.txid, &tx);
        if tx.input.iter().any(|i| i.prevout.is_none()) {
            self.unresolved.insert(prefix);
        }
        self.records.insert(prefix, TxRecord { tx, entry });
    }

    fn sample_recent(&mut self, txid: &Txid, tx: &Transaction) {
        self.recent.insert(0, MempoolRecentTx::from((txid, tx)));
        self.recent.truncate(RECENT_CAP);
    }

    pub fn recent(&self) -> &[MempoolRecentTx] {
        &self.recent
    }

    /// Remove by prefix and return the full record if present. `recent`
    /// is untouched: it's an "added" window, not a live-set mirror.
    pub fn remove_by_prefix(&mut self, prefix: &TxidPrefix) -> Option<TxRecord> {
        let record = self.records.remove(prefix)?;
        self.unresolved.remove(prefix);
        Some(record)
    }

    /// Set of prefixes with at least one unfilled prevout. Used by the
    /// prevout filler as a cheap "is there any work?" gate.
    pub fn unresolved(&self) -> &FxHashSet<TxidPrefix> {
        &self.unresolved
    }

    /// Apply resolved prevouts to a tx in place. `fills` is `(vin, prevout)`.
    /// Returns the prevouts actually written (so the caller can fold them
    /// into `AddrTracker`). Updates `unresolved` if fully resolved after
    /// the fill, and recomputes `total_sigop_cost` (P2SH and witness
    /// components depend on prevouts).
    pub fn apply_fills(&mut self, prefix: &TxidPrefix, fills: Vec<(Vin, TxOut)>) -> Vec<TxOut> {
        let Some(record) = self.records.get_mut(prefix) else {
            return Vec::new();
        };
        let applied = Self::write_prevouts(&mut record.tx, fills);
        if applied.is_empty() {
            return applied;
        }
        record.tx.total_sigop_cost = record.tx.total_sigop_cost();
        if record.tx.input.iter().all(|i| i.prevout.is_some()) {
            self.unresolved.remove(prefix);
        }
        applied
    }

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
}
