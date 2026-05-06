//! Build the cluster forest for a snapshot directly from the live
//! `EntryPool`. One traversal indexes live entries, builds parent
//! edges, floods the connected components, and constructs each
//! `Cluster<TxIndex>` (which mirrors child edges and runs SFL
//! internally).
//!
//! Returns the cluster forest plus a `tx_index → ClusterRef` reverse
//! map for O(1) lookup back from `EntryPool` slot to cluster position.

use brk_types::TxidPrefix;
use rustc_hash::{FxBuildHasher, FxHashMap};
use smallvec::SmallVec;

use crate::TxEntry;
use crate::cluster::{Cluster, ClusterId, ClusterNode, ClusterRef, LocalIdx};
use crate::stores::TxIndex;

/// Per-live-entry indexing position in the parents/children adjacency
/// arrays below. Local to this module; not exposed.
type Pos = u32;

pub fn build_clusters(
    entries: &[Option<TxEntry>],
) -> (Vec<Cluster<TxIndex>>, Vec<Option<ClusterRef>>) {
    let live = index_live(entries);
    if live.is_empty() {
        return (Vec::new(), vec![None; entries.len()]);
    }

    let parents = build_parent_edges(&live);
    let children = mirror_children(&parents);

    let mut seen = vec![false; live.len()];
    let mut clusters: Vec<Cluster<TxIndex>> = Vec::new();
    let mut cluster_of: Vec<Option<ClusterRef>> = vec![None; entries.len()];
    let mut stack: Vec<Pos> = Vec::new();
    // Reused across components: `local_of[pos]` is `Some(local)` while
    // we're building the current cluster, `None` otherwise. Cleared by
    // walking each cluster's members at the end of its iteration.
    let mut local_of: Vec<Option<LocalIdx>> = vec![None; live.len()];

    for start in 0..live.len() {
        if seen[start] {
            continue;
        }
        let members = flood_component(start as Pos, &parents, &children, &mut seen, &mut stack);
        for (i, &pos) in members.iter().enumerate() {
            local_of[pos as usize] = Some(LocalIdx::from(i));
        }

        let cluster_id = ClusterId::from(clusters.len());
        let cluster = build_cluster(&live, &parents, &members, &local_of);
        for (local_pos, node) in cluster.nodes.iter().enumerate() {
            cluster_of[node.id.as_usize()] = Some(ClusterRef {
                cluster_id,
                local: LocalIdx::from(local_pos),
            });
        }
        clusters.push(cluster);

        for &pos in &members {
            local_of[pos as usize] = None;
        }
    }

    (clusters, cluster_of)
}

fn flood_component(
    start: Pos,
    parents: &[SmallVec<[Pos; 4]>],
    children: &[SmallVec<[Pos; 8]>],
    seen: &mut [bool],
    stack: &mut Vec<Pos>,
) -> Vec<Pos> {
    let mut members: Vec<Pos> = Vec::new();
    stack.clear();
    stack.push(start);
    seen[start as usize] = true;

    while let Some(pos) = stack.pop() {
        members.push(pos);
        for &n in parents[pos as usize]
            .iter()
            .chain(children[pos as usize].iter())
        {
            if !seen[n as usize] {
                seen[n as usize] = true;
                stack.push(n);
            }
        }
    }
    members
}

/// `local_of` is set only for `Pos`es in this cluster, so each parent's
/// `LocalIdx` is one direct lookup (cross-cluster parents return `None`
/// and get filtered).
fn build_cluster(
    live: &[(TxIndex, &TxEntry)],
    parents: &[SmallVec<[Pos; 4]>],
    members: &[Pos],
    local_of: &[Option<LocalIdx>],
) -> Cluster<TxIndex> {
    let cluster_nodes: Vec<ClusterNode<TxIndex>> = members
        .iter()
        .map(|&pos| {
            let (tx_index, entry) = live[pos as usize];
            ClusterNode {
                id: tx_index,
                txid: entry.txid,
                fee: entry.fee,
                vsize: entry.vsize,
                weight: entry.weight,
                parents: parents[pos as usize]
                    .iter()
                    .filter_map(|&p| local_of[p as usize])
                    .collect(),
            }
        })
        .collect();

    Cluster::new(cluster_nodes)
}

fn index_live(entries: &[Option<TxEntry>]) -> Vec<(TxIndex, &TxEntry)> {
    entries
        .iter()
        .enumerate()
        .filter_map(|(i, opt)| opt.as_ref().map(|e| (TxIndex::from(i), e)))
        .collect()
}

fn build_parent_edges(live: &[(TxIndex, &TxEntry)]) -> Vec<SmallVec<[Pos; 4]>> {
    let mut prefix_to_pos: FxHashMap<TxidPrefix, Pos> =
        FxHashMap::with_capacity_and_hasher(live.len(), FxBuildHasher);
    for (i, (_, entry)) in live.iter().enumerate() {
        prefix_to_pos.insert(entry.txid_prefix(), i as Pos);
    }
    live.iter()
        .map(|(_, entry)| {
            entry
                .depends
                .iter()
                .filter_map(|p| prefix_to_pos.get(p).copied())
                .collect()
        })
        .collect()
}

fn mirror_children(parents: &[SmallVec<[Pos; 4]>]) -> Vec<SmallVec<[Pos; 8]>> {
    let mut children: Vec<SmallVec<[Pos; 8]>> =
        (0..parents.len()).map(|_| SmallVec::new()).collect();
    for (child_pos, ps) in parents.iter().enumerate() {
        for &p in ps {
            children[p as usize].push(child_pos as Pos);
        }
    }
    children
}
