use std::ops::{Index, IndexMut};

use brk_types::TxidPrefix;
use rustc_hash::FxHashMap;

use super::tx_node::TxNode;
use crate::{
    entry::Entry,
    types::{PoolIndex, TxIndex},
};

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
    let mut live: Vec<(TxIndex, &Entry)> = Vec::with_capacity(entries.len());
    for (i, opt) in entries.iter().enumerate() {
        if let Some(e) = opt.as_ref() {
            live.push((TxIndex::from(i), e));
        }
    }

    if live.is_empty() {
        return Graph(Vec::new());
    }

    let mut prefix_to_pool: FxHashMap<TxidPrefix, PoolIndex> =
        FxHashMap::with_capacity_and_hasher(live.len(), Default::default());
    for (i, (_, entry)) in live.iter().enumerate() {
        prefix_to_pool.insert(entry.txid_prefix(), PoolIndex::from(i));
    }

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

    // Populate children via direct indexing; no intermediate edge vec.
    // Reading parents[j] as a Copy value releases the immutable borrow
    // before the mutable borrow of children's owner.
    for i in 0..nodes.len() {
        let plen = nodes[i].parents.len();
        for j in 0..plen {
            let parent_idx = nodes[i].parents[j].as_usize();
            nodes[parent_idx].children.push(PoolIndex::from(i));
        }
    }

    Graph(nodes)
}

#[cfg(test)]
mod bench {
    use std::time::Instant;

    use bitcoin::hashes::Hash;
    use brk_types::{Sats, Timestamp, Txid, VSize};
    use smallvec::SmallVec;

    use super::build_graph;
    use crate::entry::Entry;

    /// Synthetic mempool: mostly singletons, some CPFP chains/trees.
    fn synthetic_mempool(n: usize) -> Vec<Option<Entry>> {
        let make_txid = |i: usize| -> Txid {
            let mut bytes = [0u8; 32];
            bytes[0..8].copy_from_slice(&(i as u64).to_ne_bytes());
            bytes[8..16].copy_from_slice(&((i as u64).wrapping_mul(2654435761)).to_ne_bytes());
            Txid::from(bitcoin::Txid::from_slice(&bytes).unwrap())
        };

        let mut entries: Vec<Option<Entry>> = Vec::with_capacity(n);
        let mut txids: Vec<Txid> = Vec::with_capacity(n);
        for i in 0..n {
            let txid = make_txid(i);
            txids.push(txid.clone());

            // 95% singletons, 4% 1-parent, 1% 2-parent (mimics real mempool).
            let depends: SmallVec<[brk_types::TxidPrefix; 2]> = match i % 100 {
                0..=94 => SmallVec::new(),
                95..=98 if i > 0 => {
                    let p = (i.wrapping_mul(7919)) % i;
                    std::iter::once(brk_types::TxidPrefix::from(&txids[p])).collect()
                }
                _ if i > 1 => {
                    let p1 = (i.wrapping_mul(7919)) % i;
                    let p2 = (i.wrapping_mul(6151)) % i;
                    [
                        brk_types::TxidPrefix::from(&txids[p1]),
                        brk_types::TxidPrefix::from(&txids[p2]),
                    ]
                    .into_iter()
                    .collect()
                }
                _ => SmallVec::new(),
            };

            entries.push(Some(Entry {
                txid,
                fee: Sats::from((i as u64).wrapping_mul(137) % 10_000 + 1),
                vsize: VSize::from(250u64),
                size: 250,
                ancestor_fee: Sats::from(0u64),
                ancestor_vsize: VSize::from(250u64),
                depends,
                first_seen: Timestamp::now(),
            }));
        }
        entries
    }

    #[test]
    #[ignore = "perf benchmark; run with --ignored --nocapture"]
    fn perf_build_graph() {
        let sizes = [1_000usize, 10_000, 50_000, 100_000, 300_000];
        eprintln!();
        eprintln!("build_graph perf (release, single call):");
        eprintln!("  n          build");
        eprintln!("  ------------------------");
        for &n in &sizes {
            let entries = synthetic_mempool(n);
            // Warm up allocator.
            let _ = build_graph(&entries);

            let t = Instant::now();
            let g = build_graph(&entries);
            let dt = t.elapsed();
            let ns = dt.as_nanos();
            let pretty = if ns >= 1_000_000 {
                format!("{:.2} ms", ns as f64 / 1_000_000.0)
            } else {
                format!("{:.2} µs", ns as f64 / 1_000.0)
            };
            eprintln!("  {:<10} {:<10} ({} nodes)", n, pretty, g.len());
        }
        eprintln!();
    }
}
