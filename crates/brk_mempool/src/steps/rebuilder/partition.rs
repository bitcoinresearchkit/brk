//! Pack live txs into projected blocks 1..N, sorted by descending
//! `chunk_rate`. Block 0 is filled by the caller from `getblocktemplate`
//! (Core's actual selection); blocks 1..N feed
//! `/api/v1/fees/mempool-blocks` as a coarse fee-tier gradient.
//!
//! No topological gate: a child can sit before its parent within a
//! tied-rate run, but every cluster member shares its chunk's
//! `chunk_rate` (linearized at snapshot time) so chunk-mates land in
//! the same block, and the only output is a per-block rate
//! distribution where intra-block order is invisible.
//!
//! The final block is a catch-all (no vsize cap) so leftover tail
//! vsize is accounted for instead of silently dropped.
//!
//! Walk sorted candidates once. For each, push into the current
//! block if it fits; otherwise advance to the next block (unless we
//! are already on the last one, which absorbs everything remaining).

use std::cmp::Reverse;

use brk_types::{FeeRate, VSize};
use rustc_hash::FxHashSet;

use super::snapshot::{SnapTx, TxIndex};

pub struct Partitioner;

impl Partitioner {
    pub fn partition(
        txs: &[SnapTx],
        excluded: &FxHashSet<TxIndex>,
        num_remaining_blocks: usize,
    ) -> Vec<Vec<TxIndex>> {
        if num_remaining_blocks == 0 {
            return Vec::new();
        }
        let sorted = sorted_candidates(txs, excluded);
        let mut blocks: Vec<Vec<TxIndex>> = (0..num_remaining_blocks).map(|_| Vec::new()).collect();
        let mut block_vsize = VSize::default();
        let mut current = 0;
        let last = num_remaining_blocks - 1;
        for (idx, vsize, _) in sorted {
            let fits = vsize <= VSize::MAX_BLOCK.saturating_sub(block_vsize);
            if !fits && current < last && !blocks[current].is_empty() {
                current += 1;
                block_vsize = VSize::default();
            }
            blocks[current].push(idx);
            block_vsize += vsize;
        }
        blocks
    }
}

fn sorted_candidates(
    txs: &[SnapTx],
    excluded: &FxHashSet<TxIndex>,
) -> Vec<(TxIndex, VSize, FeeRate)> {
    let mut cands: Vec<(TxIndex, VSize, FeeRate)> = txs
        .iter()
        .enumerate()
        .filter_map(|(i, t)| {
            let idx = TxIndex::from(i);
            (!excluded.contains(&idx)).then_some((idx, t.vsize, t.chunk_rate))
        })
        .collect();
    cands.sort_by_key(|(_, _, rate)| Reverse(*rate));
    cands
}
