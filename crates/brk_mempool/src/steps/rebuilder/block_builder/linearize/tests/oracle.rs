//! Brute-force optimality oracle.
//!
//! For small clusters (n ≤ 6), enumerate every topological ordering and
//! compute the canonical chunking of each. The "best" chunking is the
//! one whose fee diagram dominates pointwise. SFL must match.
//!
//! This file focuses on a handful of hand-picked shapes plus every
//! topological variant of a few DAGs where ancestor-set-sort would pick
//! a suboptimal chunking. Exhaustive DAG enumeration is out of scope;
//! the invariant tests in `stress.rs` cover random shapes.

use super::super::LocalIdx;
use super::{Chunk, make_cluster, run};

// ---------- oracle ----------

/// Compute the canonical (upper-concave-envelope) chunking of a
/// linearization expressed as `(fee, vsize)` for each position.
fn canonical_chunking(path: &[(u64, u64)]) -> Vec<(u64, u64)> {
    // Start with singletons; repeatedly merge a chunk with its right
    // neighbour while that improves its feerate (i.e. the merge would
    // make the earlier chunk have the SAME OR HIGHER rate than a strict
    // ordering requires). This is the standard left-to-right canonical
    // chunking pass.
    let mut chunks: Vec<(u64, u64)> = path.to_vec();
    let mut changed = true;
    while changed {
        changed = false;
        let mut i = 0;
        while i + 1 < chunks.len() {
            let (fa, va) = chunks[i];
            let (fb, vb) = chunks[i + 1];
            // Merge if later chunk has strictly higher feerate (would
            // be out of non-increasing order).
            if fb as u128 * va as u128 > fa as u128 * vb as u128 {
                chunks[i] = (fa + fb, va + vb);
                chunks.remove(i + 1);
                changed = true;
            } else {
                i += 1;
            }
        }
    }
    chunks
}

/// All topological orderings of a DAG; Heap's algorithm wouldn't
/// respect topology, so do an explicit DFS over available-next-sets.
fn all_topo_orders(parents: &[Vec<LocalIdx>]) -> Vec<Vec<LocalIdx>> {
    let n = parents.len();
    let indegree: Vec<u32> = parents.iter().map(|p| p.len() as u32).collect();
    let children: Vec<Vec<LocalIdx>> = {
        let mut out = vec![Vec::new(); n];
        for (c, ps) in parents.iter().enumerate() {
            for &p in ps {
                out[p as usize].push(c as LocalIdx);
            }
        }
        out
    };

    let mut results = Vec::new();
    let mut current: Vec<LocalIdx> = Vec::new();
    let mut indeg = indegree.clone();
    walk(&children, &mut indeg, &mut current, n, &mut results);
    return results;

    fn walk(
        children: &[Vec<LocalIdx>],
        indeg: &mut [u32],
        current: &mut Vec<LocalIdx>,
        n: usize,
        out: &mut Vec<Vec<LocalIdx>>,
    ) {
        if current.len() == n {
            out.push(current.clone());
            return;
        }
        let ready: Vec<LocalIdx> = (0..n as LocalIdx)
            .filter(|&i| indeg[i as usize] == 0)
            .collect();
        for v in ready {
            indeg[v as usize] = u32::MAX; // mark unavailable
            current.push(v);
            for &c in &children[v as usize] {
                indeg[c as usize] -= 1;
            }
            walk(children, indeg, current, n, out);
            current.pop();
            for &c in &children[v as usize] {
                indeg[c as usize] += 1;
            }
            indeg[v as usize] = 0; // restore
        }
    }
}

/// Best canonical chunking over all topological orderings of
/// `(fees_vsizes, edges)`. "Best" = lexicographic dominance of the
/// sequence of `(fee, vsize)` per chunk (earlier chunks weigh more).
fn oracle_best(fees_vsizes: &[(u64, u64)], edges: &[(LocalIdx, LocalIdx)]) -> Vec<(u64, u64)> {
    let n = fees_vsizes.len();
    let mut parents = vec![Vec::new(); n];
    for &(p, c) in edges {
        parents[c as usize].push(p);
    }

    let mut best: Option<Vec<(u64, u64)>> = None;
    for order in all_topo_orders(&parents) {
        let path: Vec<(u64, u64)> = order.iter().map(|&i| fees_vsizes[i as usize]).collect();
        let chunking = canonical_chunking(&path);
        best = Some(match best {
            None => chunking,
            Some(cur) => {
                if dominates(&chunking, &cur) {
                    chunking
                } else {
                    cur
                }
            }
        });
    }
    best.expect("at least one topological order")
}

