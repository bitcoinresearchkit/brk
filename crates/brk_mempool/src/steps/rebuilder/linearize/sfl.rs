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
//! cap of 100). Rate comparisons go through `FeeRate`.

use brk_types::{FeeRate, Sats, VSize};

use super::LocalIdx;
use super::chunk::Chunk;
use super::cluster::Cluster;

const BRUTE_FORCE_LIMIT: usize = 18;
const BITMASK_LIMIT: usize = 128;

pub struct Sfl;

impl Sfl {
    pub fn linearize(cluster: &Cluster) -> Vec<Chunk> {
        assert!(
            cluster.nodes.len() <= BITMASK_LIMIT,
            "cluster size {} exceeds u128 capacity",
            cluster.nodes.len()
        );
        let tables = Tables::build(cluster);
        let chunks = Self::extract_chunks(&tables);
        Self::canonicalize(chunks)
    }

    /// Peel the cluster one chunk at a time. Each iteration picks the
    /// highest-feerate topologically-closed subset of `remaining` and
    /// removes it. Loop terminates because every iteration removes at
    /// least one node.
    fn extract_chunks(t: &Tables) -> Vec<Chunk> {
        let mut chunks: Vec<Chunk> = Vec::new();
        let mut remaining: u128 = t.all;
        while remaining != 0 {
            let (mask, fee, vsize) = if t.n <= BRUTE_FORCE_LIMIT {
                Self::best_subset(t, remaining)
            } else {
                Self::best_ancestor_union(t, remaining)
            };
            chunks.push(Chunk::from_mask(mask, fee, vsize));
            remaining &= !mask;
        }
        chunks
    }

    /// Recursive enumeration of topologically-closed subsets of
    /// `remaining`. Returns the (mask, fee, vsize) with the highest rate.
    fn best_subset(t: &Tables, remaining: u128) -> (u128, Sats, VSize) {
        let ctx = Ctx { tables: t, remaining };
        let mut best = (0u128, Sats::ZERO, VSize::default());
        Self::recurse(&ctx, 0, 0, Sats::ZERO, VSize::default(), &mut best);
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
        if idx == ctx.tables.topo_order.len() {
            if included != 0 && FeeRate::from((f, v)) > FeeRate::from((best.1, best.2)) {
                *best = (included, f, v);
            }
            return;
        }
        let node = ctx.tables.topo_order[idx];
        let bit = 1u128 << node;

        // Not in remaining, or a parent (within remaining) is excluded:
        // this node is forced-excluded, no branching.
        if (bit & ctx.remaining) == 0
            || (ctx.tables.parents_mask[node as usize] & ctx.remaining & !included) != 0
        {
            Self::recurse(ctx, idx + 1, included, f, v, best);
            return;
        }

        Self::recurse(ctx, idx + 1, included, f, v, best);
        Self::recurse(
            ctx,
            idx + 1,
            included | bit,
            f + ctx.tables.fee_of[node as usize],
            v + ctx.tables.vsize_of[node as usize],
            best,
        );
    }

    /// For each node v in `remaining`, seed with anc(v) ∩ remaining, then
    /// greedily extend by adding any anc(u) whose inclusion raises the
    /// feerate. Pick the best result across all seeds.
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
            let (mut f, mut v) = Self::totals(s, &t.fee_of, &t.vsize_of);
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
                    let (df, dv) = Self::totals(add, &t.fee_of, &t.vsize_of);
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

            if rate > best_rate {
                best = (s, f, v);
                best_rate = rate;
            }
        }
        best
    }

    /// Single-pass stack merge: for each incoming chunk, merge it into
    /// the stack top while the merge would raise the top's feerate, then
    /// push. O(n) total regardless of how many merges cascade.
    fn canonicalize(chunks: Vec<Chunk>) -> Vec<Chunk> {
        let mut out: Vec<Chunk> = Vec::with_capacity(chunks.len());
        for mut cur in chunks {
            while let Some(top) = out.last() {
                if cur.fee_rate() <= top.fee_rate() {
                    break;
                }
                let mut prev = out.pop().unwrap();
                prev.fee += cur.fee;
                prev.vsize += cur.vsize;
                prev.nodes.extend(cur.nodes);
                cur = prev;
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
}

/// Per-cluster precomputed bitmasks and lookups, shared across every
/// chunk-extraction iteration. Built once in `Sfl::linearize`.
struct Tables {
    n: usize,
    /// Bitmask with one bit set per node (i.e. `(1 << n) - 1`).
    all: u128,
    /// `parents_mask[i]` = bits set for direct parents of node `i`.
    parents_mask: Vec<u128>,
    /// `ancestor_incl[i]` = bits set for `i` and all ancestors.
    ancestor_incl: Vec<u128>,
    /// LocalIdx order respecting `cluster.topo_rank`.
    topo_order: Vec<LocalIdx>,
    fee_of: Vec<Sats>,
    vsize_of: Vec<VSize>,
}

impl Tables {
    fn build(cluster: &Cluster) -> Self {
        let n = cluster.nodes.len();
        let topo_order = Self::build_topo_order(cluster);
        let (parents_mask, ancestor_incl) = Self::build_ancestor_masks(cluster, &topo_order);
        let fee_of: Vec<Sats> = cluster.nodes.iter().map(|node| node.fee).collect();
        let vsize_of: Vec<VSize> = cluster.nodes.iter().map(|node| node.vsize).collect();
        let all: u128 = if n == 128 { !0 } else { (1u128 << n) - 1 };
        Self {
            n,
            all,
            parents_mask,
            ancestor_incl,
            topo_order,
            fee_of,
            vsize_of,
        }
    }

    fn build_topo_order(cluster: &Cluster) -> Vec<LocalIdx> {
        let mut topo_order: Vec<LocalIdx> = (0..cluster.nodes.len() as LocalIdx).collect();
        topo_order.sort_by_key(|&i| cluster.topo_rank[i as usize]);
        topo_order
    }

    /// For each node `v`, compute its direct-parent bitmask and the
    /// closure of all its ancestors (including itself). Visits nodes
    /// in topological order so a parent's `ancestor_incl` is ready
    /// before any child reads it.
    fn build_ancestor_masks(
        cluster: &Cluster,
        topo_order: &[LocalIdx],
    ) -> (Vec<u128>, Vec<u128>) {
        let n = cluster.nodes.len();
        let mut parents_mask: Vec<u128> = vec![0; n];
        let mut ancestor_incl: Vec<u128> = vec![0; n];
        for &v in topo_order {
            let mut par = 0u128;
            let mut acc = 1u128 << v;
            for &p in &cluster.nodes[v as usize].parents {
                par |= 1u128 << p;
                acc |= ancestor_incl[p as usize];
            }
            parents_mask[v as usize] = par;
            ancestor_incl[v as usize] = acc;
        }
        (parents_mask, ancestor_incl)
    }
}

/// Per-iteration immutable bundle for the brute-force recursion.
/// Keeping it small lets `recurse` stay at four moving args.
struct Ctx<'a> {
    tables: &'a Tables,
    remaining: u128,
}
