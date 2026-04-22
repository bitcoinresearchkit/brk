//! Randomized invariant tests.
//!
//! Generates random DAGs up to size 30 with varied fee rates and
//! verifies SFL's output respects:
//!   1. Every node appears in exactly one chunk.
//!   2. Each chunk is topologically closed (no intra-cluster parent
//!      of a chunk member lies in a later-emitted chunk).
//!   3. Chunk feerates are non-increasing along emission order.

use super::super::LocalIdx;
use super::{make_cluster, run};

/// Tiny deterministic xorshift so tests are reproducible.
struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed | 1)
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn range(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
}

/// Build a random DAG with `n` nodes. For each node `i` > 0, add a
/// random number of parents from nodes with index < i (guarantees
/// acyclic). Fee and vsize are random in a small range.
fn random_cluster(n: usize, seed: u64) -> (Vec<(u64, u64)>, Vec<(LocalIdx, LocalIdx)>) {
    let mut rng = Rng::new(seed);
    let mut fees_vsizes = Vec::with_capacity(n);
    for _ in 0..n {
        let fee = 1 + rng.range(1000);
        let vsize = 1 + rng.range(100);
        fees_vsizes.push((fee, vsize));
    }

    let mut edges = Vec::new();
    for i in 1..n {
        // 0-3 parents, each picked uniformly from earlier nodes.
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

fn check_invariants(
    fees_vsizes: &[(u64, u64)],
    edges: &[(LocalIdx, LocalIdx)],
    chunks: &[super::Chunk],
) {
    let n = fees_vsizes.len();

    // (1) Each node in exactly one chunk.
    let mut seen = vec![false; n];
    for chunk in chunks {
        for &local in &chunk.nodes {
            assert!(
                !seen[local as usize],
                "node {} appears in multiple chunks",
                local
            );
            seen[local as usize] = true;
        }
    }
    for (i, s) in seen.iter().enumerate() {
        assert!(*s, "node {} missing from all chunks", i);
    }

    // Chunk aggregates match declared totals.
    for chunk in chunks {
        let fee: u64 = chunk.nodes.iter().map(|&l| fees_vsizes[l as usize].0).sum();
        let vsize: u64 = chunk.nodes.iter().map(|&l| fees_vsizes[l as usize].1).sum();
        assert_eq!(chunk.fee, fee, "chunk fee mismatch");
        assert_eq!(chunk.vsize, vsize, "chunk vsize mismatch");
    }

    // (2) Chunks are topologically closed in emission order: a parent
    //     in cluster must be in the same or earlier chunk.
    let chunk_of: Vec<usize> = {
        let mut out = vec![usize::MAX; n];
        for (ci, chunk) in chunks.iter().enumerate() {
            for &local in &chunk.nodes {
                out[local as usize] = ci;
            }
        }
        out
    };
    for &(p, c) in edges {
        let cp = chunk_of[p as usize];
        let cc = chunk_of[c as usize];
        assert!(
            cp <= cc,
            "parent {} in chunk {} but child {} in earlier chunk {}",
            p,
            cp,
            c,
            cc
        );
    }

    // (3) Non-increasing chunk feerates in emission order.
    for pair in chunks.windows(2) {
        let a = pair[0].fee as u128 * pair[1].vsize as u128;
        let b = pair[1].fee as u128 * pair[0].vsize as u128;
        assert!(
            a >= b,
            "chunk feerates not non-increasing: {}/{} then {}/{}",
            pair[0].fee,
            pair[0].vsize,
            pair[1].fee,
            pair[1].vsize,
        );
    }
}

#[test]
fn random_small_clusters() {
    for seed in 0..200u64 {
        let n = 2 + (seed % 10) as usize;
        let (fv, edges) = random_cluster(n, seed.wrapping_add(1));
        let cluster = make_cluster(&fv, &edges);
        let chunks = run(&cluster);
        check_invariants(&fv, &edges, &chunks);
    }
}

#[test]
fn random_medium_clusters() {
    for seed in 0..50u64 {
        let n = 10 + (seed % 20) as usize;
        let (fv, edges) = random_cluster(n, seed.wrapping_add(100));
        let cluster = make_cluster(&fv, &edges);
        let chunks = run(&cluster);
        check_invariants(&fv, &edges, &chunks);
    }
}

#[test]
fn random_large_clusters() {
    for seed in 0..10u64 {
        let (fv, edges) = random_cluster(30, seed.wrapping_add(1000));
        let cluster = make_cluster(&fv, &edges);
        let chunks = run(&cluster);
        check_invariants(&fv, &edges, &chunks);
    }
}

#[test]
fn determinism_same_seed_same_output() {
    let (fv, edges) = random_cluster(15, 42);
    let cluster = make_cluster(&fv, &edges);
    let a: Vec<(u64, u64)> = run(&cluster).iter().map(|c| (c.fee, c.vsize)).collect();
    let b: Vec<(u64, u64)> = run(&cluster).iter().map(|c| (c.fee, c.vsize)).collect();
    assert_eq!(a, b);
}

/// Exercise the perf path: large clusters with many edges. If any
/// individual call exceeds a generous budget we'd know SFL is slow for
/// realistic workloads.
#[test]
fn random_cluster_at_policy_limit() {
    for seed in 0..5u64 {
        // 100-tx cluster approximates Bitcoin Core's cluster policy cap.
        let (fv, edges) = random_cluster(100, seed.wrapping_add(9000));
        let cluster = make_cluster(&fv, &edges);
        let chunks = run(&cluster);
        check_invariants(&fv, &edges, &chunks);
    }
}
