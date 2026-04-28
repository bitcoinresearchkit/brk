use std::cmp::Reverse;

use brk_types::VSize;

use super::linearize::Package;

const LOOK_AHEAD_COUNT: usize = 100;

/// Packs ranked packages into `num_blocks` blocks. The first
/// `num_blocks - 1` are filled greedily up to `VSize::MAX_BLOCK`; the last
/// is a catch-all so no low-rate tx is silently dropped (matches
/// mempool.space).
///
/// Look-ahead respects intra-cluster order: a chunk is only taken once
/// every earlier-rate chunk of the same cluster has been placed, so a
/// child chunk never lands in an earlier block than its parent chunk.
pub struct Partitioner {
    slots: Vec<Option<Package>>,
    blocks: Vec<Vec<Package>>,
    cluster_next: Vec<u32>,
    current: Vec<Package>,
    current_vsize: VSize,
    idx: usize,
}

impl Partitioner {
    pub fn partition(mut packages: Vec<Package>, num_blocks: usize) -> Vec<Vec<Package>> {
        // Stable sort preserves SFL's per-cluster non-increasing-rate
        // emission order in the global list, which is what `cluster_next`
        // relies on.
        packages.sort_by_key(|p| Reverse(p.fee_rate));

        let mut p = Self::new(packages, num_blocks);
        p.fill_normal_blocks(num_blocks.saturating_sub(1));
        p.flush_overflow(num_blocks);
        p.blocks
    }

    fn new(packages: Vec<Package>, num_blocks: usize) -> Self {
        let num_clusters = packages
            .iter()
            .map(|p| p.cluster_id as usize + 1)
            .max()
            .unwrap_or(0);
        Self {
            cluster_next: vec![0; num_clusters],
            slots: packages.into_iter().map(Some).collect(),
            blocks: Vec::with_capacity(num_blocks),
            current: Vec::new(),
            current_vsize: VSize::default(),
            idx: 0,
        }
    }

    fn fill_normal_blocks(&mut self, target_blocks: usize) {
        while self.idx < self.slots.len() && self.blocks.len() < target_blocks {
            let Some(pkg) = &self.slots[self.idx] else {
                self.idx += 1;
                continue;
            };

            let remaining_space = VSize::MAX_BLOCK.saturating_sub(self.current_vsize);

            // Take if it fits, or if the current block is empty (avoids
            // stalling on an oversized package larger than MAX_BLOCK).
            if pkg.vsize <= remaining_space || self.current.is_empty() {
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
            let Some(pkg) = &self.slots[idx] else { continue };
            if pkg.vsize > remaining_space {
                continue;
            }
            if pkg.chunk_order != self.cluster_next[pkg.cluster_id as usize] {
                continue;
            }
            self.take(idx);
            return true;
        }
        false
    }

    fn take(&mut self, idx: usize) {
        let pkg = self.slots[idx].take().unwrap();
        debug_assert_eq!(
            pkg.chunk_order, self.cluster_next[pkg.cluster_id as usize],
            "partitioner took a chunk out of cluster order"
        );
        self.cluster_next[pkg.cluster_id as usize] = pkg.chunk_order + 1;
        self.current_vsize += pkg.vsize;
        self.current.push(pkg);
    }

    fn flush_block(&mut self) {
        self.blocks.push(std::mem::take(&mut self.current));
        self.current_vsize = VSize::default();
    }

    fn flush_overflow(&mut self, num_blocks: usize) {
        if self.blocks.len() >= num_blocks {
            return;
        }
        let overflow: Vec<Package> = self.slots[self.idx..]
            .iter_mut()
            .filter_map(Option::take)
            .collect();
        if !overflow.is_empty() {
            self.blocks.push(overflow);
        }
    }
}
