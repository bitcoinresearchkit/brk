mod graph;
mod heap_entry;
mod package;
mod partitioner;
mod selector;
mod tx_node;

use crate::{entry::Entry, types::SelectedTx};

/// Target vsize per block (~1MB, derived from 4MW weight limit).
const BLOCK_VSIZE: u64 = 1_000_000;

/// Number of projected blocks to build.
const NUM_BLOCKS: usize = 8;

/// Build projected blocks from mempool entries.
///
/// Returns transactions grouped by projected block, sorted by fee rate.
pub fn build_projected_blocks(entries: &[Option<Entry>]) -> Vec<Vec<SelectedTx>> {
    // Build dependency graph
    let mut graph = graph::build_graph(entries);

    if graph.is_empty() {
        return Vec::new();
    }

    // Select transactions into packages
    let packages = selector::select_packages(&mut graph, NUM_BLOCKS);

    // Partition packages into blocks
    partitioner::partition_into_blocks(packages, NUM_BLOCKS)
}
