//! CPFP (Child Pays For Parent) walk over a `Snapshot`'s adjacency.
//!
//! The snapshot stores per-tx parent/child edges in `TxIndex` space and
//! per-tx `(fee, vsize)` we need for chunking.
//!
//! Three independent walks:
//! - `ancestors_idx`: capped DFS up `parents` only.
//! - `descendants_idx`: capped DFS down `children` only.
//! - cluster `members`: capped DFS over `parents ∪ children`, i.e. the
//!   connected component of the seed in the in-mempool dependency
//!   graph. Required to match Core 31's cluster mempool semantics:
//!   siblings (sharing a parent) and cousins (sharing a descendant)
//!   belong to the same cluster but are missed by ancestor/descendant
//!   walks alone.
//!
//! The cluster is then linearized via `brk_types::linearize` (single fee
//! linearization) so chunks reflect Core's CPFP "lift": a child whose
//! rate exceeds its parent's gets folded into a chunk with the parent
//! at the combined feerate. The seed's chunk feerate is what
//! `effective_fee_per_vsize` reports.

use std::collections::VecDeque;

use brk_types::{
    CpfpCluster, CpfpClusterChunk, CpfpClusterTx, CpfpClusterTxIndex, CpfpEntry, CpfpInfo, FeeRate,
    SigOps, TxidPrefix, VSize,
};
use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

use crate::{
    Mempool,
    chunking::{ChunkInput, linearize},
    steps::{SnapTx, TxIndex},
};

/// Cap matches Bitcoin Core's default mempool ancestor/descendant
/// chain limits and mempool.space's truncation.
const MAX: usize = 25;

/// Cluster cap matches Bitcoin Core 31's `MAX_CLUSTER_COUNT_LIMIT`
/// (max txs in a single cluster-mempool cluster). Sized large enough
/// to hold the whole connected component for any policy-conformant
/// cluster, then truncated.
const MAX_CLUSTER: usize = 64;

impl Mempool {
    /// CPFP info for a live mempool tx. Returns `None` only when the
    /// tx isn't in the mempool, so callers can fall through to the
    /// confirmed path.
    pub fn cpfp_info(&self, prefix: &TxidPrefix) -> Option<CpfpInfo> {
        let snapshot = self.snapshot();
        let seed_idx = snapshot.idx_of(prefix)?;
        let seed = snapshot.tx(seed_idx)?;

        let sigops = self
            .read()
            .txs
            .get(&seed.txid)
            .map(|tx| tx.total_sigop_cost)
            .unwrap_or(SigOps::ZERO);

        Some(build_cpfp_info(&snapshot.txs, seed_idx, seed, sigops))
    }
}

fn build_cpfp_info(
    txs: &[SnapTx],
    seed_idx: TxIndex,
    seed: &SnapTx,
    sigops: SigOps,
) -> CpfpInfo {
    let ancestors = collect_entries(txs, seed_idx, |t| &t.parents);
    let descendants = collect_entries(txs, seed_idx, |t| &t.children);
    let best_descendant = descendants
        .iter()
        .max_by_key(|e| FeeRate::from((e.fee, e.weight)))
        .cloned();

    let (cluster, effective_fee_per_vsize) = build_cluster(txs, seed_idx, seed);
    let vsize = VSize::from(seed.weight);

    CpfpInfo {
        ancestors,
        best_descendant,
        descendants,
        effective_fee_per_vsize,
        sigops,
        fee: seed.fee,
        vsize,
        adjusted_vsize: sigops.adjust_vsize(vsize),
        cluster,
    }
}

/// Walk the graph from `seed` along `next` and lift the visited indices
/// into wire-shape `CpfpEntry`s in one go.
fn collect_entries(
    txs: &[SnapTx],
    seed: TxIndex,
    next: impl Fn(&SnapTx) -> &[TxIndex],
) -> Vec<CpfpEntry> {
    walk(txs, seed, next)
        .iter()
        .filter_map(|&i| txs.get(i.as_usize()).map(CpfpEntry::from))
        .collect()
}

