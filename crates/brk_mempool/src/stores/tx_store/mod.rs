use std::hash::{Hash, Hasher};
use std::sync::Arc;

use brk_oracle::HistogramRaw;
use brk_types::{MempoolRecentTx, Transaction, TxOut, Txid, TxidPrefix, Vin};
use indexmap::IndexMap;
use rustc_hash::{FxBuildHasher, FxHashSet, FxHasher};

use crate::{state::TxEntry, stores::LiveHistograms};

pub mod tx_record;

pub use tx_record::TxRecord;

const RECENT_CAP: usize = 10;

/// Live-pool index keyed by `TxidPrefix`. The full `Txid` lives in
/// `record.entry.txid`, so callers that only have a `Txid` derive the
/// prefix (an 8-byte truncation) at the callsite. `unresolved` is the
/// set of prefixes whose tx still has at least one `prevout: None`,
/// maintained on every `insert` / `remove_by_prefix` / `apply_fills`
/// so the post-update prevout filler can early-exit when empty.
/// `histograms` holds the eligible (oracle-blend) and raw per-bin output
/// histograms, kept in sync on `insert` / `remove_by_prefix` so each read
/// path is a single array clone, not a full pool walk.
#[derive(Default)]
pub struct TxStore {
    records: IndexMap<TxidPrefix, TxRecord, FxBuildHasher>,
    txids_hash: u64,
    content_revision: u64,
    recent: Vec<MempoolRecentTx>,
    unresolved: FxHashSet<TxidPrefix>,
    histograms: LiveHistograms,
}

impl TxStore {
    pub fn contains(&self, txid: &Txid) -> bool {
        self.get(txid).is_some()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.records.reserve(additional);
    }

    pub fn get(&self, txid: &Txid) -> Option<&Transaction> {
        self.record(txid).map(|record| record.tx.as_ref())
    }

    pub fn entry(&self, txid: &Txid) -> Option<&TxEntry> {
        self.record(txid).map(|record| &record.entry)
    }

    pub fn record(&self, txid: &Txid) -> Option<&TxRecord> {
        let record = self.records.get(&TxidPrefix::from(txid))?;
        (record.entry.txid == *txid).then_some(record)
    }

    /// Tx + entry in one map probe. Used by the RBF builder and the
    /// snapshot builder which need both per visited tx.
    pub fn record_by_prefix(&self, prefix: &TxidPrefix) -> Option<&TxRecord> {
        self.records.get(prefix)
    }

    /// `(prefix, record)` pairs in dense storage order. Used by the
    /// snapshot builder to assign a compact `TxIndex` to each live tx.
    pub fn records(&self) -> impl Iterator<Item = (&TxidPrefix, &TxRecord)> {
        self.records.iter()
    }

    pub fn txids(&self) -> impl Iterator<Item = &Txid> {
        self.records.values().map(|r| &r.entry.txid)
    }

    pub fn txids_hash(&self) -> u64 {
        self.txids_hash
    }

    /// Process-local revision for every mutation that can change serialized tx bodies.
    pub fn content_revision(&self) -> u64 {
        self.content_revision
    }

    fn txid_position_hash(txid: &Txid, position: usize) -> u64 {
        let mut hasher = FxHasher::default();
        (position as u64).hash(&mut hasher);
        txid.hash(&mut hasher);
        hasher.finish()
    }

    pub fn insert(&mut self, tx: impl Into<Arc<Transaction>>, entry: TxEntry) {
        let tx = tx.into();
        let prefix = entry.txid_prefix();
        debug_assert!(
            !self.records.contains_key(&prefix),
            "TxidPrefix collision: {prefix:?} already mapped. Birthday-rare on SHA-256d."
        );
        let position = self.records.len();
        let txid = entry.txid;
        self.sample_recent(&entry.txid, &tx);
        if tx.input.iter().any(|i| i.prevout.is_none()) {
            self.unresolved.insert(prefix);
        }
        let record = TxRecord { tx, entry };
        self.histograms.add(&record);
        self.records.insert(prefix, record);
        self.txids_hash ^= Self::txid_position_hash(&txid, position);
        self.bump_content_revision();
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
        let last_position = self.records.len().checked_sub(1)?;
        let last_txid = self.records.last()?.1.entry.txid;
        let (position, _, record) = self.records.swap_remove_full(prefix)?;
        self.txids_hash ^= Self::txid_position_hash(&record.entry.txid, position);
        if position != last_position {
            self.txids_hash ^= Self::txid_position_hash(&last_txid, last_position);
            self.txids_hash ^= Self::txid_position_hash(&last_txid, position);
        }
        self.unresolved.remove(prefix);
        self.histograms.remove(&record);
        self.bump_content_revision();
        Some(record)
    }

    /// Snapshot the round-dollar-eligible histogram that feeds the oracle
    /// blend. Maintained incrementally, so this is `O(NUM_BINS)`, not
    /// `O(live_outputs)`.
    pub fn live_eligible_histogram(&self) -> HistogramRaw {
        self.histograms.eligible()
    }

    /// Snapshot the raw histogram: every live output binned by value with no
    /// payment filtering. Maintained incrementally alongside the eligible one.
    pub fn live_raw_histogram(&self) -> HistogramRaw {
        self.histograms.raw()
    }

    /// Set of prefixes with at least one unfilled prevout. Used by the
    /// prevout filler as a cheap "is there any work?" gate.
    pub fn unresolved(&self) -> &FxHashSet<TxidPrefix> {
        &self.unresolved
    }

    /// Apply resolved prevouts to a tx in place. `fills` is `(vin, prevout)`.
    /// Returns the prevouts actually written (so the caller can fold them
    /// into `AddrTracker`). Updates `unresolved` if fully resolved after
    /// the fill, and refreshes `total_sigop_cost` (P2SH and witness
    /// components depend on prevouts). `entry.vsize` is Core's value from
    /// `MempoolEntryInfo` and is not recomputed here - the sigops shift
    /// belongs to the `Transaction`, not the entry.
    pub fn apply_fills(&mut self, prefix: &TxidPrefix, fills: Vec<(Vin, TxOut)>) -> Vec<TxOut> {
        let Some(record) = self.records.get_mut(prefix) else {
            return Vec::new();
        };
        if !fills.iter().any(|(vin, _)| {
            record
                .tx
                .input
                .get(usize::from(*vin))
                .is_some_and(|input| input.prevout.is_none())
        }) {
            return Vec::new();
        }
        let tx = Arc::make_mut(&mut record.tx);
        let applied = Self::write_prevouts(tx, fills);
        if applied.is_empty() {
            return applied;
        }
        tx.refresh_sigops();
        if record.tx.input.iter().all(|i| i.prevout.is_some()) {
            self.unresolved.remove(prefix);
        }
        self.bump_content_revision();
        applied
    }

    fn bump_content_revision(&mut self) {
        self.content_revision = self.content_revision.wrapping_add(1);
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

#[cfg(test)]
#[path = "../../../tests/unit/stores/tx_store.rs"]
mod tests;
