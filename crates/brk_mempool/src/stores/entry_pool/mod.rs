use brk_types::TxidPrefix;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

mod tx_index;

pub use tx_index::TxIndex;

use crate::TxEntry;

/// Pool of mempool entries with slot recycling.
///
/// Slot-based storage: removed entries leave holes that are reused
/// by the next insert, so `TxIndex` stays stable for the lifetime of
/// an entry. Only stores what can't be derived: the entries
/// themselves, their prefix-to-slot index, and the free slot list.
#[derive(Default)]
pub struct EntryPool {
    entries: Vec<Option<TxEntry>>,
    prefix_to_idx: FxHashMap<TxidPrefix, TxIndex>,
    free_slots: Vec<TxIndex>,
}

impl EntryPool {
    pub fn insert(&mut self, entry: TxEntry) {
        let prefix = entry.txid_prefix();
        let idx = self.claim_slot(entry);
        self.prefix_to_idx.insert(prefix, idx);
    }

    fn claim_slot(&mut self, entry: TxEntry) -> TxIndex {
        if let Some(idx) = self.free_slots.pop() {
            self.entries[idx.as_usize()] = Some(entry);
            idx
        } else {
            let idx = TxIndex::from(self.entries.len());
            self.entries.push(Some(entry));
            idx
        }
    }

    pub fn get(&self, prefix: &TxidPrefix) -> Option<&TxEntry> {
        let idx = self.prefix_to_idx.get(prefix)?;
        self.entries.get(idx.as_usize())?.as_ref()
    }

    /// Direct children of a transaction (txs whose `depends` includes
    /// `prefix`). Derived on demand via a linear scan, called only by
    /// the CPFP query endpoint, which is not on the hot path.
    pub fn children(&self, prefix: &TxidPrefix) -> SmallVec<[TxidPrefix; 2]> {
        self.entries
            .iter()
            .flatten()
            .filter(|e| e.depends.iter().any(|p| p == prefix))
            .map(TxEntry::txid_prefix)
            .collect()
    }

    pub fn remove(&mut self, prefix: &TxidPrefix) -> Option<TxEntry> {
        let idx = self.prefix_to_idx.remove(prefix)?;
        let entry = self.entries.get_mut(idx.as_usize())?.take()?;
        self.free_slots.push(idx);
        Some(entry)
    }

    pub fn entries(&self) -> &[Option<TxEntry>] {
        &self.entries
    }
}
