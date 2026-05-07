//! CPFP (Child Pays For Parent) walk over a `Snapshot`'s adjacency.
//!
//! The snapshot stores per-tx parent/child edges in `TxIndex` space and
//! a per-tx `chunk_rate` (Core's `fees.chunk` / `chunkweight` truth, or
//! the proxy fallback). The walk is a pair of capped DFSes, then the
//! cluster wire shape is materialized from the visited set.

use std::cmp::Reverse;

use brk_types::{
    CpfpCluster, CpfpClusterChunk, CpfpClusterTx, CpfpClusterTxIndex, CpfpEntry, CpfpInfo, FeeRate,
    SigOps, TxidPrefix, VSize,
};
use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

use crate::Mempool;
use crate::steps::{SnapTx, TxIndex};

/// Cap matches Bitcoin Core's default mempool ancestor/descendant
/// chain limits and mempool.space's truncation.
const MAX: usize = 25;

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
    let ancestors_idx = walk(txs, seed_idx, |t| &t.parents);
    let descendants_idx = walk(txs, seed_idx, |t| &t.children);

    let ancestors: Vec<CpfpEntry> = ancestors_idx
        .iter()
        .filter_map(|&i| txs.get(i.as_usize()).map(CpfpEntry::from))
        .collect();
    let descendants: Vec<CpfpEntry> = descendants_idx
        .iter()
        .filter_map(|&i| txs.get(i.as_usize()).map(CpfpEntry::from))
        .collect();
    let best_descendant = descendants
        .iter()
        .max_by_key(|e| FeeRate::from((e.fee, e.weight)))
        .cloned();

    let cluster = build_cluster(txs, seed_idx, &ancestors_idx, &descendants_idx);
    let vsize = VSize::from(seed.weight);

    CpfpInfo {
        ancestors,
        best_descendant,
        descendants,
        effective_fee_per_vsize: seed.chunk_rate,
        sigops,
        fee: seed.fee,
        vsize,
        adjusted_vsize: sigops.adjust_vsize(vsize),
        cluster,
    }
}

/// Capped DFS from `seed` (exclusive), following the neighbors yielded
/// by `next`. Used for both the ancestor and descendant walks.
fn walk(txs: &[SnapTx], seed: TxIndex, next: impl Fn(&SnapTx) -> &[TxIndex]) -> Vec<TxIndex> {
    let mut visited: FxHashSet<TxIndex> =
        FxHashSet::with_capacity_and_hasher(MAX + 1, FxBuildHasher);
    visited.insert(seed);
    let mut out: Vec<TxIndex> = Vec::with_capacity(MAX);
    let mut stack: Vec<TxIndex> = txs
        .get(seed.as_usize())
        .map(|t| next(t).to_vec())
        .unwrap_or_default();
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

/// Wire-shape `CpfpCluster`. Members are emitted in `[ancestors..., seed,
/// descendants...]` order so the seed's index inside the cluster is
/// `ancestors.len()`. Chunks group txs by exact `chunk_rate` value: under
/// Core 31 this matches Core's actual chunks; under proxy fallback it
/// produces a fine-grained but consistent breakdown.
fn build_cluster(
    txs: &[SnapTx],
    seed_idx: TxIndex,
    ancestors: &[TxIndex],
    descendants: &[TxIndex],
) -> CpfpCluster {
    let members: Vec<TxIndex> = ancestors
        .iter()
        .copied()
        .chain(std::iter::once(seed_idx))
        .chain(descendants.iter().copied())
        .collect();

    let local_of: FxHashMap<TxIndex, CpfpClusterTxIndex> = members
        .iter()
        .enumerate()
        .map(|(i, &idx)| (idx, CpfpClusterTxIndex::from(i as u32)))
        .collect();

    let cluster_txs: Vec<CpfpClusterTx> = members
        .iter()
        .filter_map(|&idx| {
            let t = txs.get(idx.as_usize())?;
            Some(CpfpClusterTx {
                txid: t.txid,
                weight: t.weight,
                fee: t.fee,
                parents: t
                    .parents
                    .iter()
                    .filter_map(|p| local_of.get(p).copied())
                    .collect(),
            })
        })
        .collect();

    let chunks = chunk_groups(&members, txs, &local_of);
    let seed_local = local_of[&seed_idx];
    let chunk_index = chunks
        .iter()
        .position(|ch| ch.txs.contains(&seed_local))
        .unwrap_or(0) as u32;

    CpfpCluster {
        txs: cluster_txs,
        chunks,
        chunk_index,
    }
}

/// Group cluster members into chunks by descending `chunk_rate`. Cluster
/// size is bounded by `2 * MAX + 1` so a sort-then-fold is cheaper and
/// simpler than a hashmap keyed on `f64` bits.
fn chunk_groups(
    members: &[TxIndex],
    txs: &[SnapTx],
    local_of: &FxHashMap<TxIndex, CpfpClusterTxIndex>,
) -> Vec<CpfpClusterChunk> {
    let mut entries: Vec<(FeeRate, CpfpClusterTxIndex)> = members
        .iter()
        .filter_map(|&idx| Some((txs.get(idx.as_usize())?.chunk_rate, local_of[&idx])))
        .collect();
    entries.sort_by_key(|e| Reverse(e.0));

    let mut chunks: Vec<CpfpClusterChunk> = Vec::new();
    for (rate, local) in entries {
        match chunks.last_mut() {
            Some(last) if last.feerate == rate => last.txs.push(local),
            _ => chunks.push(CpfpClusterChunk {
                txs: vec![local],
                feerate: rate,
            }),
        }
    }
    chunks
}
