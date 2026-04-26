//! Tests for the SFL linearizer.
//!
//! Mirrors Bitcoin Core's `src/test/cluster_linearize_tests.cpp` split:
//! - `basic`  — hand-built cluster shapes, deterministic assertions.
//! - `oracle` — brute-force optimality checks for small clusters.
//! - `stress` — randomized invariant checks for larger clusters.

mod basic;
mod oracle;
mod stress;

use smallvec::SmallVec;

use super::sfl::Chunk;
use super::{Cluster, ClusterNode, LocalIdx, kahn_topo_rank, sfl};
use crate::stores::TxIndex;

/// Build a `Cluster` from `(fee, vsize)` tuples plus a list of
/// `(parent_local, child_local)` edges. Tx indices are assigned 0..n.
/// Panics if the graph has a cycle or a bad edge.
pub(super) fn make_cluster(fees_vsizes: &[(u64, u64)], edges: &[(LocalIdx, LocalIdx)]) -> Cluster {
    let mut nodes: Vec<ClusterNode> = fees_vsizes
        .iter()
        .enumerate()
        .map(|(i, &(fee, vsize))| ClusterNode {
            tx_index: TxIndex::from(i),
            fee: brk_types::Sats::from(fee),
            vsize: brk_types::VSize::from(vsize),
            parents: SmallVec::new(),
            children: SmallVec::new(),
        })
        .collect();

    for &(p, c) in edges {
        nodes[c as usize].parents.push(p);
        nodes[p as usize].children.push(c);
    }

    let topo_rank = kahn_topo_rank(&nodes);
    Cluster { nodes, topo_rank }
}

pub(super) fn run(cluster: &Cluster) -> Vec<Chunk> {
    sfl::linearize(cluster)
}

/// Shortcut: return `(chunk_size, fee, vsize)` tuples in emitted order.
pub(super) fn chunk_shapes(chunks: &[Chunk]) -> Vec<(usize, u64, u64)> {
    chunks
        .iter()
        .map(|c| (c.nodes.len(), c.fee, c.vsize))
        .collect()
}
