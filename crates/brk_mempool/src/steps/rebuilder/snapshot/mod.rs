mod fees;
mod stats;

pub use stats::BlockStats;

use std::hash::{DefaultHasher, Hash, Hasher};

use brk_types::{FeeRate, RecommendedFees};

use crate::TxEntry;
use crate::cluster::{Cluster, ClusterRef};
use crate::stores::TxIndex;

use fees::Fees;

#[derive(Default)]
pub struct Snapshot {
    /// SFL-linearized cluster forest. Snapshot is `Arc`'d, so consumers
    /// share the cluster data without cloning. Each `ClusterNode.id`
    /// is the live `TxIndex` (pool slot) of that node.
    pub clusters: Vec<Cluster<TxIndex>>,
    /// Reverse of `clusters`: indexed by `TxIndex.as_usize()`. `None`
    /// means the slot is empty (between two cycles a tx confirmed/was
    /// evicted) or never made it into the live pool. Read via
    /// `cluster_of(idx)` from outside the snapshot.
    cluster_of: Vec<Option<ClusterRef>>,
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
    pub fn build(
        clusters: Vec<Cluster<TxIndex>>,
        cluster_of: Vec<Option<ClusterRef>>,
        blocks: Vec<Vec<TxIndex>>,
        entries: &[Option<TxEntry>],
        min_fee: FeeRate,
    ) -> Self {
        let block_stats: Vec<BlockStats> = blocks
            .iter()
            .map(|block| BlockStats::compute(block, &clusters, &cluster_of, entries))
            .collect();
        let fees = Fees::compute(&block_stats, min_fee);
        let next_block_hash = Self::hash_next_block(&blocks);

        Self {
            clusters,
            cluster_of,
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

    /// Cluster + local position for a live tx, or `None` if the slot
    /// is empty or `idx` is out of range.
    pub fn cluster_of(&self, idx: TxIndex) -> Option<ClusterRef> {
        self.cluster_of.get(idx.as_usize()).copied().flatten()
    }

    pub fn cluster_of_len(&self) -> usize {
        self.cluster_of.len()
    }

    pub fn cluster_of_active(&self) -> usize {
        self.cluster_of.iter().filter(|c| c.is_some()).count()
    }

    /// SFL chunk feerate for a live tx, or `None` if it isn't in any
    /// cluster. Cheap shortcut for callers that need the rate but not
    /// the full `CpfpInfo`.
    pub fn chunk_rate_of(&self, idx: TxIndex) -> Option<FeeRate> {
        let ClusterRef { cluster_id, local } = self.cluster_of(idx)?;
        Some(
            self.clusters[cluster_id.as_usize()]
                .chunk_of(local)
                .fee_rate(),
        )
    }
}
