//! Cluster-mempool linearization.
//!
//! Partitions the mempool dependency graph into connected components
//! ("clusters"), linearizes each into chunks ordered by descending
//! feerate, and emits the resulting chunks as `Package`s. The inner
//! algorithm (see `sfl.rs`) is a topologically-closed-subset search,
//! optimal for clusters up to 18 txs and near-optimal beyond that.

mod sfl;

#[cfg(test)]
mod tests;

use brk_types::{FeeRate, Sats, VSize};
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use super::{graph::Graph, package::Package, pool_index::PoolIndex};
use crate::stores::TxIndex;

/// Cluster-local index for a node within one cluster's flat array.
type LocalIdx = u32;

/// A connected component of the mempool graph, re-indexed locally.
struct Cluster {
    /// Nodes indexed by `LocalIdx`.
    nodes: Vec<ClusterNode>,
    /// `topo_rank[i] = position of node i in a Kahn topological order`.
    /// Used during chunk emission to print txs parents-first.
    topo_rank: Vec<u32>,
}

struct ClusterNode {
    tx_index: TxIndex,
    fee: Sats,
    vsize: VSize,
    parents: SmallVec<[LocalIdx; 2]>,
    children: SmallVec<[LocalIdx; 2]>,
}

/// Partition `graph` into clusters, linearize each, and flatten the
/// resulting chunks into a `Vec<Package>`. Order across clusters is
/// unspecified; the partitioner re-sorts by fee rate downstream.
pub fn linearize_clusters(graph: &Graph) -> Vec<Package> {
    let clusters = find_components(graph);
    let mut packages: Vec<Package> = Vec::with_capacity(clusters.len());

    for (cluster_id, cluster) in clusters.into_iter().enumerate() {
        let cluster_id = cluster_id as u32;
        if cluster.nodes.len() == 1 {
            packages.push(singleton_package(&cluster, cluster_id));
            continue;
        }
        for (chunk_order, chunk) in sfl::linearize(&cluster).iter().enumerate() {
            packages.push(chunk_to_package(&cluster, chunk, cluster_id, chunk_order as u32));
        }
    }

    packages
}

/// DFS over (parents + children) adjacency to partition `graph` into
/// connected components, each re-indexed locally.
fn find_components(graph: &Graph) -> Vec<Cluster> {
    let n = graph.len();
    let mut seen: Vec<bool> = vec![false; n];
    let mut clusters: Vec<Cluster> = Vec::new();
    let mut stack: Vec<PoolIndex> = Vec::new();

    for start in 0..n {
        if seen[start] {
            continue;
        }

        let mut members: Vec<PoolIndex> = Vec::new();
        stack.clear();
        stack.push(PoolIndex::from(start));
        seen[start] = true;

        while let Some(idx) = stack.pop() {
            members.push(idx);
            let node = &graph[idx];
            for &p in &node.parents {
                if !seen[p.as_usize()] {
                    seen[p.as_usize()] = true;
                    stack.push(p);
                }
            }
            for &c in &node.children {
                if !seen[c.as_usize()] {
                    seen[c.as_usize()] = true;
                    stack.push(c);
                }
            }
        }

        // Sort by PoolIndex for deterministic LocalIdx assignment (keeps
        // SFL output stable across sync ticks).
        members.sort_unstable();
        clusters.push(build_cluster(graph, members));
    }

    clusters
}

/// Build a re-indexed `Cluster` from a set of graph members.
fn build_cluster(graph: &Graph, members: Vec<PoolIndex>) -> Cluster {
    let pool_to_local: FxHashMap<PoolIndex, LocalIdx> = members
        .iter()
        .enumerate()
        .map(|(i, &p)| (p, i as LocalIdx))
        .collect();

    let mut nodes: Vec<ClusterNode> = Vec::with_capacity(members.len());
    for &pool_idx in &members {
        let node = &graph[pool_idx];
        let mut parents: SmallVec<[LocalIdx; 2]> = SmallVec::new();
        for &p in &node.parents {
            if let Some(&local) = pool_to_local.get(&p) {
                parents.push(local);
            }
        }
        let mut children: SmallVec<[LocalIdx; 2]> = SmallVec::new();
        for &c in &node.children {
            if let Some(&local) = pool_to_local.get(&c) {
                children.push(local);
            }
        }
        nodes.push(ClusterNode {
            tx_index: node.tx_index,
            fee: node.fee,
            vsize: node.vsize,
            parents,
            children,
        });
    }

    let topo_rank = kahn_topo_rank(&nodes);
    Cluster { nodes, topo_rank }
}

/// Kahn's algorithm: returns `rank[i] = position in a topological order`.
fn kahn_topo_rank(nodes: &[ClusterNode]) -> Vec<u32> {
    let n = nodes.len();
    let mut indegree: Vec<u32> = nodes.iter().map(|n| n.parents.len() as u32).collect();
    let mut ready: Vec<LocalIdx> = (0..n as LocalIdx)
        .filter(|&i| indegree[i as usize] == 0)
        .collect();

    let mut rank: Vec<u32> = vec![0; n];
    let mut position: u32 = 0;
    let mut head = 0;

    while head < ready.len() {
        let v = ready[head];
        head += 1;
        rank[v as usize] = position;
        position += 1;
        for &c in &nodes[v as usize].children {
            indegree[c as usize] -= 1;
            if indegree[c as usize] == 0 {
                ready.push(c);
            }
        }
    }

    debug_assert_eq!(position as usize, n, "cluster contained a cycle");
    rank
}

/// Build a one-tx `Package` for a cluster of size 1.
fn singleton_package(cluster: &Cluster, cluster_id: u32) -> Package {
    let node = &cluster.nodes[0];
    let fee_rate = FeeRate::from((node.fee, node.vsize));
    let mut package = Package::new(fee_rate, cluster_id, 0);
    package.add_tx(node.tx_index, u64::from(node.vsize));
    package
}

/// Convert an SFL-emitted chunk (set of local indices) into a `Package`.
/// Txs inside the package are ordered parents-first by `topo_rank`.
fn chunk_to_package(
    cluster: &Cluster,
    chunk: &sfl::Chunk,
    cluster_id: u32,
    chunk_order: u32,
) -> Package {
    let fee_rate = FeeRate::from((Sats::from(chunk.fee), VSize::from(chunk.vsize)));
    let mut package = Package::new(fee_rate, cluster_id, chunk_order);

    let mut ordered: SmallVec<[LocalIdx; 8]> = chunk.nodes.iter().copied().collect();
    ordered.sort_by_key(|&local| cluster.topo_rank[local as usize]);

    for local in ordered {
        let node = &cluster.nodes[local as usize];
        package.add_tx(node.tx_index, u64::from(node.vsize));
    }

    package
}
