//! CPFP (Child Pays For Parent) cluster reasoning for live mempool
//! transactions. Cluster scope is the seed's projected block: txs in
//! other projected blocks share no mining fate with the seed, so
//! including them in `effectiveFeePerVsize` would be misleading.
//!
//! Confirmed-tx CPFP (the same-block connected component on the
//! chain) lives in `brk_query`, since it reads indexer/computer vecs.

use brk_types::{
    CpfpCluster, CpfpClusterChunk, CpfpClusterTx, CpfpClusterTxIndex, CpfpEntry, CpfpInfo, FeeRate,
    TxidPrefix, VSize, Weight,
};
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;

use crate::steps::rebuilder::linearize::{
    LocalIdx, cluster::Cluster, cluster_node::ClusterNode, sfl::Sfl,
};
use crate::stores::{EntryPool, TxIndex};
use crate::{Mempool, TxEntry};

/// Cap matches Bitcoin Core's default mempool ancestor/descendant
/// chain limits and `confirmed_cpfp`'s cap.
const MAX: usize = 25;

impl Mempool {
    /// CPFP info for a live mempool tx, scoped to the seed's projected
    /// block. Returns `None` if the tx is not in the mempool, so
    /// callers can fall through to the confirmed path. Returns `Some`
    /// with empty arms if the tx is in the mempool but below the
    /// projection floor (no projected block to share fate with).
    pub fn cpfp_info(&self, prefix: &TxidPrefix) -> Option<CpfpInfo> {
        let snapshot = self.snapshot();
        let entries = self.entries();
        let txs = self.txs();
        let seed_idx = entries.idx_of(prefix)?;
        let seed = entries.slot(seed_idx)?;

        let mut ancestor_idxs: Vec<TxIndex> = Vec::new();
        let mut descendant_idxs: Vec<TxIndex> = Vec::new();
        let mut ancestors: Vec<CpfpEntry> = Vec::new();
        let mut descendants: Vec<CpfpEntry> = Vec::new();

        if let Some(seed_block) = snapshot.block_of(seed_idx) {
            let mut visited: FxHashSet<TxidPrefix> = FxHashSet::default();
            visited.insert(*prefix);
            let mut stack: Vec<TxidPrefix> = seed.depends.iter().copied().collect();
            while let Some(p) = stack.pop() {
                if ancestors.len() >= MAX {
                    break;
                }
                if !visited.insert(p) {
                    continue;
                }
                let Some(idx) = entries.idx_of(&p) else { continue };
                if snapshot.block_of(idx) != Some(seed_block) {
                    continue;
                }
                let Some(anc) = entries.slot(idx) else { continue };
                ancestor_idxs.push(idx);
                ancestors.push(to_entry(anc));
                stack.extend(anc.depends.iter().copied());
            }

            let mut desc_set: FxHashSet<TxidPrefix> = FxHashSet::default();
            desc_set.insert(*prefix);
            for &i in &snapshot.blocks[seed_block.as_usize()] {
                if descendants.len() >= MAX {
                    break;
                }
                let Some(e) = entries.slot(i) else { continue };
                if !e.depends.iter().any(|d| desc_set.contains(d)) {
                    continue;
                }
                desc_set.insert(e.txid_prefix());
                descendant_idxs.push(i);
                descendants.push(to_entry(e));
            }
        }

        let best_descendant = descendants
            .iter()
            .max_by_key(|e| FeeRate::from((e.fee, e.weight)))
            .cloned();

        let sigops = txs.get(&seed.txid).map(|tx| {
            // Bitcoin Core's `total_sigop_cost` is the segwit-weighted sigop
            // count (legacy * 4 + segwit * 1), divided by 5 to match
            // mempool.space's reported `sigops`. Mempool.space converts
            // back to count via `sigopcost / 5`.
            u32::try_from(tx.total_sigop_cost / 5).unwrap_or(u32::MAX)
        });

        // mempool.space's adjustedVsize = max(vsize, sigops * 5).
        let adjusted_vsize = match sigops {
            Some(s) => VSize::from(u64::from(seed.vsize).max(u64::from(s) * 5)),
            None => seed.vsize,
        };

        let cluster = build_cluster(seed_idx, &ancestor_idxs, &descendant_idxs, &entries);

        // mempool.space sets effectiveFeePerVsize to the seed's chunk feerate
        // when the cluster is known, falls back to the seed's own rate.
        let effective = cluster
            .as_ref()
            .and_then(|c| c.chunks.get(c.chunk_index as usize))
            .map(|chunk| chunk.feerate)
            .unwrap_or_else(|| seed.fee_rate());

        Some(CpfpInfo {
            ancestors,
            best_descendant,
            descendants,
            effective_fee_per_vsize: Some(effective),
            sigops,
            fee: Some(seed.fee),
            adjusted_vsize: Some(adjusted_vsize),
            cluster,
        })
    }
}

