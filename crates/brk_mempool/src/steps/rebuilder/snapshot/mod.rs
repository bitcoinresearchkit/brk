mod builder;
mod fees;
mod stats;
mod tx;
mod tx_index;

pub(crate) use builder::{PrefixIndex, build_txs};
pub use stats::BlockStats;
pub use tx::SnapTx;
pub use tx_index::TxIndex;

use std::hash::{DefaultHasher, Hash, Hasher};

use brk_types::{FeeRate, NextBlockHash, RecommendedFees, Txid, TxidPrefix};

use fees::Fees;

#[derive(Default)]
pub struct Snapshot {
    /// Dense per-tx data indexed by `TxIndex`. Each entry carries the
    /// linearized chunk rate (computed locally at snapshot build time)
    /// plus resolved parent/child adjacency, so CPFP queries don't
    /// re-read any external state.
    pub txs: Vec<SnapTx>,
    /// Projected blocks. `blocks[0]` is Core's `getblocktemplate`
    /// (Bitcoin Core's actual selection); the rest are greedy-packed
    /// by descending chunk rate, with a final overflow block.
    pub blocks: Vec<Vec<TxIndex>>,
    pub block_stats: Vec<BlockStats>,
    pub fees: RecommendedFees,
    /// Content hash of the projected next block. Same value as the
    /// mempool ETag.
    pub next_block_hash: NextBlockHash,
    /// Per-snapshot `TxidPrefix -> TxIndex` index, so live queries can
    /// resolve a prefix to the snapshot's compact index without
    /// re-walking `txs`. Built once by `build_txs` and reused by the
    /// rebuilder for GBT mapping.
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
        let block_stats = BlockStats::for_blocks(&blocks, &txs);
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

    fn hash_next_block(blocks: &[Vec<TxIndex>]) -> NextBlockHash {
        let Some(block) = blocks.first() else {
            return NextBlockHash::ZERO;
        };
        let mut hasher = DefaultHasher::new();
        block.hash(&mut hasher);
        NextBlockHash::new(hasher.finish())
    }

    pub fn tx(&self, idx: TxIndex) -> Option<&SnapTx> {
        self.txs.get(idx.as_usize())
    }

    pub fn idx_of(&self, prefix: &TxidPrefix) -> Option<TxIndex> {
        self.prefix_to_idx.get(prefix).copied()
    }

    /// Txids of `blocks[0]` (Core's `getblocktemplate` selection),
    /// in template order. Empty for a default snapshot.
    pub fn block0_txids(&self) -> impl Iterator<Item = Txid> + '_ {
        self.blocks
            .first()
            .into_iter()
            .flatten()
            .map(|idx| self.txs[idx.as_usize()].txid)
    }

    /// Linearized chunk rate for a live tx by prefix. Always fresh
    /// (recomputed each snapshot), package-aware (CPFP and ancestor
    /// chains lift correctly), and equals `fee/vsize` for singletons.
    /// Returns `None` if the tx isn't in this snapshot.
    pub fn chunk_rate_for(&self, prefix: &TxidPrefix) -> Option<FeeRate> {
        let idx = self.idx_of(prefix)?;
        Some(self.txs[idx.as_usize()].chunk_rate)
    }
}
