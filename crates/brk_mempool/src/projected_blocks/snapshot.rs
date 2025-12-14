use brk_types::RecommendedFees;

use super::fees;
use super::stats::{self, BlockStats};
use crate::entry::Entry;
use crate::types::{SelectedTx, TxIndex};

/// Immutable snapshot of projected blocks.
#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    /// Block structure: indices into entries Vec
    pub blocks: Vec<Vec<TxIndex>>,
    /// Pre-computed stats per block
    pub block_stats: Vec<BlockStats>,
    /// Pre-computed fee recommendations
    pub fees: RecommendedFees,
}

impl Snapshot {
    /// Build snapshot from selected transactions and entries.
    pub fn build(blocks: Vec<Vec<SelectedTx>>, entries: &[Option<Entry>]) -> Self {
        let block_stats: Vec<BlockStats> = blocks
            .iter()
            .map(|selected| stats::compute_block_stats(selected, entries))
            .collect();

        let fees = fees::compute_recommended_fees(&block_stats);

        // Extract just the indices from selected transactions
        let blocks = blocks
            .into_iter()
            .map(|selected| selected.into_iter().map(|s| s.tx_index).collect())
            .collect();

        Self {
            blocks,
            block_stats,
            fees,
        }
    }
}
