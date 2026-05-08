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
    let empty: FxHashSet<u32> = FxHashSet::default();

    while remaining.iter().any(|&r| r) {
        // Pick the top single-anchored ancestor-closed set. On rate
        // ties the larger set wins so a uniform-rate chain emits one
        // chunk instead of n singletons. The extension loop below
        // catches the same case at zero extra cost, but starting big
        // shaves iterations.
        let mut best: Option<(FeeRate, FxHashSet<u32>, Sats, VSize)> = None;
        for i in 0..n {
            if !remaining[i] {
                continue;
            }
            let anc = closure(items, &remaining, &empty, i as u32);
            let (fee, vsize) = sum_fee_vsize(items, &anc);
            let rate = FeeRate::from((fee, vsize));
            let replace = match &best {
                None => true,
                Some((br, ba, _, _)) => rate > *br || (rate == *br && anc.len() > ba.len()),
            };
            if replace {
                best = Some((rate, anc, fee, vsize));
            }
        }

        let (mut chunk_rate, mut anc, mut chunk_fee, mut chunk_vsize) =
            best.expect("at least one remaining tx");

        // Extend the chunk with any other remaining ancestor-closed
        // subset whose union keeps the chunk rate >= current rate.
        // SFL chunks are the *maximum* ancestor-closed set at the top
        // rate, but a single anchor only sees one connected component
        // up to the cluster root: a parent with one long chain plus
        // additional same-rate sibling children leaves the siblings
        // stranded as same-rate singleton chunks - which can even
        // appear "above" the main chunk under integer-vsize rounding,
        // breaking the descending-rate invariant.
        loop {
            let mut best_ext: Option<(FeeRate, FxHashSet<u32>, Sats, VSize)> = None;
            for i in 0..n {
                if !remaining[i] || anc.contains(&(i as u32)) {
                    continue;
                }
                let extra = closure(items, &remaining, &anc, i as u32);
                if extra.is_empty() {
                    continue;
                }
                let (ef, ev) = sum_fee_vsize(items, &extra);
                let new_fee = chunk_fee + ef;
                let new_vsize = chunk_vsize + ev;
                let new_rate = FeeRate::from((new_fee, new_vsize));
                if new_rate < chunk_rate {
                    continue;
                }
                let replace = match &best_ext {
                    None => true,
                    Some((br, _, _, _)) => new_rate > *br,
                };
                if replace {
                    best_ext = Some((new_rate, extra, new_fee, new_vsize));
                }
            }
            match best_ext {
                Some((r, e, f, v)) => {
                    anc.extend(&e);
                    chunk_fee = f;
                    chunk_vsize = v;
                    chunk_rate = r;
                }
                None => break,
            }
        }

        let mut indices: Vec<u32> = anc.into_iter().collect();
        indices.sort_unstable();
        for &x in &indices {
            remaining[x as usize] = false;
        }
        let txs: Vec<CpfpClusterTxIndex> =
            indices.into_iter().map(CpfpClusterTxIndex::from).collect();
        chunks.push(CpfpClusterChunk { txs, feerate: chunk_rate });
    }
    chunks
}

fn closure(
    items: &[ChunkInput<'_>],
    remaining: &[bool],
    excluded: &FxHashSet<u32>,
    start: u32,
) -> FxHashSet<u32> {
    let mut set: FxHashSet<u32> = FxHashSet::with_capacity_and_hasher(8, FxBuildHasher);
    if !remaining[start as usize] || excluded.contains(&start) {
        return set;
    }
    let mut stack: Vec<u32> = vec![start];
    while let Some(x) = stack.pop() {
        if !set.insert(x) {
            continue;
        }
        for &p in items[x as usize].parents {
            let pu: u32 = u32::from(p);
            if remaining[pu as usize] && !excluded.contains(&pu) && !set.contains(&pu) {
                stack.push(pu);
            }
        }
    }
    set
}

fn sum_fee_vsize(items: &[ChunkInput<'_>], set: &FxHashSet<u32>) -> (Sats, VSize) {
    let mut fee = Sats::ZERO;
    let mut vsize = VSize::from(0u64);
    for &x in set {
        fee += items[x as usize].fee;
        vsize += items[x as usize].vsize;
    }
    (fee, vsize)
}
