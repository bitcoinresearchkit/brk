//! Cluster mempool linearization (Core 31's "Single Fee Linearization").
//!
//! Given a topologically ordered cluster (parents before children) with
//! per-tx `(fee, vsize)` and parent edges as local indices, partition the
//! cluster into chunks ordered by descending feerate, where each chunk is
//! the highest-rate ancestor-closed set of remaining txs.
//!
//! The "lift" merging this implements is what makes CPFP visible at the
//! cluster level: a child whose rate exceeds its parent's rate gets folded
//! into a chunk with the parent, and the chunk's rate is the combined
//! `(parent_fee + child_fee) / (parent_vsize + child_vsize)`. Cascades
//! upward through any further parents until rates are non-increasing.
//!
//! This is the proxy-fallback case; under Core 31+ each tx's `fees.chunk`
//! / `chunkweight` already encodes the chunked rate, so all members of a
//! chunk would share that rate. Computing locally from `(fee, vsize)`
//! gives the same answer either way and works on older Core too.
//!
//! Complexity is `O(n^2)` per linearization (n bounded by cluster cap),
//! matching mempool.space's frontend implementation.

use brk_types::{CpfpClusterChunk, CpfpClusterTxIndex, FeeRate, Sats, VSize};
use rustc_hash::{FxBuildHasher, FxHashSet};

/// One cluster member: its `(fee, vsize)` and parent edges as
/// local indices into the same array.
pub struct ChunkInput<'a> {
    pub fee: Sats,
    pub vsize: VSize,
    pub parents: &'a [CpfpClusterTxIndex],
}

/// Linearize `items` into chunks. `items` must be in topological order
/// (parents before children); `parents` indices must point earlier in
/// the slice. Returns chunks sorted by descending feerate, with each
/// chunk's `txs` listed in the input topological order.
pub fn linearize(items: &[ChunkInput<'_>]) -> Vec<CpfpClusterChunk> {
    let n = items.len();
    if n == 0 {
        return Vec::new();
    }
    let mut remaining: Vec<bool> = vec![true; n];
    let mut chunks: Vec<CpfpClusterChunk> = Vec::new();

    while remaining.iter().any(|&r| r) {
        let mut best: Option<(FeeRate, FxHashSet<u32>)> = None;
        for i in 0..n {
            if !remaining[i] {
                continue;
            }
            let mut anc: FxHashSet<u32> =
                FxHashSet::with_capacity_and_hasher(8, FxBuildHasher);
            let mut stack: Vec<u32> = vec![i as u32];
            while let Some(x) = stack.pop() {
                if !anc.insert(x) {
                    continue;
                }
                for &p in items[x as usize].parents {
                    let pu: u32 = u32::from(p);
                    if remaining[pu as usize] && !anc.contains(&pu) {
                        stack.push(pu);
                    }
                }
            }
            let mut fee = Sats::ZERO;
            let mut vsize = VSize::from(0u64);
            for &x in &anc {
                fee += items[x as usize].fee;
                vsize += items[x as usize].vsize;
            }
            let rate = FeeRate::from((fee, vsize));
            match &best {
                Some((br, _)) if *br >= rate => {}
                _ => best = Some((rate, anc)),
            }
        }

        let (rate, set) = best.expect("at least one remaining tx");
        let mut indices: Vec<u32> = set.into_iter().collect();
        indices.sort_unstable();
        for &x in &indices {
            remaining[x as usize] = false;
        }
        let txs: Vec<CpfpClusterTxIndex> =
            indices.into_iter().map(CpfpClusterTxIndex::from).collect();
        chunks.push(CpfpClusterChunk { txs, feerate: rate });
    }
    chunks
}
