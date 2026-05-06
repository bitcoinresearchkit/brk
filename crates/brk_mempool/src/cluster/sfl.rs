//! Cluster linearizer.
//!
//! Two-branch dispatch by cluster size:
//! - **n ≤ 18**: recursive enumeration of topologically-closed subsets.
//!   Provably optimal. Visits only valid subsets (skips non-closed ones
//!   without filtering) and maintains running fee/vsize incrementally.
//! - **n > 18**: "greedy-union" ancestor-set search. Seeds with each
//!   node's ancestor closure, then greedily adds any other ancestor
//!   closure whose inclusion raises the combined feerate. Strict
//!   superset of ancestor-set-sort's candidate space, catching the
//!   sibling-union shapes that pure ASS misses.
//!
//! A final stack-based `canonicalize` pass merges adjacent chunks when
//! the later one's feerate beats the earlier's, restoring the
//! non-increasing-rate invariant.
//!
//! Everything runs on `u128` bitmasks (covers Bitcoin Core 31's cluster
//! cap of 100). Rate comparisons go through `FeeRate`. The caller is
//! `Cluster::new`, which has already permuted nodes into topological
//! order — so `LocalIdx == position == topological rank`, and this
//! module never has to take a `topo_order` permutation.

use brk_types::{FeeRate, Sats, VSize};

use super::ClusterNode;

const BRUTE_FORCE_LIMIT: usize = 18;
/// Cluster nodes are indexed by `u128` bitmask, so `n < 128`. Bitcoin
/// Core's cluster cap is 100, so this leaves comfortable margin.
pub(super) const BITMASK_LIMIT: usize = 128;

/// Raw SFL output: a chunk's bitmask plus its totals. `Cluster::new`
/// converts these into final `Chunk`s with topo-ordered `txs`, so the
/// algorithm doesn't have to materialize them itself.
pub(super) struct ChunkMask {
    pub mask: u128,
    pub fee: Sats,
    pub vsize: VSize,
}

impl ChunkMask {
    fn fee_rate(&self) -> FeeRate {
        FeeRate::from((self.fee, self.vsize))
    }
}

/// Linearize a cluster into SFL chunks.
///
/// Precondition: `nodes.len() < BITMASK_LIMIT`. `Cluster::new` enforces
/// this by dispatching oversized clusters to a trivial fallback before
/// reaching here, so the check is `debug_assert!` rather than runtime.
pub(super) fn linearize<I>(nodes: &[ClusterNode<I>]) -> Vec<ChunkMask> {
    debug_assert!(
        nodes.len() < BITMASK_LIMIT,
        "cluster size {} exceeds u128 capacity",
        nodes.len()
    );
    let tables = Tables::build(nodes);
    let chunks = extract_chunks(&tables);
    canonicalize(chunks)
}

/// Peel the cluster one chunk at a time. Each iteration picks the
/// highest-feerate topologically-closed subset of `remaining` and
/// removes it. Loop terminates because every iteration removes at
/// least one node.
fn extract_chunks(t: &Tables) -> Vec<ChunkMask> {
    let pick: fn(&Tables, u128) -> (u128, Sats, VSize) = if t.n <= BRUTE_FORCE_LIMIT {
        best_subset
    } else {
        best_ancestor_union
    };
    let mut chunks: Vec<ChunkMask> = Vec::new();
    let mut remaining: u128 = t.all;
    while remaining != 0 {
        let (mask, fee, vsize) = pick(t, remaining);
        chunks.push(ChunkMask { mask, fee, vsize });
        remaining &= !mask;
    }
    chunks
}

/// Recursive enumeration of topologically-closed subsets of
/// `remaining`. Returns the (mask, fee, vsize) with the highest rate;
/// when `remaining` is all zero-fee (e.g. a CPFP-parent leftover after
/// the paying chunk was extracted), the first non-empty subset wins so
/// `extract_chunks` always makes progress. Iterates nodes by index
/// `0..n`; since the cluster is stored in topological order, that *is*
/// a topological sweep.
fn best_subset(t: &Tables, remaining: u128) -> (u128, Sats, VSize) {
    let ctx = Ctx {
        tables: t,
        remaining,
    };
    let mut best = (0u128, Sats::ZERO, VSize::default());
    recurse(&ctx, 0, 0, Sats::ZERO, VSize::default(), &mut best);
    best
}

fn recurse(
    ctx: &Ctx,
    idx: usize,
    included: u128,
    f: Sats,
    v: VSize,
    best: &mut (u128, Sats, VSize),
) {
    if idx == ctx.tables.n {
        if included != 0 && (best.0 == 0 || FeeRate::from((f, v)) > FeeRate::from((best.1, best.2)))
        {
            *best = (included, f, v);
        }
        return;
    }
    let bit = 1u128 << idx;

    // Not in remaining, or a parent (within remaining) is excluded:
    // this node is forced-excluded, no branching.
    if (bit & ctx.remaining) == 0 || (ctx.tables.parents_mask[idx] & ctx.remaining & !included) != 0
    {
        recurse(ctx, idx + 1, included, f, v, best);
        return;
    }

    recurse(ctx, idx + 1, included, f, v, best);
    recurse(
        ctx,
        idx + 1,
        included | bit,
        f + ctx.tables.fee_of[idx],
        v + ctx.tables.vsize_of[idx],
        best,
    );
}

