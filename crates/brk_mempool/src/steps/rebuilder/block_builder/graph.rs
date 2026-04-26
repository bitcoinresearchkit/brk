use std::ops::{Index, IndexMut};

use brk_types::TxidPrefix;
use rustc_hash::FxHashMap;

use super::{pool_index::PoolIndex, tx_node::TxNode};
use crate::stores::{Entry, TxIndex};

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
    // Pass 1: collect live entries and index their prefixes in lockstep.
    // We can't resolve parent links yet because a parent may sit later in
    // slot order than its child, so prefix_to_pool needs to be complete
    // before we touch `entry.depends`.
    let mut live: Vec<(TxIndex, &Entry)> = Vec::with_capacity(entries.len());
    let mut prefix_to_pool: FxHashMap<TxidPrefix, PoolIndex> =
        FxHashMap::with_capacity_and_hasher(entries.len(), Default::default());
    for (i, opt) in entries.iter().enumerate() {
        if let Some(e) = opt.as_ref() {
            prefix_to_pool.insert(e.txid_prefix(), PoolIndex::from(live.len()));
            live.push((TxIndex::from(i), e));
        }
    }

    if live.is_empty() {
        return Graph(Vec::new());
    }

    // Pass 2: materialize nodes with their parent edges.
    let mut nodes: Vec<TxNode> = live
        .iter()
        .map(|(tx_index, entry)| {
            let mut node = TxNode::new(*tx_index, entry.fee, entry.vsize);
            for parent_prefix in &entry.depends {
                if let Some(&parent_pool_idx) = prefix_to_pool.get(parent_prefix) {
                    node.parents.push(parent_pool_idx);
                }
            }
            node
        })
        .collect();

    // Pass 3: mirror parent edges as children. Direct indexing only;
    // no intermediate edge vec.
    for i in 0..nodes.len() {
        let plen = nodes[i].parents.len();
        for j in 0..plen {
            let parent_idx = nodes[i].parents[j].as_usize();
            nodes[parent_idx].children.push(PoolIndex::from(i));
        }
    }

    Graph(nodes)
}