/// Capped DFS from `seed` (exclusive), following the neighbors yielded
/// by `next`. Used for both the ancestor and descendant walks.
fn walk(txs: &[SnapTx], seed: TxIndex, next: impl Fn(&SnapTx) -> &[TxIndex]) -> Vec<TxIndex> {
    let Some(seed_node) = txs.get(seed.as_usize()) else {
        return Vec::new();
    };
    let mut visited: FxHashSet<TxIndex> =
        FxHashSet::with_capacity_and_hasher(MAX + 1, FxBuildHasher);
    visited.insert(seed);
    let mut out: Vec<TxIndex> = Vec::with_capacity(MAX);
    let mut stack: Vec<TxIndex> = next(seed_node).to_vec();
    while let Some(idx) = stack.pop() {
        if out.len() >= MAX {
            break;
        }
        if !visited.insert(idx) {
            continue;
        }
        out.push(idx);
        if let Some(t) = txs.get(idx.as_usize()) {
            stack.extend(next(t).iter().copied());
        }
    }
    out
}

/// Wire-shape `CpfpCluster` plus the seed's chunk feerate. Members are
/// the connected component of the seed in the dependency graph, then
/// topologically sorted (parents before children) so wire indices and
/// chunk-internal ordering are valid for client-side reconstruction.
/// Returns `(None, seed_per_tx_rate)` for singletons (matches
/// mempool.space, which omits `cluster` when no relations exist).
fn build_cluster(
    txs: &[SnapTx],
    seed_idx: TxIndex,
    seed: &SnapTx,
) -> (Option<CpfpCluster>, FeeRate) {
    let seed_per_tx_rate = FeeRate::from((seed.fee, seed.vsize));
    let component = walk_cluster(txs, seed_idx);
    if component.len() <= 1 {
        return (None, seed_per_tx_rate);
    }

    let members = topo_sort(txs, &component);
    let local_of = build_local_index(&members);
    let (cluster_txs, vsizes) = collect_cluster_members(txs, &members, &local_of);
    let chunks = linearize_cluster(&cluster_txs, &vsizes);
    let (chunk_index, seed_chunk_rate) =
        locate_seed_chunk(local_of[&seed_idx], &chunks, seed_per_tx_rate);

    (
        Some(CpfpCluster {
            txs: cluster_txs,
            chunks,
            chunk_index,
        }),
        seed_chunk_rate,
    )
}

/// `members[i]`'s wire index, keyed by snapshot `TxIndex`. Built once
/// so per-tx parent edges can be remapped without a linear scan.
fn build_local_index(members: &[TxIndex]) -> FxHashMap<TxIndex, CpfpClusterTxIndex> {
    members
        .iter()
        .enumerate()
        .map(|(i, &idx)| (idx, CpfpClusterTxIndex::from(i as u32)))
        .collect()
}

/// Materialize wire-shape `CpfpClusterTx`s for every member with parent
/// edges remapped to local indices, plus the parallel `vsize` column the
/// linearizer needs (not carried on `CpfpClusterTx`, which only stores
/// weight).
fn collect_cluster_members(
    txs: &[SnapTx],
    members: &[TxIndex],
    local_of: &FxHashMap<TxIndex, CpfpClusterTxIndex>,
) -> (Vec<CpfpClusterTx>, Vec<VSize>) {
    let mut cluster_txs: Vec<CpfpClusterTx> = Vec::with_capacity(members.len());
    let mut vsizes: Vec<VSize> = Vec::with_capacity(members.len());
    for &idx in members {
        let Some(t) = txs.get(idx.as_usize()) else {
            continue;
        };
        let parents: Vec<CpfpClusterTxIndex> = t
            .parents
            .iter()
            .filter_map(|p| local_of.get(p).copied())
            .collect();
        cluster_txs.push(CpfpClusterTx {
            txid: t.txid,
            weight: t.weight,
            fee: t.fee,
            parents,
        });
        vsizes.push(t.vsize);
    }
    (cluster_txs, vsizes)
}

