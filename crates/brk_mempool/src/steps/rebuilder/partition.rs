use std::cmp::Reverse;

use brk_types::{FeeRate, VSize};

use crate::cluster::{ChunkId, Cluster, ClusterId};
use crate::stores::TxIndex;

const LOOK_AHEAD_COUNT: usize = 100;

/// Packs SFL chunks (referenced by `(ClusterId, ChunkId)`) into
/// `num_blocks` blocks. The first `num_blocks - 1` are filled greedily
/// up to `VSize::MAX_BLOCK`; the last is a catch-all so no low-rate tx
/// is silently dropped (matches mempool.space).
///
/// Look-ahead respects intra-cluster order: a chunk is only taken once
/// every earlier-rate chunk of the same cluster has been placed, so a
/// child chunk never lands in an earlier block than its parent chunk.
///
/// Output is the flat tx-list per block, parents-first within each
/// chunk via the cluster's `topo_order`.
pub struct Partitioner<'a> {
    clusters: &'a [Cluster<TxIndex>],
    /// Candidate chunks sorted by descending feerate. Slots are taken
    /// (set to `None`) as they're placed.
    slots: Vec<Option<Candidate>>,
    /// Per-cluster cursor: the next `ChunkId` that must be taken next.
    cluster_next: Vec<ChunkId>,
    blocks: Vec<Vec<TxIndex>>,
    current: Vec<Candidate>,
    current_vsize: VSize,
    idx: usize,
}

#[derive(Clone, Copy)]
struct Candidate {
    cluster_id: ClusterId,
    chunk_id: ChunkId,
    fee_rate: FeeRate,
    vsize: VSize,
}

impl<'a> Partitioner<'a> {
    pub fn partition(clusters: &'a [Cluster<TxIndex>], num_blocks: usize) -> Vec<Vec<TxIndex>> {
        let mut p = Self::new(clusters, num_blocks);
        p.fill_normal_blocks(num_blocks.saturating_sub(1));
        p.flush_overflow(num_blocks);
        p.blocks
    }

    fn new(clusters: &'a [Cluster<TxIndex>], num_blocks: usize) -> Self {
        let mut candidates: Vec<Candidate> = clusters
            .iter()
            .enumerate()
            .flat_map(|(cid, cluster)| {
                let cluster_id = ClusterId::from(cid);
                cluster
                    .chunks
                    .iter()
                    .enumerate()
                    .map(move |(chid, chunk)| Candidate {
                        cluster_id,
                        chunk_id: ChunkId::from(chid),
                        fee_rate: chunk.fee_rate(),
                        vsize: chunk.vsize,
                    })
            })
            .collect();
        // Stable sort preserves SFL's per-cluster non-increasing-rate
        // order, which is what `cluster_next` relies on.
        candidates.sort_by_key(|c| Reverse(c.fee_rate));

        Self {
            clusters,
            slots: candidates.into_iter().map(Some).collect(),
            cluster_next: vec![ChunkId::ZERO; clusters.len()],
            blocks: Vec::with_capacity(num_blocks),
            current: Vec::new(),
            current_vsize: VSize::default(),
            idx: 0,
        }
    }

    fn fill_normal_blocks(&mut self, target_blocks: usize) {
        while self.idx < self.slots.len() && self.blocks.len() < target_blocks {
            let Some(cand) = self.slots[self.idx] else {
                self.idx += 1;
                continue;
            };

            let remaining_space = VSize::MAX_BLOCK.saturating_sub(self.current_vsize);

            // Take if it fits, or if the current block is empty (avoids
            // stalling on an oversized chunk larger than MAX_BLOCK).
            if cand.vsize <= remaining_space || self.current.is_empty() {
                self.take(self.idx);
                self.idx += 1;
                continue;
            }

            if self.try_fill_with_smaller(self.idx, remaining_space) {
                continue;
            }

            self.flush_block();
        }

        if !self.current.is_empty() && self.blocks.len() < target_blocks {
            self.flush_block();
        }
    }

    /// Skips any candidate whose cluster has an earlier unplaced chunk:
    /// that chunk's parents would land after its children.
    fn try_fill_with_smaller(&mut self, start: usize, remaining_space: VSize) -> bool {
        let end = (start + LOOK_AHEAD_COUNT).min(self.slots.len());
        for idx in (start + 1)..end {
            let Some(cand) = self.slots[idx] else {
                continue;
            };
            if cand.vsize > remaining_space {
                continue;
            }
            if cand.chunk_id != self.cluster_next[cand.cluster_id.as_usize()] {
                continue;
            }
            self.take(idx);
            return true;
        }
        false
    }

    fn take(&mut self, idx: usize) {
        let cand = self.slots[idx].take().unwrap();
        debug_assert_eq!(
            cand.chunk_id,
            self.cluster_next[cand.cluster_id.as_usize()],
            "partitioner took a chunk out of cluster order"
        );
        self.cluster_next[cand.cluster_id.as_usize()] = ChunkId::from(cand.chunk_id.inner() + 1);
        self.current_vsize += cand.vsize;
        self.current.push(cand);
    }

    fn flush_block(&mut self) {
        let candidates = std::mem::take(&mut self.current);
        let block = Self::materialize(self.clusters, candidates);
        self.blocks.push(block);
        self.current_vsize = VSize::default();
    }

    fn flush_overflow(&mut self, num_blocks: usize) {
        if self.blocks.len() >= num_blocks {
            return;
        }
        let overflow: Vec<Candidate> = self.slots[self.idx..]
            .iter_mut()
            .filter_map(Option::take)
            .collect();
        if !overflow.is_empty() {
            let block = Self::materialize(self.clusters, overflow);
            self.blocks.push(block);
        }
    }

    /// Expand each chunk into its txs. `chunk.txs` is already topo-ordered
    /// (parents-first) by `Cluster::new`, so we iterate it directly.
    fn materialize(clusters: &[Cluster<TxIndex>], candidates: Vec<Candidate>) -> Vec<TxIndex> {
        let mut out: Vec<TxIndex> = Vec::new();
        for cand in candidates {
            let cluster = &clusters[cand.cluster_id.as_usize()];
            let chunk = &cluster.chunks[cand.chunk_id.as_usize()];
            for &local in &chunk.txs {
                out.push(cluster.nodes[local.as_usize()].id);
            }
        }
        out
    }
}
