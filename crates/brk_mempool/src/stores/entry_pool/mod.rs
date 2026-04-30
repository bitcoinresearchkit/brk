use brk_types::TxidPrefix;
use rustc_hash::FxHashMap;

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
        self.slot(self.idx_of(prefix)?)
    }

    /// Slot index for a prefix, or `None` if not in the pool.
    pub fn idx_of(&self, prefix: &TxidPrefix) -> Option<TxIndex> {
        self.prefix_to_idx.get(prefix).copied()
    }

    /// Direct slot read by index. `None` if the slot is empty or the
    /// index is out of range.
    pub fn slot(&self, idx: TxIndex) -> Option<&TxEntry> {
        self.entries.get(idx.as_usize())?.as_ref()
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
