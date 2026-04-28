use brk_types::{FeeRate, Sats, VSize};

use super::{Chunk, LocalIdx, Sfl, make_cluster, run};

fn to_typed(fv: &[(u64, u64)]) -> Vec<(Sats, VSize)> {
    fv.iter()
        .map(|&(f, v)| (Sats::from(f), VSize::from(v)))
        .collect()
}

fn canonical_chunking(path: &[(Sats, VSize)]) -> Vec<(Sats, VSize)> {
    let mut chunks: Vec<(Sats, VSize)> = path.to_vec();
    let mut changed = true;
    while changed {
        changed = false;
        let mut i = 0;
        while i + 1 < chunks.len() {
            let (fa, va) = chunks[i];
            let (fb, vb) = chunks[i + 1];
            if FeeRate::from((fb, vb)) > FeeRate::from((fa, va)) {
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
            indeg[v as usize] = u32::MAX;
            current.push(v);
            for &c in &children[v as usize] {
                indeg[c as usize] -= 1;
            }
            walk(children, indeg, current, n, out);
            current.pop();
            for &c in &children[v as usize] {
                indeg[c as usize] += 1;
            }
            indeg[v as usize] = 0;
        }
    }
}

fn oracle_best(
    fees_vsizes: &[(Sats, VSize)],
    edges: &[(LocalIdx, LocalIdx)],
) -> Vec<(Sats, VSize)> {
    let n = fees_vsizes.len();
    let mut parents = vec![Vec::new(); n];
    for &(p, c) in edges {
        parents[c as usize].push(p);
    }

    let mut best: Option<Vec<(Sats, VSize)>> = None;
    for order in all_topo_orders(&parents) {
        let path: Vec<(Sats, VSize)> = order.iter().map(|&i| fees_vsizes[i as usize]).collect();
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

fn dominates(a: &[(Sats, VSize)], b: &[(Sats, VSize)]) -> bool {
    let a_points = cumulative(a);
    let b_points = cumulative(b);
    let total_vsize = a_points.last().map(|p| p.0).unwrap_or_default();
    debug_assert_eq!(
        total_vsize,
        b_points.last().map(|p| p.0).unwrap_or_default()
    );
    for v in 1..=u64::from(total_vsize) {
        let v = VSize::from(v);
        let fa = fee_at(&a_points, v);
        let fb = fee_at(&b_points, v);
        if fa < fb {
            return false;
        }
        if fa > fb {
            return true;
        }
    }
    true
}

fn cumulative(chunks: &[(Sats, VSize)]) -> Vec<(VSize, Sats)> {
    let mut out = Vec::with_capacity(chunks.len() + 1);
    let mut v = VSize::default();
    let mut f = Sats::ZERO;
    out.push((v, f));
    for &(fee, vsize) in chunks {
        v += vsize;
        f += fee;
        out.push((v, f));
    }
    out
}

/// Linear interpolation of cumulative fee at vsize `v`. Returns a
/// scaled `u128` (sub-sat precision via `df * dx / dv`) so dominance
/// ties resolve at the bit level.
fn fee_at(cum: &[(VSize, Sats)], v: VSize) -> u128 {
    for pair in cum.windows(2) {
        let (v0, f0) = pair[0];
        let (v1, f1) = pair[1];
        if v <= v1 {
            let dv = u64::from(v1 - v0) as u128;
            let f0 = u64::from(f0) as u128;
            if dv == 0 {
                return f0;
            }
            let df = u64::from(f1) as u128 - f0;
            let dx = u64::from(v - v0) as u128;
            return f0 + df * dx / dv;
        }
    }
    cum.last().map_or(0, |&(_, f)| u64::from(f) as u128)
}

fn chunk_rate(chunks: &[Chunk]) -> Vec<(Sats, VSize)> {
    chunks.iter().map(|c| (c.fee, c.vsize)).collect()
}

fn assert_matches_oracle(fees_vsizes: &[(u64, u64)], edges: &[(LocalIdx, LocalIdx)]) {
    let cluster = make_cluster(fees_vsizes, edges);
    let chunks = run(&cluster);
    let got = chunk_rate(&chunks);
    let want = oracle_best(&to_typed(fees_vsizes), edges);

    let got_cum = cumulative(&got);
    let want_cum = cumulative(&want);
    let total = got_cum.last().unwrap().0;
    assert_eq!(total, want_cum.last().unwrap().0, "total vsize mismatch");

    for v in 1..=u64::from(total) {
        let v = VSize::from(v);
        let fa = fee_at(&got_cum, v);
        let fb = fee_at(&want_cum, v);
        assert!(
            fa >= fb,
            "SFL diagram below oracle at vsize {:?}: got {} want {}\n  got={:?}\n  want={:?}",
            v,
            fa,
            fb,
            got,
            want,
        );
    }
}

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
    assert_matches_oracle(
        &[(1, 1), (1, 1), (1, 1), (100, 1), (100, 1)],
        &[(0, 1), (0, 2), (1, 3), (2, 4)],
    );
}

#[test]
fn oracle_branching_with_cheap_sibling() {
    assert_matches_oracle(&[(1, 1), (50, 1), (100, 1)], &[(0, 1), (0, 2)]);
}

#[test]
fn oracle_four_chain_alternating() {
    assert_matches_oracle(
        &[(10, 1), (1, 1), (10, 1), (1, 1)],
        &[(0, 1), (1, 2), (2, 3)],
    );
}

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

type FvAndEdges = (Vec<(u64, u64)>, Vec<(LocalIdx, LocalIdx)>);

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

#[expect(
    dead_code,
    reason = "kept for ad-hoc oracle sweeps; called via uncommented stress tests"
)]
fn assert_optimal_on_random(n: usize, seed: u64) {
    let (fv, edges) = random_dag(n, seed);
    let cluster = make_cluster(&fv, &edges);
    let chunks = run(&cluster);
    let got = chunk_rate(&chunks);

    let want = oracle_best(&to_typed(&fv), &edges);

    let got_cum = cumulative(&got);
    let want_cum = cumulative(&want);
    let total = got_cum.last().unwrap().0;
    assert_eq!(total, want_cum.last().unwrap().0);

    for v in 1..=u64::from(total) {
        let v = VSize::from(v);
        let fa = fee_at(&got_cum, v);
        let fb = fee_at(&want_cum, v);
        assert!(
            fa >= fb,
            "merge-only suboptimal (n={}, seed={})\n  fv = {:?}\n  edges = {:?}\n  got = {:?}\n  want = {:?}\n  at vsize {:?}: got {}, want {}",
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

fn optimality_gap_of(got: &[(Sats, VSize)], want: &[(Sats, VSize)]) -> Option<u128> {
    let got_cum = cumulative(got);
    let want_cum = cumulative(want);
    let total = got_cum.last().unwrap().0;
    debug_assert_eq!(total, want_cum.last().unwrap().0);

    let mut worst_gap: u128 = 0;
    for v in 1..=u64::from(total) {
        let v = VSize::from(v);
        let fa = fee_at(&got_cum, v);
        let fb = fee_at(&want_cum, v);
        if fb > fa {
            worst_gap = worst_gap.max(fb - fa);
        }
    }
    if worst_gap == 0 {
        None
    } else {
        Some(worst_gap)
    }
}

fn optimality_gap(n: usize, seed: u64) -> Option<u128> {
    let (fv, edges) = random_dag(n, seed);
    let cluster = make_cluster(&fv, &edges);
    let chunks = Sfl::linearize(&cluster);
    let got: Vec<(Sats, VSize)> = chunks.iter().map(|c| (c.fee, c.vsize)).collect();
    let want = oracle_best(&to_typed(&fv), &edges);
    optimality_gap_of(&got, &want)
}

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
                make_cluster(&fv, &edges)
            })
            .collect();

        let t = Instant::now();
        let mut sink = 0u64;
        for c in &clusters {
            for chunk in Sfl::linearize(c) {
                sink = sink.wrapping_add(u64::from(chunk.fee));
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
        eprintln!("  {:<4} {:<8}  {:<10} {:.2?}", n, calls, pretty, elapsed);
    }
    eprintln!();
}
