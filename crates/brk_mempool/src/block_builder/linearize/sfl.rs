//! Cluster linearizer.
//!
//! Two-branch dispatch by cluster size:
//! - **n ≤ 18**: recursive enumeration of topologically-closed subsets.
//!   Provably optimal. Visits only valid subsets (skips non-closed ones
//!   without filtering) and maintains running fee/vsize incrementally.
//! - **n > 18**: "greedy-union" ancestor-set search. Seeds with each
//!   node's ancestor closure, then greedily adds any other ancestor
//!   closure whose inclusion raises the combined feerate. Strict
//!   superset of ancestor-set-sort's candidate space — catches the
//!   sibling-union shapes that pure ASS misses.
//!
//! A final stack-based `canonicalize` pass merges adjacent chunks when
//! the later one's feerate beats the earlier's, restoring the
//! non-increasing-rate invariant.
//!
//! Everything runs on `u128` bitmasks (covers Bitcoin Core 31's cluster
//! cap of 100). No RNG, no spanning-forest state, no floating-point.

use smallvec::SmallVec;

use super::{Cluster, LocalIdx};

pub struct Chunk {
    pub nodes: SmallVec<[LocalIdx; 4]>,
    pub fee: u64,
    pub vsize: u64,
}

const BRUTE_FORCE_LIMIT: usize = 18;
const BITMASK_LIMIT: usize = 128;

pub fn linearize(cluster: &Cluster) -> Vec<Chunk> {
    let n = cluster.nodes.len();
    if n == 0 {
        return Vec::new();
    }
    assert!(n <= BITMASK_LIMIT, "cluster size {} exceeds u128 capacity", n);

    let mut parents_mask: Vec<u128> = vec![0; n];
    let mut ancestor_incl: Vec<u128> = vec![0; n];
    let mut order: Vec<LocalIdx> = (0..n as LocalIdx).collect();
    order.sort_by_key(|&i| cluster.topo_rank[i as usize]);
    for &v in &order {
        let mut par = 0u128;
        let mut acc = 1u128 << v;
        for &p in &cluster.nodes[v as usize].parents {
            par |= 1u128 << p;
            acc |= ancestor_incl[p as usize];
        }
        parents_mask[v as usize] = par;
        ancestor_incl[v as usize] = acc;
    }

    let fee_of: Vec<u64> = cluster.nodes.iter().map(|n| u64::from(n.fee)).collect();
    let vsize_of: Vec<u64> = cluster.nodes.iter().map(|n| u64::from(n.vsize)).collect();
    let all: u128 = if n == 128 { !0 } else { (1u128 << n) - 1 };

    let mut chunks: Vec<Chunk> = Vec::new();
    let mut remaining: u128 = all;
    while remaining != 0 {
        let (mask, fee, vsize) = if n <= BRUTE_FORCE_LIMIT {
            best_subset(remaining, &order, &parents_mask, &fee_of, &vsize_of)
        } else {
            best_ancestor_union(remaining, &ancestor_incl, &fee_of, &vsize_of)
        };
        chunks.push(chunk_of(mask, fee, vsize));
        remaining &= !mask;
    }

    canonicalize(&mut chunks);
    chunks
}

/// Recursive enumeration of topologically-closed subsets of
/// `remaining`. Returns the (mask, fee, vsize) with the highest rate.
fn best_subset(
    remaining: u128,
    topo_order: &[LocalIdx],
    parents_mask: &[u128],
    fee_of: &[u64],
    vsize_of: &[u64],
) -> (u128, u64, u64) {
    let mut best = (0u128, 0u64, 1u64);
    recurse(
        0,
        topo_order,
        parents_mask,
        remaining,
        0,
        0,
        0,
        fee_of,
        vsize_of,
        &mut best,
    );
    best
}

fn recurse(
    idx: usize,
    topo_order: &[LocalIdx],
    parents_mask: &[u128],
    remaining: u128,
    included: u128,
    f: u64,
    v: u64,
    fee_of: &[u64],
    vsize_of: &[u64],
    best: &mut (u128, u64, u64),
) {
    if idx == topo_order.len() {
        if included != 0 && f as u128 * best.2 as u128 > best.1 as u128 * v as u128 {
            *best = (included, f, v);
        }
        return;
    }
    let node = topo_order[idx];
    let bit = 1u128 << node;

    // Not in remaining, or a parent (within remaining) is excluded:
    // this node is forced-excluded, no branching.
    if (bit & remaining) == 0
        || (parents_mask[node as usize] & remaining & !included) != 0
    {
        recurse(
            idx + 1, topo_order, parents_mask, remaining, included, f, v, fee_of, vsize_of, best,
        );
        return;
    }

    // Exclude
    recurse(
        idx + 1, topo_order, parents_mask, remaining, included, f, v, fee_of, vsize_of, best,
    );
    // Include
    recurse(
        idx + 1,
        topo_order,
        parents_mask,
        remaining,
        included | bit,
        f + fee_of[node as usize],
        v + vsize_of[node as usize],
        fee_of,
        vsize_of,
        best,
    );
}