/// For each node v in `remaining`, seed with anc(v) ∩ remaining, then
/// greedily extend by adding any anc(u) whose inclusion raises the
/// feerate. Pick the best result across all seeds; when every seed has
/// rate 0 (e.g. a CPFP-parent leftover after the paying chunk was
/// extracted), the first seed wins so `extract_chunks` always makes
/// progress.
///
/// Every candidate evaluated is a union of ancestor closures, so it
/// is topologically closed by construction. Strictly explores more
/// candidates than pure ancestor-set-sort, at O(n³) per chunk step.
fn best_ancestor_union(t: &Tables, remaining: u128) -> (u128, Sats, VSize) {
    let mut best = (0u128, Sats::ZERO, VSize::default());
    let mut best_rate = FeeRate::default();
    let mut seeds = remaining;
    while seeds != 0 {
        let i = seeds.trailing_zeros() as usize;
        seeds &= seeds - 1;

        let mut s = t.ancestor_incl[i] & remaining;
        let (mut f, mut v) = totals(s, &t.fee_of, &t.vsize_of);
        let mut rate = FeeRate::from((f, v));

        // Greedy extension to fixed point: pick the ancestor-closure
        // addition that yields the highest resulting feerate, if any.
        loop {
            let mut picked: Option<(u128, Sats, VSize, FeeRate)> = None;
            let mut cands = remaining & !s;
            while cands != 0 {
                let j = cands.trailing_zeros() as usize;
                cands &= cands - 1;
                let add = t.ancestor_incl[j] & remaining & !s;
                if add == 0 {
                    continue;
                }
                let (df, dv) = totals(add, &t.fee_of, &t.vsize_of);
                let nf = f + df;
                let nv = v + dv;
                let nrate = FeeRate::from((nf, nv));
                if nrate <= rate {
                    continue;
                }
                if picked.is_none_or(|(_, _, _, prate)| nrate > prate) {
                    picked = Some((add, nf, nv, nrate));
                }
            }
            match picked {
                Some((add, nf, nv, nrate)) => {
                    s |= add;
                    f = nf;
                    v = nv;
                    rate = nrate;
                }
                None => break,
            }
        }

        if best.0 == 0 || rate > best_rate {
            best = (s, f, v);
            best_rate = rate;
        }
    }
    best
}

/// Single-pass stack merge: for each incoming chunk, merge it into
/// the stack top while the merge would raise the top's feerate, then
/// push. O(n) total regardless of how many merges cascade.
fn canonicalize(chunks: Vec<ChunkMask>) -> Vec<ChunkMask> {
    let mut out: Vec<ChunkMask> = Vec::with_capacity(chunks.len());
    for mut cur in chunks {
        while let Some(top) = out.last() {
            if cur.fee_rate() <= top.fee_rate() {
                break;
            }
            let prev = out.pop().unwrap();
            cur = ChunkMask {
                mask: prev.mask | cur.mask,
                fee: prev.fee + cur.fee,
                vsize: prev.vsize + cur.vsize,
            };
        }
        out.push(cur);
    }
    out
}

#[inline]
fn totals(mask: u128, fee_of: &[Sats], vsize_of: &[VSize]) -> (Sats, VSize) {
    let mut f = Sats::ZERO;
    let mut v = VSize::default();
    let mut bits = mask;
    while bits != 0 {
        let i = bits.trailing_zeros() as usize;
        f += fee_of[i];
        v += vsize_of[i];
        bits &= bits - 1;
    }
    (f, v)
}

/// Per-cluster precomputed bitmasks and lookups, shared across every
/// chunk-extraction iteration. Built once in `linearize`.
struct Tables {
    n: usize,
    /// Bitmask with one bit set per node (i.e. `(1 << n) - 1`).
    all: u128,
    /// `parents_mask[i]` = bits set for direct parents of node `i`.
    parents_mask: Vec<u128>,
    /// `ancestor_incl[i]` = bits set for `i` and all ancestors.
    ancestor_incl: Vec<u128>,
    fee_of: Vec<Sats>,
    vsize_of: Vec<VSize>,
}

impl Tables {
    /// Single pass over nodes (in topological order, so each parent's
    /// `ancestor_incl` is ready before the child reads it): build
    /// parent-bit masks, ancestor closures, and pick out fee/vsize.
    fn build<I>(nodes: &[ClusterNode<I>]) -> Self {
        let n = nodes.len();
        let mut parents_mask: Vec<u128> = vec![0; n];
        let mut ancestor_incl: Vec<u128> = vec![0; n];
        let mut fee_of: Vec<Sats> = Vec::with_capacity(n);
        let mut vsize_of: Vec<VSize> = Vec::with_capacity(n);
        for (vi, node) in nodes.iter().enumerate() {
            let mut par = 0u128;
            let mut acc = 1u128 << vi;
            for &p in &node.parents {
                par |= 1u128 << p.inner();
                acc |= ancestor_incl[p.as_usize()];
            }
            parents_mask[vi] = par;
            ancestor_incl[vi] = acc;
            fee_of.push(node.fee);
            vsize_of.push(node.vsize);
        }
        Self {
            n,
            all: (1u128 << n) - 1,
            parents_mask,
            ancestor_incl,
            fee_of,
            vsize_of,
        }
    }
}

/// Per-iteration immutable bundle for the brute-force recursion.
/// Keeping it small lets `recurse` stay at four moving args.
struct Ctx<'a> {
    tables: &'a Tables,
    remaining: u128,
}
