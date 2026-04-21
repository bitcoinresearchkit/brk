use std::collections::BinaryHeap;

use brk_types::FeeRate;
use rustc_hash::FxHashSet;
use smallvec::SmallVec;

use super::{graph::Graph, heap_entry::HeapEntry, package::Package};
use crate::types::PoolIndex;

/// Sentinel for `package_of` entries that haven't been placed in a package yet.
const UNASSIGNED: u32 = u32::MAX;

/// Select transactions from the graph and group them into CPFP packages,
/// running until every unselected tx has been placed into a package.
pub fn select_packages(graph: &mut Graph) -> Vec<Package> {
    let mut packages: Vec<Package> = Vec::new();
    let mut package_of: Vec<u32> = vec![UNASSIGNED; graph.len()];

    let mut heap: BinaryHeap<HeapEntry> = (0..graph.len())
        .map(|i| HeapEntry::new(&graph[PoolIndex::from(i)]))
        .collect();

    while let Some(entry) = heap.pop() {
        let node = &graph[entry.pool_index];
        if node.selected || entry.generation != node.generation {
            continue;
        }

        let own_rate = FeeRate::from((node.ancestor_fee, node.ancestor_vsize));
        let package_idx = packages.len() as u32;
        let mut package = Package::new(own_rate);

        for pool_idx in select_with_ancestors(graph, entry.pool_index) {
            let tx = &graph[pool_idx];
            package.add_tx(tx.tx_index, u64::from(tx.vsize));
            package_of[pool_idx.as_usize()] = package_idx;

            // Cap placement_rate by any ancestor packages this tx depends on.
            // select_with_ancestors returns parents before children, so a
            // parent sitting in this same package already has package_of
            // set to package_idx; only parents in earlier packages matter.
            for &parent in &tx.parents {
                let parent_pkg = package_of[parent.as_usize()];
                if parent_pkg != package_idx && parent_pkg != UNASSIGNED {
                    package.placement_rate = package
                        .placement_rate
                        .min(packages[parent_pkg as usize].placement_rate);
                }
            }

            update_descendants(graph, pool_idx, &mut heap);
        }

        packages.push(package);
    }

    packages
}

/// Return `pool_idx` and all its unselected ancestors in topological order
/// (parents before children), marking each one selected as we go.
fn select_with_ancestors(graph: &mut Graph, pool_idx: PoolIndex) -> SmallVec<[PoolIndex; 8]> {
    let mut result: SmallVec<[PoolIndex; 8]> = SmallVec::new();
    let mut stack: SmallVec<[(PoolIndex, bool); 16]> = smallvec::smallvec![(pool_idx, false)];

    while let Some((idx, parents_done)) = stack.pop() {
        if graph[idx].selected {
            continue;
        }

        if parents_done {
            graph[idx].selected = true;
            result.push(idx);
        } else {
            stack.push((idx, true));
            for &parent in &graph[idx].parents {
                if !graph[parent].selected {
                    stack.push((parent, false));
                }
            }
        }
    }

    result
}

/// Subtract the selected tx's fee and vsize from every unselected
/// descendant's ancestor totals, and re-push updated entries to the heap.
fn update_descendants(
    graph: &mut Graph,
    selected_idx: PoolIndex,
    heap: &mut BinaryHeap<HeapEntry>,
) {
    let selected_fee = graph[selected_idx].fee;
    let selected_vsize = graph[selected_idx].vsize;

    // Track visited to avoid double-updates in diamond patterns
    let mut visited: FxHashSet<PoolIndex> = FxHashSet::default();
    let mut stack: SmallVec<[PoolIndex; 16]> =
        graph[selected_idx].children.iter().copied().collect();

    while let Some(child_idx) = stack.pop() {
        if !visited.insert(child_idx) {
            continue;
        }

        let child = &mut graph[child_idx];

        // Walk through selected intermediates: descendants behind them still
        // need their ancestor totals reduced, otherwise CPFP chains with
        // already-selected parents keep inflated scores and get split.
        if !child.selected {
            child.ancestor_fee -= selected_fee;
            child.ancestor_vsize -= selected_vsize;
            child.generation += 1;
            heap.push(HeapEntry::new(child));
        }

        stack.extend(child.children.iter().copied());
    }
}