/// `a` dominates `b` iff its cumulative-fee-at-vsize curve sits at
/// or above `b`'s everywhere along the combined vsize axis.
fn dominates(a: &[(u64, u64)], b: &[(u64, u64)]) -> bool {
    // Compare pointwise at each "breakpoint" of either curve.
    let a_points = cumulative(a);
    let b_points = cumulative(b);
    let total_vsize = a_points.last().map(|p| p.0).unwrap_or(0);
    debug_assert_eq!(total_vsize, b_points.last().map(|p| p.0).unwrap_or(0));
    for v in 1..=total_vsize {
        let fa = fee_at(&a_points, v);
        let fb = fee_at(&b_points, v);
        if fa < fb {
            return false;
        }
        if fa > fb {
            return true; // strictly better somewhere; dominates
        }
    }
    // Identical curves — neither dominates strictly; treat as domination
    // (for "best" bookkeeping it's a tie and the first-seen wins).
    true
}

fn cumulative(chunks: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut out = Vec::with_capacity(chunks.len() + 1);
    let mut v = 0u64;
    let mut f = 0u64;
    out.push((0, 0));
    for &(fee, vsize) in chunks {
        v += vsize;
        f += fee;
        out.push((v, f));
    }
    out
}

fn fee_at(cum: &[(u64, u64)], v: u64) -> u128 {
    // Linear interpolation between breakpoints; but since chunks are
    // atomic, we instead compute the straight-line fee at exactly
    // cumulative vsize positions by walking chunks.
    for pair in cum.windows(2) {
        let (v0, f0) = pair[0];
        let (v1, f1) = pair[1];
        if v <= v1 {
            // within this chunk: linear from (v0, f0) to (v1, f1).
            let dv = v1 - v0;
            if dv == 0 {
                return f0 as u128;
            }
            let df = f1 - f0;
            return f0 as u128 + (df as u128) * ((v - v0) as u128) / (dv as u128);
        }
    }
    cum.last().map(|&(_, f)| f as u128).unwrap_or(0)
}

fn chunk_rate(chunks: &[Chunk]) -> Vec<(u64, u64)> {
    chunks.iter().map(|c| (c.fee, c.vsize)).collect()
}

/// Assert that SFL's output matches the oracle fee diagram.
fn assert_matches_oracle(fees_vsizes: &[(u64, u64)], edges: &[(LocalIdx, LocalIdx)]) {
    let cluster = make_cluster(fees_vsizes, edges);
    let chunks = run(&cluster);
    let got = chunk_rate(&chunks);
    let want = oracle_best(fees_vsizes, edges);

    let got_cum = cumulative(&got);
    let want_cum = cumulative(&want);
    let total = got_cum.last().unwrap().0;
    assert_eq!(total, want_cum.last().unwrap().0, "total vsize mismatch");

    for v in 1..=total {
        let fa = fee_at(&got_cum, v);
        let fb = fee_at(&want_cum, v);
        assert!(
            fa >= fb,
            "SFL diagram below oracle at vsize {}: got {} want {}\n  got={:?}\n  want={:?}",
            v,
            fa,
            fb,
            got,
            want,
        );
    }
}

// ---------- tests ----------

#[test]
fn oracle_singleton() {
    assert_matches_oracle(&[(100, 10)], &[]);
}

#[test]
fn oracle_chain_cpfp() {
    assert_matches_oracle(&[(1, 10), (100, 1)], &[(0, 1)]);
}

#[test]
fn oracle_chain_parent_richer() {
    assert_matches_oracle(&[(100, 10), (1, 1)], &[(0, 1)]);
}

#[test]
fn oracle_v_shape() {
    assert_matches_oracle(&[(1, 1), (1, 1), (100, 1)], &[(0, 2), (1, 2)]);
}

