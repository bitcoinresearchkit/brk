use brk_oracle::HistogramRaw;
use brk_types::{MempoolRecentTx, Transaction, TxOut, Txid, TxidPrefix, Vin};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{state::TxEntry, stores::OutputBins};

const RECENT_CAP: usize = 10;

/// Per-tx record: live tx body, its mempool entry, and the pre-bucketed
/// oracle bins for its outputs. Kept under one key so a single map probe
/// returns everything readers need.
pub struct TxRecord {
    pub tx: Transaction,
    pub entry: TxEntry,
    pub output_bins: OutputBins,
}

impl TxRecord {
    pub fn new(tx: Transaction, entry: TxEntry) -> Self {
        let output_bins = OutputBins::from_tx(&tx);
        Self {
            tx,
            entry,
            output_bins,
        }
    }
}

/// Live-pool index keyed by `TxidPrefix`. The full `Txid` lives in
/// `record.entry.txid`, so callers that only have a `Txid` derive the
/// prefix (an 8-byte truncation) at the callsite. `unresolved` is the
/// set of prefixes whose tx still has at least one `prevout: None`,
/// maintained on every `insert` / `remove_by_prefix` / `apply_fills`
/// so the post-update prevout filler can early-exit when empty.
/// `live_histogram` mirrors the union of each record's `OutputBins`,
/// kept in sync on `insert` / `remove_by_prefix` so the oracle-blend
/// read path is a single array clone, not a full pool walk.
#[derive(Default)]
pub struct TxStore {
    records: FxHashMap<TxidPrefix, TxRecord>,
    recent: Vec<MempoolRecentTx>,
    unresolved: FxHashSet<TxidPrefix>,
    live_histogram: HistogramRaw,
}

impl TxStore {
    pub fn contains(&self, txid: &Txid) -> bool {
        self.records.contains_key(&TxidPrefix::from(txid))
    }