/// For each node v in `remaining`, seed with anc(v) ∩ remaining, then
/// greedily extend by adding any anc(u) whose inclusion raises the
/// feerate. Pick the best result across all seeds.
///
/// Every candidate evaluated is a union of ancestor closures —
/// topologically closed by construction. Strictly explores more
/// candidates than pure ancestor-set-sort, at O(n³) per chunk step.
fn best_ancestor_union(
    remaining: u128,
    ancestor_incl: &[u128],
    fee_of: &[u64],
    vsize_of: &[u64],
) -> (u128, u64, u64) {
    let mut best = (0u128, 0u64, 1u64);
    let mut seeds = remaining;
    while seeds != 0 {
        let i = seeds.trailing_zeros() as usize;
        seeds &= seeds - 1;

        let mut s = ancestor_incl[i] & remaining;
        let (mut f, mut v) = totals(s, fee_of, vsize_of);

        // Greedy extension to fixed point: pick the ancestor-closure
        // addition that yields the highest resulting feerate, if any.
        loop {
            let mut picked: Option<(u128, u64, u64)> = None;
            let mut cands = remaining & !s;
            while cands != 0 {
                let j = cands.trailing_zeros() as usize;
                cands &= cands - 1;
                let add = ancestor_incl[j] & remaining & !s;
                if add == 0 {
                    continue;
                }
                let (df, dv) = totals(add, fee_of, vsize_of);
                let nf = f + df;
                let nv = v + dv;
                // Must strictly improve current rate: nf/nv > f/v.
                if nf as u128 * v as u128 <= f as u128 * nv as u128 {
                    continue;
                }
                match picked {
                    None => picked = Some((add, nf, nv)),
                    Some((_, pf, pv)) => {
                        if nf as u128 * pv as u128 > pf as u128 * nv as u128 {
                            picked = Some((add, nf, nv));
                        }
                    }
                }
            }
            match picked {
                Some((add, nf, nv)) => {
                    s |= add;
                    f = nf;
                    v = nv;
                }
                None => break,
            }
        }

        if f as u128 * best.2 as u128 > best.1 as u128 * v as u128 {
            best = (s, f, v);
        }
    }
    best
}

/// Single-pass stack merge: for each incoming chunk, merge it into
/// the stack top while the merge would raise the top's feerate, then
/// push. O(n) total regardless of how many merges cascade.
fn canonicalize(chunks: &mut Vec<Chunk>) {
    let taken = std::mem::take(chunks);
    let mut out: Vec<Chunk> = Vec::with_capacity(taken.len());
    for mut cur in taken {
        while let Some(top) = out.last() {
            if cur.fee as u128 * top.vsize as u128 > top.fee as u128 * cur.vsize as u128 {
                let mut prev = out.pop().unwrap();
                prev.fee += cur.fee;
                prev.vsize += cur.vsize;
                prev.nodes.extend(cur.nodes);
                cur = prev;
            } else {
                break;
            }
        }
        out.push(cur);
    }
    *chunks = out;
}

#[inline]
fn totals(mask: u128, fee_of: &[u64], vsize_of: &[u64]) -> (u64, u64) {
    let mut f = 0u64;
    let mut v = 0u64;
    let mut bits = mask;
    while bits != 0 {
        let i = bits.trailing_zeros() as usize;
        f += fee_of[i];
        v += vsize_of[i];
        bits &= bits - 1;
    }
    (f, v)
}

fn chunk_of(mask: u128, fee: u64, vsize: u64) -> Chunk {
    let mut nodes: SmallVec<[LocalIdx; 4]> = SmallVec::new();
    let mut bits = mask;
    while bits != 0 {
        let i = bits.trailing_zeros();
        nodes.push(i as LocalIdx);
        bits &= bits - 1;
    }
    Chunk { nodes, fee, vsize }
}
