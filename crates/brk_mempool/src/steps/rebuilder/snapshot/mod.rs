mod fees;
mod stats;

pub use stats::BlockStats;

use std::hash::{DefaultHasher, Hash, Hasher};

use brk_types::{FeeRate, RecommendedFees};

use super::linearize::Package;
use crate::{TxEntry, stores::TxIndex};

use fees::Fees;

#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    pub blocks: Vec<Vec<TxIndex>>,
    pub block_stats: Vec<BlockStats>,
    pub fees: RecommendedFees,
    /// ETag-like cache key for the first projected block. A hash of
    /// the tx ordering, not a Bitcoin block header hash (no header
    /// exists yet, it's a projection). `0` iff no projected blocks.
    pub next_block_hash: u64,
}

impl Snapshot {
    /// `min_fee` is bitcoind's live `mempoolminfee`, used as the floor
    /// for every recommended-fee tier.
    pub fn build(blocks: Vec<Vec<Package>>, entries: &[Option<TxEntry>], min_fee: FeeRate) -> Self {
        let block_stats = Self::compute_block_stats(&blocks, entries);
        let fees = Fees::compute(&block_stats, min_fee);
        let blocks = Self::flatten_blocks(blocks);
        let next_block_hash = Self::hash_next_block(&blocks);

        Self {
            blocks,
            block_stats,
            fees,
            next_block_hash,
        }
    }

    fn compute_block_stats(
        blocks: &[Vec<Package>],
        entries: &[Option<TxEntry>],
    ) -> Vec<BlockStats> {
        blocks
            .iter()
            .map(|block| BlockStats::compute(block, entries))
            .collect()
    }

    /// Drop the package grouping, keep only the linearized tx order.
    /// Packages were a vehicle for chunk-level fee accounting; once
    /// `compute_block_stats` is done, they're noise to API consumers.
    fn flatten_blocks(blocks: Vec<Vec<Package>>) -> Vec<Vec<TxIndex>> {
        blocks
            .into_iter()
            .map(|block| block.into_iter().flat_map(|pkg| pkg.txs).collect())
            .collect()
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
