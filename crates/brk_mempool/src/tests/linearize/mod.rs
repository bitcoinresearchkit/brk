mod basic;
mod oracle;
mod stress;

use brk_types::{Sats, VSize};
use smallvec::SmallVec;

use crate::{
    steps::rebuilder::linearize::{
        LocalIdx, chunk::Chunk, cluster::Cluster, cluster_node::ClusterNode, sfl::Sfl,
    },
    stores::TxIndex,
};

pub(super) fn make_cluster(fees_vsizes: &[(u64, u64)], edges: &[(LocalIdx, LocalIdx)]) -> Cluster {
    let mut nodes: Vec<ClusterNode> = fees_vsizes
        .iter()
        .enumerate()
        .map(|(i, &(fee, vsize))| ClusterNode {
            tx_index: TxIndex::from(i),
            fee: Sats::from(fee),
            vsize: VSize::from(vsize),
            parents: SmallVec::new(),
            children: SmallVec::new(),
        })
        .collect();

    for &(p, c) in edges {
        nodes[c as usize].parents.push(p);
        nodes[p as usize].children.push(c);
    }

    Cluster::new(nodes)
}

pub(super) fn run(cluster: &Cluster) -> Vec<Chunk> {
    Sfl::linearize(cluster)
}

pub(super) fn chunk_shapes(chunks: &[Chunk]) -> Vec<(usize, Sats, VSize)> {
    chunks
        .iter()
        .map(|c| (c.nodes.len(), c.fee, c.vsize))
        .collect()
}
