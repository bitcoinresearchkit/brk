mod pool_index;
mod tx_node;

pub use pool_index::PoolIndex;
pub use tx_node::TxNode;

use brk_types::TxidPrefix;
use rustc_hash::{FxBuildHasher, FxHashMap};

use crate::{TxEntry, stores::TxIndex};

pub struct Graph;

impl Graph {
    /// Build the dependency graph for the live mempool.
    ///
    /// Nodes are indexed by `PoolIndex`; the caller indexes with
    /// `idx.as_usize()`.
    pub fn build(entries: &[Option<TxEntry>]) -> Vec<TxNode> {
        let (live, prefix_to_pool) = Self::index_live(entries);
        if live.is_empty() {
            return Vec::new();
        }
        let mut nodes = Self::build_parent_edges(&live, &prefix_to_pool);
        Self::mirror_child_edges(&mut nodes);
        nodes
    }

    /// First pass: collect live entries and map their prefixes to pool
    /// indexes. Done before parent edges so a parent appearing later in
    /// slot order than its child is still resolvable.
    fn index_live(
        entries: &[Option<TxEntry>],
    ) -> (Vec<(TxIndex, &TxEntry)>, FxHashMap<TxidPrefix, PoolIndex>) {
        let mut live: Vec<(TxIndex, &TxEntry)> = Vec::with_capacity(entries.len());
        let mut prefix_to_pool: FxHashMap<TxidPrefix, PoolIndex> =
            FxHashMap::with_capacity_and_hasher(entries.len(), FxBuildHasher);
        for (i, opt) in entries.iter().enumerate() {
            if let Some(e) = opt.as_ref() {
                prefix_to_pool.insert(e.txid_prefix(), PoolIndex::from(live.len()));
                live.push((TxIndex::from(i), e));
            }
        }
        (live, prefix_to_pool)
    }

    fn build_parent_edges(
        live: &[(TxIndex, &TxEntry)],
        prefix_to_pool: &FxHashMap<TxidPrefix, PoolIndex>,
    ) -> Vec<TxNode> {
        live.iter()
            .map(|(tx_index, entry)| {
                let mut node = TxNode::new(*tx_index, entry.fee, entry.vsize);
                for parent_prefix in &entry.depends {
                    if let Some(&parent_pool_idx) = prefix_to_pool.get(parent_prefix) {
                        node.parents.push(parent_pool_idx);
                    }
                }
                node
            })
            .collect()
    }

    fn mirror_child_edges(nodes: &mut [TxNode]) {
        for i in 0..nodes.len() {
            let plen = nodes[i].parents.len();
            for j in 0..plen {
                let parent_idx = nodes[i].parents[j].as_usize();
                nodes[parent_idx].children.push(PoolIndex::from(i));
            }
        }
    }
}
