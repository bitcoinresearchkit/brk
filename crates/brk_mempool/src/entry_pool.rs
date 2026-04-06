use brk_types::TxidPrefix;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use crate::{entry::Entry, types::TxIndex};

/// Pool of mempool entries with slot recycling.
///
/// Uses a slot-based storage where removed entries leave holes
/// that get reused for new entries, avoiding index invalidation.
#[derive(Default)]
pub struct EntryPool {
    entries: Vec<Option<Entry>>,
    prefix_to_idx: FxHashMap<TxidPrefix, TxIndex>,
    parent_to_children: FxHashMap<TxidPrefix, SmallVec<[TxidPrefix; 2]>>,
    free_slots: Vec<TxIndex>,
}

impl EntryPool {
    /// Insert an entry, returning its index.
    pub fn insert(&mut self, prefix: TxidPrefix, entry: Entry) -> TxIndex {
        for parent in &entry.depends {
            self.parent_to_children
                .entry(*parent)
                .or_default()
                .push(prefix);
        }

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

    /// Get direct children of a transaction (txs that depend on it).
    pub fn children(&self, prefix: &TxidPrefix) -> &[TxidPrefix] {
        self.parent_to_children
            .get(prefix)
            .map(SmallVec::as_slice)
            .unwrap_or_default()
    }

    /// Remove an entry by its txid prefix.
    pub fn remove(&mut self, prefix: &TxidPrefix) {
        if let Some(idx) = self.prefix_to_idx.remove(prefix) {
            if let Some(entry) = self.entries.get(idx.as_usize()).and_then(|e| e.as_ref()) {
                for parent in &entry.depends {
                    if let Some(children) = self.parent_to_children.get_mut(parent) {
                        children.retain(|c| c != prefix);
                        if children.is_empty() {
                            self.parent_to_children.remove(parent);
                        }
                    }
                }
            }
            self.parent_to_children.remove(prefix);
            if let Some(slot) = self.entries.get_mut(idx.as_usize()) {
                *slot = None;
            }
            self.free_slots.push(idx);
        }
    }

    /// Get the entries slice for block building.
    pub fn entries(&self) -> &[Option<Entry>] {
        &self.entries
    }
}