fn to_entry(e: &TxEntry) -> CpfpEntry {
    CpfpEntry {
        txid: e.txid.clone(),
        weight: Weight::from(e.vsize),
        fee: e.fee,
    }
}

/// Build the cluster output: seed + ancestors + descendants in topological
/// order, with parent indexes inside the cluster, plus SFL-linearized chunks.
fn build_cluster(
    seed_idx: TxIndex,
    ancestor_idxs: &[TxIndex],
    descendant_idxs: &[TxIndex],
    entries: &EntryPool,
) -> Option<CpfpCluster> {
    let mut ordered: Vec<TxIndex> = Vec::with_capacity(ancestor_idxs.len() + 1 + descendant_idxs.len());
    ordered.extend(ancestor_idxs.iter().copied());
    ordered.push(seed_idx);
    ordered.extend(descendant_idxs.iter().copied());

    let pool: Vec<&TxEntry> = ordered.iter().filter_map(|&i| entries.slot(i)).collect();
    if pool.len() != ordered.len() {
        return None;
    }

    let prefix_to_local: FxHashMap<TxidPrefix, LocalIdx> = pool
        .iter()
        .enumerate()
        .map(|(i, e)| (e.txid_prefix(), i as LocalIdx))
        .collect();

    let mut children_of: Vec<SmallVec<[LocalIdx; 2]>> = vec![SmallVec::new(); pool.len()];
    let parents_of: Vec<SmallVec<[LocalIdx; 2]>> = pool
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let parents: SmallVec<[LocalIdx; 2]> = e
                .depends
                .iter()
                .filter_map(|p| prefix_to_local.get(p).copied())
                .collect();
            for &p in &parents {
                children_of[p as usize].push(i as LocalIdx);
            }
            parents
        })
        .collect();

    let cluster_nodes: Vec<ClusterNode> = pool
        .iter()
        .enumerate()
        .map(|(i, e)| ClusterNode {
            tx_index: ordered[i],
            fee: e.fee,
            vsize: e.vsize,
            parents: parents_of[i].clone(),
            children: children_of[i].clone(),
        })
        .collect();

    let cluster = Cluster::new(cluster_nodes);

    // Re-order pool so parents come before children (mempool.space convention).
    // `topo_rank[i]` gives the position of local index `i` in topological order.
    let mut local_to_topo: Vec<usize> = (0..pool.len()).collect();
    local_to_topo.sort_unstable_by_key(|&i| cluster.topo_rank[i]);
    let topo_to_local: Vec<usize> = {
        let mut v = vec![0usize; pool.len()];
        for (topo_pos, &local) in local_to_topo.iter().enumerate() {
            v[local] = topo_pos;
        }
        v
    };

    let topo_idx = |local: usize| CpfpClusterTxIndex::from(topo_to_local[local] as u32);

    let txs: Vec<CpfpClusterTx> = local_to_topo
        .iter()
        .map(|&local| {
            let e = pool[local];
            let parents: Vec<CpfpClusterTxIndex> = parents_of[local]
                .iter()
                .map(|&p| topo_idx(p as usize))
                .collect();
            CpfpClusterTx {
                txid: e.txid.clone(),
                fee: e.fee,
                weight: Weight::from(e.vsize),
                parents,
            }
        })
        .collect();

    let raw_chunks = Sfl::linearize(&cluster);
    let chunks: Vec<CpfpClusterChunk> = raw_chunks
        .iter()
        .map(|chunk| {
            let mut chunk_txs: Vec<CpfpClusterTxIndex> = chunk
                .nodes
                .iter()
                .map(|&local| topo_idx(local as usize))
                .collect();
            chunk_txs.sort_unstable();
            CpfpClusterChunk {
                txs: chunk_txs,
                feerate: chunk.fee_rate(),
            }
        })
        .collect();

    let seed_local = *prefix_to_local.get(&entries.slot(seed_idx)?.txid_prefix())?;
    let seed_topo = topo_idx(seed_local as usize);
    let chunk_index = chunks
        .iter()
        .position(|c| c.txs.contains(&seed_topo))
        .unwrap_or(0) as u32;

    Some(CpfpCluster {
        txs,
        chunks,
        chunk_index,
    })
}
