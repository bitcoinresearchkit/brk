mod graph;
mod linearize;
mod package;
mod partitioner;
mod tx_node;

pub use package::Package;

use crate::entry::Entry;

/// Target vsize per block (~1MB, derived from 4MW weight limit).
pub(crate) const BLOCK_VSIZE: u64 = 1_000_000;

/// Number of projected blocks to build (last one is a catch-all overflow).
const NUM_BLOCKS: usize = 8;

/// Build projected blocks from mempool entries.
///
/// Returns packages grouped by projected block. Blocks 1 through
/// `NUM_BLOCKS - 1` are standard ~1MB blocks sorted by placement rate
/// descending; the final block is a catch-all containing every remaining
/// package (matches mempool.space behavior).
pub fn build_projected_blocks(entries: &[Option<Entry>]) -> Vec<Vec<Package>> {
    let graph = graph::build_graph(entries);

    if graph.is_empty() {
        return Vec::new();
    }

    let packages = linearize::linearize_clusters(&graph);
    partitioner::partition_into_blocks(packages, NUM_BLOCKS)
}
