use std::ops::{Index, IndexMut};

use brk_types::TxidPrefix;
use rustc_hash::FxHashMap;

use super::tx_node::TxNode;
use crate::{entry::Entry, types::{PoolIndex, TxIndex}};

/// Type-safe wrapper around Vec<TxNode> that only allows PoolIndex access.
pub struct Graph(Vec<TxNode>);

impl Graph {
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Index<PoolIndex> for Graph {
    type Output = TxNode;

    #[inline]
    fn index(&self, idx: PoolIndex) -> &Self::Output {
        &self.0[idx.as_usize()]
    }
}

impl IndexMut<PoolIndex> for Graph {
    #[inline]
    fn index_mut(&mut self, idx: PoolIndex) -> &mut Self::Output {
        &mut self.0[idx.as_usize()]
    }
}

/// Build a dependency graph from mempool entries.
pub fn build_graph(entries: &[Option<Entry>]) -> Graph {
    // Collect live entries with their indices
    let live: Vec<(TxIndex, &Entry)> = entries
        .iter()
        .enumerate()
        .filter_map(|(i, opt)| opt.as_ref().map(|e| (TxIndex::from(i), e)))
        .collect();

    if live.is_empty() {
        return Graph(Vec::new());
    }

    // Map TxidPrefix -> PoolIndex for parent lookups
    let prefix_to_pool: FxHashMap<TxidPrefix, PoolIndex> = live
        .iter()
        .enumerate()
        .map(|(i, (_, entry))| (entry.txid_prefix(), PoolIndex::from(i)))
        .collect();

    // Build nodes with parent relationships
    let mut nodes: Vec<TxNode> = live
        .iter()
        .enumerate()
        .map(|(pool_idx, (tx_index, entry))| {
            let pool_index = PoolIndex::from(pool_idx);
            let mut node = TxNode::new(
                *tx_index,
                pool_index,
                entry.fee,
                entry.vsize,
                entry.ancestor_fee,
                entry.ancestor_vsize,
            );

            // Add in-mempool parents
            for parent_prefix in &entry.depends {
                if let Some(&parent_pool_idx) = prefix_to_pool.get(parent_prefix) {
                    node.parents.push(parent_pool_idx);
                }
            }

            node
        })
        .collect();

    // Collect parent->child edges (avoids cloning each node's parents)
    let edges: Vec<(usize, PoolIndex)> = nodes
        .iter()
        .enumerate()
        .flat_map(|(i, node)| {
            node.parents
                .iter()
                .map(move |&p| (p.as_usize(), PoolIndex::from(i)))
        })
        .collect();

    // Build child relationships
    for (parent_idx, child_idx) in edges {
        nodes[parent_idx].children.push(child_idx);
    }

    Graph(nodes)
}
