use brk_types::TxidPrefix;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use super::{Entry, TxIndex};

/// Pool of mempool entries with slot recycling.
///
/// Slot-based storage: removed entries leave holes that are reused
/// by the next insert, so `TxIndex` stays stable for the lifetime of
/// an entry. Only stores what can't be derived: the entries
/// themselves, their prefix-to-slot index, and the free slot list.
#[derive(Default)]
pub struct EntryPool {
    entries: Vec<Option<Entry>>,
    prefix_to_idx: FxHashMap<TxidPrefix, TxIndex>,
    free_slots: Vec<TxIndex>,
}

impl EntryPool {
    /// Insert an entry, returning its index. The prefix is derived from
    /// `entry.txid`, so the caller never has to pass it in.
    pub fn insert(&mut self, entry: Entry) -> TxIndex {
        let prefix = entry.txid_prefix();
        let idx = match self.free_slots.pop() {
            Some(idx) => {
                self.entries[idx.as_usize()] = Some(entry);
                idx
            }
            None => {
                let idx = TxIndex::from(self.entries.len());
                self.entries.push(Some(entry));
                idx
            }
        };

        self.prefix_to_idx.insert(prefix, idx);
        idx
    }

    /// Get an entry by its txid prefix.
    pub fn get(&self, prefix: &TxidPrefix) -> Option<&Entry> {
        let idx = self.prefix_to_idx.get(prefix)?;
        self.entries.get(idx.as_usize())?.as_ref()
    }

    /// Direct children of a transaction (txs whose `depends` includes
    /// `prefix`). Derived on demand via a linear scan — called only by
    /// the CPFP query endpoint, which is not on the hot path.
    pub fn children(&self, prefix: &TxidPrefix) -> SmallVec<[TxidPrefix; 2]> {
        let mut out: SmallVec<[TxidPrefix; 2]> = SmallVec::new();
        for entry in self.entries.iter().flatten() {
            if entry.depends.iter().any(|p| p == prefix) {
                out.push(entry.txid_prefix());
            }
        }
        out
    }

    /// Remove an entry by its txid prefix, returning it if present.
    pub fn remove(&mut self, prefix: &TxidPrefix) -> Option<Entry> {
        let idx = self.prefix_to_idx.remove(prefix)?;
        let entry = self.entries.get_mut(idx.as_usize()).and_then(Option::take)?;
        self.free_slots.push(idx);
        Some(entry)
    }

    /// Get the entries slice for block building.
    pub fn entries(&self) -> &[Option<Entry>] {
        &self.entries
    }
}
