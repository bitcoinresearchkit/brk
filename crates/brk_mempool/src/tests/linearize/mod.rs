mod basic;
mod oracle;
mod stress;

use brk_types::{Sats, Txid, VSize, Weight};
use smallvec::SmallVec;

use crate::cluster::{Chunk, Cluster, ClusterNode, LocalIdx};

/// Test cluster: each node carries its input position as `id`, so
/// invariant checks can map `LocalIdx` (post-permutation) back to the
/// caller's `fees_vsizes` / `edges` index space.
pub(super) type TestCluster = Cluster<u32>;

pub(super) fn make_cluster(fees_vsizes: &[(u64, u64)], edges: &[(u32, u32)]) -> TestCluster {
    let mut parents: Vec<SmallVec<[LocalIdx; 2]>> =
        (0..fees_vsizes.len()).map(|_| SmallVec::new()).collect();
    for &(p, c) in edges {
        parents[c as usize].push(LocalIdx::from(p));
    }

    let nodes: Vec<ClusterNode<u32>> = fees_vsizes
        .iter()
        .zip(parents)
        .enumerate()
        .map(|(i, (&(fee, vsize), parents))| ClusterNode {
            id: i as u32,
            txid: Txid::COINBASE,
            fee: Sats::from(fee),
            vsize: VSize::from(vsize),
            weight: Weight::from(vsize * 4),
            parents,
        })
        .collect();

    Cluster::new(nodes)
}

pub(super) fn run(cluster: &TestCluster) -> &[Chunk] {
    &cluster.chunks
}

pub(super) fn chunk_shapes(chunks: &[Chunk]) -> Vec<(usize, Sats, VSize)> {
    chunks
        .iter()
        .map(|c| (c.txs.len(), c.fee, c.vsize))
        .collect()
}