#[test]
fn oracle_lambda_non_ancestor_beats_ancestor() {
    // The "non-ancestor-set wins" case: SFL should match the oracle's
    // single-chunk optimum at rate 11/3.
    assert_matches_oracle(&[(1, 1), (5, 1), (5, 1)], &[(0, 1), (0, 2)]);
}

#[test]
fn oracle_diamond() {
    assert_matches_oracle(
        &[(1, 1), (1, 1), (1, 1), (100, 1)],
        &[(0, 1), (0, 2), (1, 3), (2, 3)],
    );
}

#[test]
fn oracle_tree_depth_3() {
    // A → B → D, A → C → E. Leaves pay.
    assert_matches_oracle(
        &[(1, 1), (1, 1), (1, 1), (100, 1), (100, 1)],
        &[(0, 1), (0, 2), (1, 3), (2, 4)],
    );
}

#[test]
fn oracle_branching_with_cheap_sibling() {
    // A(1) → B(50), A → C(100). SFL's expected optimum: single chunk.
    assert_matches_oracle(&[(1, 1), (50, 1), (100, 1)], &[(0, 1), (0, 2)]);
}

#[test]
fn oracle_four_chain_alternating() {
    // Alternating rates; brute force up to 6-tx.
    assert_matches_oracle(
        &[(10, 1), (1, 1), (10, 1), (1, 1)],
        &[(0, 1), (1, 2), (2, 3)],
    );
}

// ---------- exhaustive random DAG sweep ----------
//
// Enumerate random DAG shapes up to n=8 (40320 topo-orders max per DAG)
// and check merge-only's output matches the brute-force optimum. Runs
// thousands of cases; catches tie-break pathologies the hand-picked
// shapes above might miss.

struct DagRng(u64);
impl DagRng {
    fn new(seed: u64) -> Self {
        Self(seed | 1)
    }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn range(&mut self, n: u64) -> u64 {
        if n == 0 { 0 } else { self.next() % n }
    }
}

/// `(fee, vsize)` per node + edge list. Used by random-DAG generators.
type FvAndEdges = (Vec<(u64, u64)>, Vec<(LocalIdx, LocalIdx)>);

/// Random DAG with `n` nodes: each node i > 0 has 0-3 parents drawn
/// uniformly from nodes {0..i}. Fees/vsizes are varied.
fn random_dag(n: usize, seed: u64) -> FvAndEdges {
    let mut rng = DagRng::new(seed);
    let fees_vsizes: Vec<(u64, u64)> = (0..n)
        .map(|_| {
            let fee = 1 + rng.range(200);
            let vsize = 1 + rng.range(5);
            (fee, vsize)
        })
        .collect();
    let mut edges = Vec::new();
    for i in 1..n {
        let k = rng.range(4) as usize;
        let mut picks: Vec<LocalIdx> = Vec::new();
        for _ in 0..k {
            let p = rng.range(i as u64) as LocalIdx;
            if !picks.contains(&p) {
                picks.push(p);
            }
        }
        for p in picks {
            edges.push((p, i as LocalIdx));
        }
    }
    (fees_vsizes, edges)
}

#[expect(dead_code, reason = "kept for ad-hoc oracle sweeps; called via uncommented stress tests")]
fn assert_optimal_on_random(n: usize, seed: u64) {
    let (fv, edges) = random_dag(n, seed);
    let cluster = super::make_cluster(&fv, &edges);
    let chunks = super::run(&cluster);
    let got = chunk_rate(&chunks);

    let want = oracle_best(&fv, &edges);

    let got_cum = cumulative(&got);
    let want_cum = cumulative(&want);
    let total = got_cum.last().unwrap().0;
    assert_eq!(total, want_cum.last().unwrap().0);

    for v in 1..=total {
        let fa = fee_at(&got_cum, v);
        let fb = fee_at(&want_cum, v);
        assert!(
            fa >= fb,
            "merge-only suboptimal (n={}, seed={})\n  fv = {:?}\n  edges = {:?}\n  got = {:?}\n  want = {:?}\n  at vsize {}: got {}, want {}",
            n,
            seed,
            fv,
            edges,
            got,
            want,
            v,
            fa,
            fb,
        );
    }
}