    pub fn len(&self) -> usize {
        self.records.len()
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

    /// `(prefix, record)` pairs in `HashMap` iteration order. Used by
    /// the snapshot builder to assign a compact `TxIndex` to each
    /// live tx in one pass.
    pub fn records(&self) -> impl Iterator<Item = (&TxidPrefix, &TxRecord)> {
        self.records.iter()
    }

    pub fn txids(&self) -> impl Iterator<Item = &Txid> {
        self.records.values().map(|r| &r.entry.txid)
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
        let record = TxRecord::new(tx, entry);
        for bin in record.output_bins.iter() {
            self.live_histogram[bin as usize] += 1;
        }
        self.records.insert(prefix, record);
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
        for bin in record.output_bins.iter() {
            self.live_histogram[bin as usize] -= 1;
        }
        Some(record)
    }

    /// Snapshot the live oracle-bin histogram. Maintained incrementally
    /// on insert/remove, so this is `O(NUM_BINS)`, not `O(live_outputs)`.
    pub fn live_histogram(&self) -> HistogramRaw {
        self.live_histogram.clone()
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
        let applied = Self::write_prevouts(&mut record.tx, fills);
        if applied.is_empty() {
            return applied;
        }
        record.tx.refresh_sigops();
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

#[cfg(test)]
mod tests {
    use bitcoin::ScriptBuf;
    use brk_types::{MempoolEntryInfo, Sats, Timestamp, VSize, Weight};

    use super::*;
    use crate::test_support::{fake_tx, fake_txid, p2wpkh_script};

    fn entry_for(tx: &Transaction, fee: u64, vsize: u64) -> TxEntry {
        let info = MempoolEntryInfo {
            txid: tx.txid,
            vsize: VSize::from(vsize),
            weight: Weight::from(VSize::from(vsize)),
            fee: Sats::from(fee),
            first_seen: Timestamp::from(0u32),
            depends: vec![],
        };
        TxEntry::new(&info, vsize, false)
    }

    fn tx_without_prevouts(seed: u8) -> Transaction {
        fake_tx(seed, &[None, None], &[(p2wpkh_script(1), 1_000)])
    }

    fn tx_with_prevouts(seed: u8) -> Transaction {
        let prev = Some(TxOut::from((p2wpkh_script(2), Sats::from(2_000u64))));
        fake_tx(seed, &[prev], &[(p2wpkh_script(3), 500)])
    }

    #[test]
    fn insert_records_unresolved_when_prevouts_missing() {
        let mut store = TxStore::default();
        let tx = tx_without_prevouts(1);
        let entry = entry_for(&tx, 100, 100);
        let prefix = entry.txid_prefix();
        store.insert(tx, entry);

        assert!(store.unresolved().contains(&prefix));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn insert_skips_unresolved_when_all_prevouts_present() {
        let mut store = TxStore::default();
        let tx = tx_with_prevouts(2);
        let entry = entry_for(&tx, 200, 150);
        let prefix = entry.txid_prefix();
        store.insert(tx, entry);

        assert!(!store.unresolved().contains(&prefix));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn remove_by_prefix_clears_unresolved_and_returns_record() {
        let mut store = TxStore::default();
        let tx = tx_without_prevouts(3);
        let entry = entry_for(&tx, 300, 200);
        let prefix = entry.txid_prefix();
        store.insert(tx, entry);
        assert!(store.unresolved().contains(&prefix));

        let removed = store.remove_by_prefix(&prefix).expect("record present");
        assert_eq!(removed.entry.txid_prefix(), prefix);
        assert!(!store.unresolved().contains(&prefix));
        assert_eq!(store.len(), 0);
        assert!(store.remove_by_prefix(&prefix).is_none());
    }

    #[test]
    fn apply_fills_writes_only_missing_inputs_and_refreshes_sigops() {
        let mut store = TxStore::default();
        let prev_present = TxOut::from((p2wpkh_script(4), Sats::from(7_000u64)));
        let tx = fake_tx(
            4,
            &[None, Some(prev_present.clone())],
            &[(p2wpkh_script(5), 1_000)],
        );
        let entry = entry_for(&tx, 400, 250);
        let prefix = entry.txid_prefix();
        store.insert(tx, entry);
        assert!(store.unresolved().contains(&prefix));

        let new_prevout = TxOut::from((p2wpkh_script(6), Sats::from(9_000u64)));
        let overwrite_attempt = TxOut::from((p2wpkh_script(99), Sats::from(1u64)));
        let applied = store.apply_fills(
            &prefix,
            vec![
                (Vin::from(0u32), new_prevout.clone()),
                (Vin::from(1u32), overwrite_attempt),
            ],
        );

        assert_eq!(applied.len(), 1);
        assert_eq!(applied[0].value, new_prevout.value);

        let record = store.record_by_prefix(&prefix).expect("record present");
        assert_eq!(
            record.tx.input[0].prevout.as_ref().unwrap().value,
            new_prevout.value
        );
        assert_eq!(
            record.tx.input[1].prevout.as_ref().unwrap().value,
            prev_present.value
        );
        assert!(!store.unresolved().contains(&prefix));
    }

    #[test]
    fn apply_fills_unknown_prefix_is_noop() {
        let mut store = TxStore::default();
        let stray_prefix = TxidPrefix::from(&fake_txid(0xFF));
        let applied = store.apply_fills(
            &stray_prefix,
            vec![(
                Vin::from(0u32),
                TxOut::from((ScriptBuf::new(), Sats::from(1u64))),
            )],
        );
        assert!(applied.is_empty());
    }

    #[test]
    fn apply_fills_partial_keeps_unresolved() {
        let mut store = TxStore::default();
        let tx = tx_without_prevouts(5);
        let entry = entry_for(&tx, 500, 300);
        let prefix = entry.txid_prefix();
        store.insert(tx, entry);

        let one = TxOut::from((p2wpkh_script(7), Sats::from(3_000u64)));
        let applied = store.apply_fills(&prefix, vec![(Vin::from(0u32), one)]);
        assert_eq!(applied.len(), 1);
        assert!(
            store.unresolved().contains(&prefix),
            "input 1 still has None prevout"
        );
    }

    #[test]
    fn recent_is_capped_and_newest_first() {
        let mut store = TxStore::default();
        for i in 0..(RECENT_CAP as u8 + 5) {
            let tx = tx_with_prevouts(i + 10);
            let entry = entry_for(&tx, 100, 100);
            store.insert(tx, entry);
        }
        assert_eq!(store.recent().len(), RECENT_CAP);
        let newest = store.recent().first().expect("at least one");
        let last_inserted_txid = fake_txid(RECENT_CAP as u8 + 5 + 10 - 1);
        assert_eq!(newest.txid, last_inserted_txid);
    }

    #[test]
    fn live_histogram_total_tracks_inserts_and_removes() {
        let mut store = TxStore::default();
        let tx_a = fake_tx(
            20,
            &[Some(TxOut::from((p2wpkh_script(8), Sats::from(1_234u64))))],
            &[(p2wpkh_script(9), 2_345), (p2wpkh_script(10), 3_456)],
        );
        let tx_b = fake_tx(
            21,
            &[Some(TxOut::from((p2wpkh_script(11), Sats::from(4_567u64))))],
            &[(p2wpkh_script(12), 7_891)],
        );
        let entry_a = entry_for(&tx_a, 100, 100);
        let entry_b = entry_for(&tx_b, 100, 100);
        let prefix_a = entry_a.txid_prefix();
        store.insert(tx_a, entry_a);
        store.insert(tx_b, entry_b);

        let total_after_both: u32 = store.live_histogram().iter().sum();
        assert_eq!(total_after_both, 3, "two outputs + one output");

        store.remove_by_prefix(&prefix_a);
        let total_after_remove: u32 = store.live_histogram().iter().sum();
        assert_eq!(total_after_remove, 1);
    }
}
