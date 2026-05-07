pub mod builder;
mod fees;
mod stats;
mod tx;
mod tx_index;

pub use builder::PrefixIndex;
pub use stats::BlockStats;
pub use tx::SnapTx;
pub use tx_index::TxIndex;

use std::hash::{DefaultHasher, Hash, Hasher};

use brk_types::{FeeRate, RecommendedFees, TxidPrefix};

use fees::Fees;

#[derive(Default)]
pub struct Snapshot {
    /// Dense per-tx data indexed by `TxIndex`. Each entry carries the
    /// chunk rate (Core's chunk-mempool truth or proxy fallback) plus
    /// resolved parent/child adjacency, so CPFP queries don't re-read
    /// any external state.
    pub txs: Vec<SnapTx>,
    /// Projected blocks. `blocks[0]` is Core's `getblocktemplate`
    /// (Bitcoin Core's actual selection); the rest are greedy-packed
    /// by descending chunk rate, with a final overflow block.
    pub blocks: Vec<Vec<TxIndex>>,
    pub block_stats: Vec<BlockStats>,
    pub fees: RecommendedFees,
    /// Content hash of the projected next block. Same value as the
    /// mempool ETag.
    pub next_block_hash: u64,
    /// Per-snapshot `TxidPrefix -> TxIndex` index, so live queries can
    /// resolve a prefix to the snapshot's compact index without
    /// re-walking `txs`. Built once by `builder::build_txs` and reused
    /// by the rebuilder for GBT mapping.
    prefix_to_idx: PrefixIndex,
}

impl Snapshot {
    /// `min_fee` is bitcoind's live `mempoolminfee`, used as the floor
    /// for every recommended-fee tier.
    pub fn build(
        txs: Vec<SnapTx>,
        blocks: Vec<Vec<TxIndex>>,
        prefix_to_idx: PrefixIndex,
        min_fee: FeeRate,
    ) -> Self {
        let block_stats: Vec<BlockStats> = blocks
            .iter()
            .enumerate()
            .map(|(i, block)| {
                if i == 0 {
                    BlockStats::compute_core(block, &txs)
                } else {
                    BlockStats::compute_projected(block, &txs)
                }
            })
            .collect();
        let fees = Fees::compute(&block_stats, min_fee);
        let next_block_hash = Self::hash_next_block(&blocks);
        Self {
            txs,
            blocks,
            block_stats,
            fees,
            next_block_hash,
            prefix_to_idx,
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

    pub fn tx(&self, idx: TxIndex) -> Option<&SnapTx> {
        self.txs.get(idx.as_usize())
    }

    pub fn idx_of(&self, prefix: &TxidPrefix) -> Option<TxIndex> {
        self.prefix_to_idx.get(prefix).copied()
    }

    /// Effective chunk rate for a live tx by prefix, or `None` if the
    /// tx isn't in this snapshot.
    pub fn chunk_rate_for(&self, prefix: &TxidPrefix) -> Option<FeeRate> {
        let idx = self.idx_of(prefix)?;
        Some(self.txs[idx.as_usize()].chunk_rate)
    }
}
