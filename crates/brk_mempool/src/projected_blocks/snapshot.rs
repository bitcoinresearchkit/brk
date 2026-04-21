use std::hash::{DefaultHasher, Hash, Hasher};

use brk_types::RecommendedFees;

use super::{
    fees,
    stats::{self, BlockStats},
};
use crate::{block_builder::Package, entry::Entry, types::TxIndex};

/// Immutable snapshot of projected blocks.
#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    /// Block structure: indices into the mempool entries Vec, in the
    /// order they'd appear in the block.
    pub blocks: Vec<Vec<TxIndex>>,
    pub block_stats: Vec<BlockStats>,
    pub fees: RecommendedFees,
}

impl Snapshot {
    /// Build a snapshot from packages grouped by projected block.
    pub fn build(blocks: Vec<Vec<Package>>, entries: &[Option<Entry>]) -> Self {
        let block_stats: Vec<BlockStats> = blocks
            .iter()
            .map(|block| stats::compute_block_stats(block, entries))
            .collect();

        let fees = fees::compute_recommended_fees(&block_stats);

        let blocks = blocks
            .into_iter()
            .map(|block| block.into_iter().flat_map(|pkg| pkg.txs).collect())
            .collect();

        Self {
            blocks,
            block_stats,
            fees,
        }
    }

    /// Hash of the first projected block (the one about to be mined).
    pub fn next_block_hash(&self) -> u64 {
        let Some(block) = self.blocks.first() else {
            return 0;
        };
        let mut hasher = DefaultHasher::new();
        block.hash(&mut hasher);
        hasher.finish()
    }
}
