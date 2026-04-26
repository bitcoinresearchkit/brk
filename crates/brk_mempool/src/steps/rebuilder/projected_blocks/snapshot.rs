use std::hash::{DefaultHasher, Hash, Hasher};

use brk_types::RecommendedFees;

use super::{
    super::block_builder::Package,
    fees,
    stats::{self, BlockStats},
};
use crate::stores::{Entry, TxIndex};

/// Immutable snapshot of projected blocks.
#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    /// Block structure: indices into the mempool entries Vec, in the
    /// order they'd appear in the block.
    pub blocks: Vec<Vec<TxIndex>>,
    pub block_stats: Vec<BlockStats>,
    pub fees: RecommendedFees,
    /// ETag-like cache key for the first projected block. A hash of
    /// the block's tx ordering, not a Bitcoin block header hash (no
    /// header exists yet - it's a projection). Precomputed at build
    /// time since the snapshot is immutable; `0` iff there are no
    /// projected blocks.
    pub next_block_hash: u64,
}

impl Snapshot {
    /// Build a snapshot from packages grouped by projected block.
    pub fn build(blocks: Vec<Vec<Package>>, entries: &[Option<Entry>]) -> Self {
        let block_stats: Vec<BlockStats> = blocks
            .iter()
            .map(|block| stats::compute_block_stats(block, entries))
            .collect();

        let fees = fees::compute_recommended_fees(&block_stats);

        let blocks: Vec<Vec<TxIndex>> = blocks
            .into_iter()
            .map(|block| block.into_iter().flat_map(|pkg| pkg.txs).collect())
            .collect();

        let next_block_hash = Self::hash_next_block(&blocks);

        Self {
            blocks,
            block_stats,
            fees,
            next_block_hash,
        }
    }

    fn hash_next_block(blocks: &[Vec<TxIndex>]) -> u64 {
        let Some(block) = blocks.first() else {
            return 0;
        };
        let mut hasher = DefaultHasher::new();
        block.hash(&mut hasher);
        hasher.finish()
    }
}
