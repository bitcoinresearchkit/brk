//! Cluster-mempool linearization.
//!
//! Partitions the mempool dependency graph into connected components
//! ("clusters"), linearizes each into chunks ordered by descending
//! feerate, and emits the resulting chunks as `Package`s. The inner
//! algorithm (see `sfl.rs`) is a topologically-closed-subset search,
//! optimal for clusters up to 18 txs and near-optimal beyond that.

pub(crate) mod chunk;
pub(crate) mod cluster;
pub(crate) mod cluster_node;
pub(crate) mod package;
pub(crate) mod sfl;

pub use package::Package;

use rustc_hash::{FxBuildHasher, FxHashMap};
use smallvec::SmallVec;

use cluster::Cluster;
use cluster_node::ClusterNode;
use sfl::Sfl;

use super::graph::{PoolIndex, TxNode};

pub(crate) type LocalIdx = u32;

pub struct Linearizer;

impl Linearizer {
    /// Order across clusters is unspecified: the partitioner re-sorts by
    /// fee rate downstream.
    pub fn linearize(nodes: &[TxNode]) -> Vec<Package> {
        let clusters = Self::find_components(nodes);
        Self::pack_clusters(clusters)
    }

    fn pack_clusters(clusters: Vec<Cluster>) -> Vec<Package> {
        clusters
            .iter()
            .enumerate()
            .flat_map(|(cluster_id, cluster)| Self::pack_cluster(cluster, cluster_id as u32))
            .collect()
    }

    fn pack_cluster(cluster: &Cluster, cluster_id: u32) -> Vec<Package> {
        if cluster.nodes.len() == 1 {
            return vec![Package::singleton(cluster, cluster_id)];
        }
        Sfl::linearize(cluster)
            .into_iter()
            .enumerate()
            .map(|(chunk_order, chunk)| {
                Package::from_chunk(cluster, chunk, cluster_id, chunk_order as u32)
            })
            .collect()
    }

    fn find_components(nodes: &[TxNode]) -> Vec<Cluster> {
        let n = nodes.len();
        let mut seen: Vec<bool> = vec![false; n];
        let mut clusters: Vec<Cluster> = Vec::new();
        let mut stack: Vec<PoolIndex> = Vec::new();

        for start in 0..n {
            if seen[start] {
                continue;
            }
            let mut members = Self::flood_component(start, nodes, &mut seen, &mut stack);
            // Deterministic LocalIdx assignment keeps SFL output stable
            // across sync ticks.
            members.sort_unstable();
            clusters.push(Self::build_cluster(nodes, &members));
        }

        clusters
    }

    fn flood_component(
        start: usize,
        nodes: &[TxNode],
        seen: &mut [bool],
        stack: &mut Vec<PoolIndex>,
    ) -> Vec<PoolIndex> {
        let mut members: Vec<PoolIndex> = Vec::new();
        stack.clear();
        stack.push(PoolIndex::from(start));
        seen[start] = true;

        while let Some(idx) = stack.pop() {
            members.push(idx);
            let node = &nodes[idx.as_usize()];
            for &n in node.parents.iter().chain(node.children.iter()) {
                if !seen[n.as_usize()] {
                    seen[n.as_usize()] = true;
                    stack.push(n);
                }
            }
        }
        members
    }

    fn build_cluster(nodes: &[TxNode], members: &[PoolIndex]) -> Cluster {
        let mut pool_to_local: FxHashMap<PoolIndex, LocalIdx> =
            FxHashMap::with_capacity_and_hasher(members.len(), FxBuildHasher);
        for (i, &p) in members.iter().enumerate() {
            pool_to_local.insert(p, i as LocalIdx);
        }

        let cluster_nodes: Vec<ClusterNode> = members
            .iter()
            .map(|&pool_idx| {
                let node = &nodes[pool_idx.as_usize()];
                ClusterNode {
                    tx_index: node.tx_index,
                    fee: node.fee,
                    vsize: node.vsize,
                    parents: Self::local_neighbors(&node.parents, &pool_to_local),
                    children: Self::local_neighbors(&node.children, &pool_to_local),
                }
            })
            .collect();

        Cluster::new(cluster_nodes)
    }

    fn local_neighbors(
        pool_neighbors: &[PoolIndex],
        pool_to_local: &FxHashMap<PoolIndex, LocalIdx>,
    ) -> SmallVec<[LocalIdx; 2]> {
        pool_neighbors
            .iter()
            .filter_map(|p| pool_to_local.get(p).copied())
            .collect()
    }
}
