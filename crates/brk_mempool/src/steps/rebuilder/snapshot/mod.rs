mod blk_index;
mod fees;
mod stats;

pub use blk_index::BlkIndex;
pub use stats::BlockStats;

use std::hash::{DefaultHasher, Hash, Hasher};

use brk_types::{FeeRate, RecommendedFees};

use super::linearize::Package;
use crate::{TxEntry, stores::TxIndex};

use fees::Fees;

#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    pub blocks: Vec<Vec<TxIndex>>,
    /// Reverse of `blocks`: indexed by `TxIndex.as_usize()`. Slots that
    /// hold no entry, or hold an entry that didn't make any projected
    /// block, store `BlkIndex::MAX`. Read via the `block_of` accessor.
    block_of: Vec<BlkIndex>,
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
        let block_of = Self::build_block_of(&blocks, entries.len());
        let next_block_hash = Self::hash_next_block(&blocks);

        Self {
            blocks,
            block_of,
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

    /// One pass over `blocks` to invert the mapping. `BlkIndex::MAX`
    /// stays as the sentinel for slots that aren't in any projected
    /// block (empty slots and below-floor txs alike).
    fn build_block_of(blocks: &[Vec<TxIndex>], entry_count: usize) -> Vec<BlkIndex> {
        let mut block_of = vec![BlkIndex::MAX; entry_count];
        for (b, txs) in blocks.iter().enumerate() {
            let blk = BlkIndex::from(b);
            for &idx in txs {
                block_of[idx.as_usize()] = blk;
            }
        }
        block_of
    }

    fn hash_next_block(blocks: &[Vec<TxIndex>]) -> u64 {
        let Some(block) = blocks.first() else {
            return 0;
        };
        let mut hasher = DefaultHasher::new();
        block.hash(&mut hasher);
        hasher.finish()
    }

    /// Projected block that holds `idx`, or `None` if the tx is below
    /// the mempool floor (or `idx` is out of range).
    pub fn block_of(&self, idx: TxIndex) -> Option<BlkIndex> {
        self.block_of
            .get(idx.as_usize())
            .copied()
            .filter(|b| !b.is_not_in_projected())
    }
}
