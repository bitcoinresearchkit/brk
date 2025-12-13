use brk_types::{Sats, Transaction, Txid, VSize, Vout};
use rustc_hash::{FxHashMap, FxHashSet};

use super::entry::MempoolOutpoint;
use super::MempoolEntry;

/// Transaction dependency graph for the mempool
///
/// Tracks parent-child relationships and computes ancestor feerates
/// for proper CPFP (Child-Pays-For-Parent) handling.
#[derive(Debug, Default)]
pub struct TxGraph {
    /// All mempool entries by txid
    entries: FxHashMap<Txid, MempoolEntry>,

    /// Maps outpoint -> txid that created it (for finding parents)
    outpoint_to_tx: FxHashMap<MempoolOutpoint, Txid>,

    /// Maps txid -> txids that spend its outputs (children)
    children: FxHashMap<Txid, FxHashSet<Txid>>,
}

impl TxGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entries(&self) -> &FxHashMap<Txid, MempoolEntry> {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Add a transaction to the graph
    pub fn insert(&mut self, tx: &Transaction) {
        let mut entry = MempoolEntry::new(tx);

        // Find in-mempool parents and build ancestor set
        let parents = self.find_parents(&entry.spends);
        entry.ancestors = self.compute_ancestors(&parents);

        // Compute ancestor fee/vsize
        let (ancestor_fee, ancestor_vsize) = self.sum_ancestors(&entry.ancestors);
        entry.ancestor_fee = entry.fee + ancestor_fee;
        entry.ancestor_vsize = entry.vsize + ancestor_vsize;

        // Register this tx's outputs
        for (vout, _) in tx.output.iter().enumerate() {
            let outpoint = (entry.txid.clone(), Vout::from(vout as u32));
            self.outpoint_to_tx.insert(outpoint, entry.txid.clone());
        }

        // Register as child of parents
        for parent in &parents {
            self.children
                .entry(parent.clone())
                .or_default()
                .insert(entry.txid.clone());
        }

        self.entries.insert(entry.txid.clone(), entry);
    }

    /// Remove a transaction from the graph
    pub fn remove(&mut self, txid: &Txid) -> Option<MempoolEntry> {
        let entry = self.entries.remove(txid)?;

        // Remove from outpoint index
        // Note: We don't know the vout count, so we remove all entries pointing to this txid
        self.outpoint_to_tx.retain(|_, tx| tx != txid);

        // Remove from children index
        self.children.remove(txid);
        for children_set in self.children.values_mut() {
            children_set.remove(txid);
        }

        // Update descendants' ancestor data
        self.update_descendants_after_removal(txid, &entry);

        Some(entry)
    }

    /// Check if a txid is in the mempool
    pub fn contains(&self, txid: &Txid) -> bool {
        self.entries.contains_key(txid)
    }

    /// Get all txids currently in the graph
    pub fn txids(&self) -> impl Iterator<Item = &Txid> {
        self.entries.keys()
    }

    /// Find which inputs reference in-mempool transactions (parents)
    fn find_parents(&self, spends: &[MempoolOutpoint]) -> Vec<Txid> {
        spends
            .iter()
            .filter_map(|outpoint| self.outpoint_to_tx.get(outpoint).cloned())
            .collect()
    }

    /// Compute full ancestor set (transitive closure)
    fn compute_ancestors(&self, parents: &[Txid]) -> FxHashSet<Txid> {
        let mut ancestors = FxHashSet::default();
        let mut stack: Vec<Txid> = parents.to_vec();

        while let Some(txid) = stack.pop() {
            if ancestors.insert(txid.clone()) {
                if let Some(entry) = self.entries.get(&txid) {
                    stack.extend(entry.ancestors.iter().cloned());
                }
            }
        }

        ancestors
    }

    /// Sum fee and vsize of all ancestors
    fn sum_ancestors(&self, ancestors: &FxHashSet<Txid>) -> (Sats, VSize) {
        ancestors.iter().fold(
            (Sats::default(), VSize::default()),
            |(fee, vsize), txid| {
                if let Some(entry) = self.entries.get(txid) {
                    (fee + entry.fee, vsize + entry.vsize)
                } else {
                    (fee, vsize)
                }
            },
        )
    }

    /// Update all descendants after removing a transaction
    fn update_descendants_after_removal(&mut self, removed: &Txid, removed_entry: &MempoolEntry) {
        // Find all descendants
        let descendants = self.find_descendants(removed);

        // Update each descendant's ancestor set and cumulative values
        for desc_txid in descendants {
            if let Some(desc) = self.entries.get_mut(&desc_txid) {
                // Remove the removed tx from ancestors
                desc.ancestors.remove(removed);

                // Subtract the removed tx's contribution
                desc.ancestor_fee = desc.ancestor_fee - removed_entry.fee;
                desc.ancestor_vsize = desc.ancestor_vsize - removed_entry.vsize;
            }
        }
    }

    /// Find all descendants of a transaction (children, grandchildren, etc.)
    fn find_descendants(&self, txid: &Txid) -> Vec<Txid> {
        let mut descendants = Vec::new();
        let mut stack = vec![txid.clone()];
        let mut visited = FxHashSet::default();

        while let Some(current) = stack.pop() {
            if let Some(children) = self.children.get(&current) {
                for child in children {
                    if visited.insert(child.clone()) {
                        descendants.push(child.clone());
                        stack.push(child.clone());
                    }
                }
            }
        }

        descendants
    }
}