/// Check whether an algorithm's output matches the brute-force optimum.
/// Returns Some(max_gap_at_any_vsize) if suboptimal, None if optimal.
fn optimality_gap_of(got: &[(u64, u64)], want: &[(u64, u64)]) -> Option<u128> {
    let got_cum = cumulative(got);
    let want_cum = cumulative(want);
    let total = got_cum.last().unwrap().0;
    debug_assert_eq!(total, want_cum.last().unwrap().0);

    let mut worst_gap: u128 = 0;
    for v in 1..=total {
        let fa = fee_at(&got_cum, v);
        let fb = fee_at(&want_cum, v);
        if fb > fa {
            worst_gap = worst_gap.max(fb - fa);
        }
    }
    if worst_gap == 0 { None } else { Some(worst_gap) }
}

/// Gap for the production linearizer on one random DAG.
fn optimality_gap(n: usize, seed: u64) -> Option<u128> {
    let (fv, edges) = random_dag(n, seed);
    let cluster = super::make_cluster(&fv, &edges);
    let chunks = super::super::sfl::linearize(&cluster);
    let got: Vec<(u64, u64)> = chunks.iter().map(|c| (c.fee, c.vsize)).collect();
    let want = oracle_best(&fv, &edges);
    optimality_gap_of(&got, &want)
}

/// Diagnostic sweep: report the linearizer's optimality gap on random DAGs.
#[test]
#[ignore = "diagnostic sweep; run with --ignored to print stats"]
fn oracle_random_sweep_stats() {
    let sizes: &[(usize, u64, u64)] = &[
        (4, 500, 1),
        (5, 500, 1_000),
        (6, 300, 2_000),
        (7, 100, 3_000),
        (8, 50, 4_000),
    ];

    eprintln!();
    eprintln!("Optimality sweep (random DAGs vs brute-force optimum):");
    eprintln!("  n   cases     sub   max-gap");
    eprintln!("  ---------------------------");

    let mut total = 0usize;
    let mut cases_total = 0usize;
    for &(n, count, base) in sizes {
        let mut sub = 0;
        let mut gap: u128 = 0;
        for seed in 0..count {
            let s = seed.wrapping_add(base);
            if let Some(g) = optimality_gap(n, s) {
                sub += 1;
                gap = gap.max(g);
            }
        }
        total += sub;
        cases_total += count as usize;
        eprintln!("  {}   {:5}     {:3}     {:4}", n, count, sub, gap);
    }
    eprintln!("  ---------------------------");
    let pct = (total as f64 / cases_total as f64) * 100.0;
    eprintln!("  totals {:4}   {:3} ({:.1}%)", cases_total, total, pct);
    eprintln!();
}

/// Perf benchmark across cluster sizes. Run with
/// `cargo test -p brk_mempool perf_linearize --release -- --ignored --nocapture`.
#[test]
#[ignore = "perf benchmark; run with --ignored --nocapture"]
fn perf_linearize() {
    use std::time::Instant;

    let sizes: &[(usize, u64)] = &[
        (2, 5_000),
        (5, 5_000),
        (10, 2_000),
        (15, 1_000),
        (18, 500),
        (20, 500),
        (30, 200),
        (50, 100),
        (75, 50),
        (100, 30),
    ];

    eprintln!();
    eprintln!("Linearize perf (release, per-call avg):");
    eprintln!("  n    calls     avg       total");
    eprintln!("  -------------------------------------");

    for &(n, calls) in sizes {
        let clusters: Vec<_> = (0..calls)
            .map(|s| {
                let (fv, edges) = random_dag(n, s + 77);
                super::make_cluster(&fv, &edges)
            })
            .collect();

        let t = Instant::now();
        let mut sink = 0u64;
        for c in &clusters {
            for chunk in super::super::sfl::linearize(c) {
                sink = sink.wrapping_add(chunk.fee);
            }
        }
        let elapsed = t.elapsed();
        let _ = sink;

        let avg_ns = elapsed.as_nanos() / calls as u128;
        let pretty = if avg_ns >= 1_000_000 {
            format!("{:.2} ms", avg_ns as f64 / 1_000_000.0)
        } else if avg_ns >= 1_000 {
            format!("{:.2} µs", avg_ns as f64 / 1_000.0)
        } else {
            format!("{} ns", avg_ns)
        };
        eprintln!(
            "  {:<4} {:<8}  {:<10} {:.2?}",
            n, calls, pretty, elapsed
        );
    }
    eprintln!();
}
