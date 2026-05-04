//! CPFP (Child Pays For Parent) cluster reasoning.
//!
//! Two consumers, one shared converter:
//!
//! - **Mempool path** (`Mempool::cpfp_info`): looks up the seed in the
//!   `Snapshot.cluster_of` map, which already contains the SFL-linearized
//!   connected component built once per snapshot cycle. No graph walk,
//!   no SFL recomputation.
//! - **Confirmed path** (`brk_query::Query::confirmed_cpfp`): builds a
//!   `Cluster` from same-block parent/child edges on demand.
//!
//! Both feed `Cluster::to_cpfp_info`, which walks the cluster from the
//! seed (parents → ancestors, topo-sweep → descendants), reads the seed's
//! chunk feerate as `effectiveFeePerVsize`, and emits the wire shape.
//!
//! The cluster spans the full connected component (matches mempool.space);
//! we don't scope to the seed's projected block, which would drop info
//! when a cluster crosses the projection floor.

use brk_types::{
    CpfpCluster, CpfpClusterChunk, CpfpClusterTx, CpfpEntry, CpfpInfo, FeeRate, SigOps, TxidPrefix,
    VSize,
};


use crate::Mempool;
use crate::cluster::{Cluster, ClusterRef, LocalIdx};

impl<I> Cluster<I> {
    /// Wire-shape `CpfpInfo` for `seed` inside this cluster. `txid` and
    /// `weight` come straight off each `ClusterNode`, so the converter
    /// is self-contained — no parallel `members` slice required.
    pub fn to_cpfp_info(&self, seed: LocalIdx, sigops: SigOps) -> CpfpInfo {
        let descendants = self.walk_descendants(seed);
        let best_descendant = descendants
            .iter()
            .max_by_key(|e| FeeRate::from((e.fee, e.weight)))
            .cloned();
        let seed_node = &self.nodes[seed.as_usize()];

        let vsize = VSize::from(seed_node.weight);
        let adjusted_vsize = sigops.adjust_vsize(vsize);

        CpfpInfo {
            ancestors: self.walk_ancestors(seed),
            best_descendant,
            descendants,
            effective_fee_per_vsize: self.chunk_of(seed).fee_rate(),
            sigops,
            fee: seed_node.fee,
            vsize,
            adjusted_vsize,
            cluster: self.cluster_view(seed),
        }
    }

    /// DFS up the parent edges from `seed`, exclusive. Cluster size is
    /// capped at 128 by SFL, so a `u128` covers the visited set.
    fn walk_ancestors(&self, seed: LocalIdx) -> Vec<CpfpEntry> {
        let mut visited = 1u128 << seed.inner();
        let mut out: Vec<CpfpEntry> = Vec::new();
        let mut stack: Vec<LocalIdx> = self.nodes[seed.as_usize()].parents.to_vec();
        while let Some(idx) = stack.pop() {
            let b = 1u128 << idx.inner();
            if visited & b != 0 {
                continue;
            }
            visited |= b;
            let node = &self.nodes[idx.as_usize()];
            out.push(CpfpEntry::from(node));
            stack.extend(node.parents.iter().copied());
        }
        out
    }

    /// Forward sweep over the topo-ordered tail after `seed`. A node is
    /// a descendant iff any of its parents is `seed` or already-reached.
    /// Nodes before `seed` can't reach it, so they're skipped entirely.
    fn walk_descendants(&self, seed: LocalIdx) -> Vec<CpfpEntry> {
        let seed_pos = seed.as_usize();
        let mut reachable = 1u128 << seed.inner();
        let mut out: Vec<CpfpEntry> = Vec::new();
        for (i, node) in self.nodes.iter().enumerate().skip(seed_pos + 1) {
            if node.parents.iter().any(|&p| reachable & (1u128 << p.inner()) != 0) {
                reachable |= 1u128 << i;
                out.push(CpfpEntry::from(node));
            }
        }
        out
    }

    /// Wire-shape `CpfpCluster`. Cluster nodes are stored in topological
    /// order, so `LocalIdx` maps directly onto `CpfpClusterTxIndex`
    /// without a permutation lookup.
    fn cluster_view(&self, seed: LocalIdx) -> CpfpCluster {
        CpfpCluster {
            txs: self.nodes.iter().map(CpfpClusterTx::from).collect(),
            chunks: self.chunks.iter().map(CpfpClusterChunk::from).collect(),
            chunk_index: self.node_to_chunk[seed.as_usize()].inner(),
        }
    }
}

impl Mempool {
    /// CPFP info for a live mempool tx. Returns `None` only when the
    /// tx isn't in the mempool, so callers can fall through to the
    /// confirmed path.
    pub fn cpfp_info(&self, prefix: &TxidPrefix) -> Option<CpfpInfo> {
        let snapshot = self.snapshot();
        let seed_idx = self.entries().idx_of(prefix)?;
        let ClusterRef { cluster_id, local: seed_local } = snapshot.cluster_of(seed_idx)?;
        let cluster = &snapshot.clusters[cluster_id.as_usize()];
        let seed_txid = &cluster.nodes[seed_local.as_usize()].txid;

        let sigops = self
            .txs()
            .get(seed_txid)
            .map(|tx| tx.total_sigop_cost)
            .unwrap_or(SigOps::ZERO);

        Some(cluster.to_cpfp_info(seed_local, sigops))
    }
}
