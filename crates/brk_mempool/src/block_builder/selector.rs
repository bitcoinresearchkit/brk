use std::collections::BinaryHeap;

use brk_types::FeeRate;
use rustc_hash::FxHashSet;
use smallvec::SmallVec;

use super::BLOCK_VSIZE;
use super::graph::Graph;
use super::heap_entry::HeapEntry;
use super::package::Package;
use crate::types::PoolIndex;

/// Select transactions from the graph and group into CPFP packages.
pub fn select_packages(graph: &mut Graph, num_blocks: usize) -> Vec<Package> {
    let target_vsize = BLOCK_VSIZE * num_blocks as u64;
    let mut total_vsize: u64 = 0;
    let mut packages: Vec<Package> = Vec::new();

    // Initialize heap with all transactions
    let mut heap: BinaryHeap<HeapEntry> = (0..graph.len())
        .map(|i| HeapEntry::new(&graph[PoolIndex::from(i)]))
        .collect();

    while let Some(entry) = heap.pop() {
        let node = &graph[entry.pool_index];

        // Skip if already selected or entry is stale
        if node.selected {
            continue;
        }

        // Package fee rate at selection time
        let package_rate = FeeRate::from((node.ancestor_fee, node.ancestor_vsize));

        // Select this tx and all unselected ancestors (parents first)
        let ancestors = select_with_ancestors(graph, entry.pool_index);

        let mut package = Package::new(package_rate);
        for pool_idx in ancestors {
            let vsize = u64::from(graph[pool_idx].vsize);
            package.add_tx(graph[pool_idx].tx_index, vsize);
            update_descendants(graph, pool_idx, &mut heap);
        }

        total_vsize += package.vsize;
        packages.push(package);

        if total_vsize >= target_vsize {
            break;
        }
    }

    packages
}

/// Select a tx and all its unselected ancestors in topological order.
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

/// Update descendants' ancestor scores after selecting a tx.
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
        if child.selected {
            continue;
        }

        // Update ancestor totals
        child.ancestor_fee -= selected_fee;
        child.ancestor_vsize -= selected_vsize;

        // Increment generation and re-push to heap
        child.generation += 1;
        heap.push(HeapEntry::new(child));

        // Continue to grandchildren
        stack.extend(child.children.iter().copied());
    }
}