/// Single-fee-linearize the cluster, borrowing parents from the
/// already-built `cluster_txs` so no re-allocation is needed.
fn linearize_cluster(cluster_txs: &[CpfpClusterTx], vsizes: &[VSize]) -> Vec<CpfpClusterChunk> {
    let inputs: Vec<ChunkInput<'_>> = cluster_txs
        .iter()
        .zip(vsizes)
        .map(|(c, &vsize)| ChunkInput {
            fee: c.fee,
            vsize,
            parents: &c.parents,
        })
        .collect();
    linearize(&inputs)
}

/// Find the chunk containing the seed and return its index plus rate.
/// Falls back to `(0, seed_per_tx_rate)` when the seed isn't in any
/// chunk - shouldn't happen but keeps the wire shape valid.
fn locate_seed_chunk(
    seed_local: CpfpClusterTxIndex,
    chunks: &[CpfpClusterChunk],
    seed_per_tx_rate: FeeRate,
) -> (u32, FeeRate) {
    chunks
        .iter()
        .enumerate()
        .find(|(_, ch)| ch.txs.contains(&seed_local))
        .map(|(i, ch)| (i as u32, ch.feerate))
        .unwrap_or((0, seed_per_tx_rate))
}

/// Capped DFS over the undirected dependency graph (`parents ∪
/// children`) starting from `seed`. Returns the connected component
/// truncated to `MAX_CLUSTER`, with `seed` at index 0.
fn walk_cluster(txs: &[SnapTx], seed: TxIndex) -> Vec<TxIndex> {
    if txs.get(seed.as_usize()).is_none() {
        return Vec::new();
    }
    let mut visited: FxHashSet<TxIndex> =
        FxHashSet::with_capacity_and_hasher(MAX_CLUSTER, FxBuildHasher);
    visited.insert(seed);
    let mut out: Vec<TxIndex> = Vec::with_capacity(MAX_CLUSTER);
    out.push(seed);
    let mut stack: Vec<TxIndex> = vec![seed];
    while let Some(idx) = stack.pop() {
        let Some(t) = txs.get(idx.as_usize()) else {
            continue;
        };
        for &n in t.parents.iter().chain(t.children.iter()) {
            if out.len() >= MAX_CLUSTER {
                return out;
            }
            if visited.insert(n) {
                out.push(n);
                stack.push(n);
            }
        }
    }
    out
}

/// Kahn's topological sort over the connected component, restricted to
/// in-cluster parent edges. Returns members in an order where every tx
/// follows all its in-cluster parents.
fn topo_sort(txs: &[SnapTx], component: &[TxIndex]) -> Vec<TxIndex> {
    let n = component.len();
    let pos: FxHashMap<TxIndex, usize> = component
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i))
        .collect();
    let mut indeg: Vec<u32> = vec![0; n];
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, &idx) in component.iter().enumerate() {
        let Some(t) = txs.get(idx.as_usize()) else {
            continue;
        };
        indeg[i] = t.parents.iter().filter(|p| pos.contains_key(p)).count() as u32;
        for &c in t.children.iter() {
            if let Some(&ci) = pos.get(&c) {
                children[i].push(ci);
            }
        }
    }
    let mut queue: VecDeque<usize> = (0..n).filter(|&i| indeg[i] == 0).collect();
    let mut out: Vec<TxIndex> = Vec::with_capacity(n);
    while let Some(i) = queue.pop_front() {
        out.push(component[i]);
        for &c in &children[i] {
            indeg[c] -= 1;
            if indeg[c] == 0 {
                queue.push_back(c);
            }
        }
    }
    out
}
