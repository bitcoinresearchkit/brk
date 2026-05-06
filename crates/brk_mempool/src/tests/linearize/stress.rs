use brk_types::{Sats, VSize};

use super::{TestCluster, make_cluster, run};

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

type FvAndEdges = (Vec<(u64, u64)>, Vec<(u32, u32)>);

fn random_cluster(n: usize, seed: u64) -> FvAndEdges {
    let mut rng = Rng::new(seed);
    let mut fees_vsizes = Vec::with_capacity(n);
    for _ in 0..n {
        let fee = 1 + rng.range(1000);
        let vsize = 1 + rng.range(100);
        fees_vsizes.push((fee, vsize));
    }

    let mut edges = Vec::new();
    for i in 1..n {
        let k = rng.range(4) as usize;
        let mut picks: Vec<u32> = Vec::new();
        for _ in 0..k {
            let p = rng.range(i as u64) as u32;
            if !picks.contains(&p) {
                picks.push(p);
            }
        }
        for p in picks {
            edges.push((p, i as u32));
        }
    }

    (fees_vsizes, edges)
}

/// `cluster.nodes` is in topological order, so each node's `LocalIdx`
/// may differ from the caller's input position. The cluster's `id`
/// field carries the input index, and we use it to map back when the
/// invariant being checked is expressed in input space (fees/vsizes
/// table, edges list).
fn check_invariants(fees_vsizes: &[(u64, u64)], edges: &[(u32, u32)], cluster: &TestCluster) {
    let n = fees_vsizes.len();
    let chunks = &cluster.chunks;
    let input_of = |l: crate::cluster::LocalIdx| cluster.nodes[l.as_usize()].id as usize;

    let mut seen = vec![false; n];
    for chunk in chunks {
        for &local in &chunk.txs {
            let i = input_of(local);
            assert!(!seen[i], "input node {} appears in multiple chunks", i);
            seen[i] = true;
        }
    }
    for (i, s) in seen.iter().enumerate() {
        assert!(*s, "input node {} missing from all chunks", i);
    }

    for chunk in chunks {
        let fee: u64 = chunk.txs.iter().map(|&l| fees_vsizes[input_of(l)].0).sum();
        let vsize: u64 = chunk.txs.iter().map(|&l| fees_vsizes[input_of(l)].1).sum();
        assert_eq!(chunk.fee, Sats::from(fee), "chunk fee mismatch");
        assert_eq!(chunk.vsize, VSize::from(vsize), "chunk vsize mismatch");
    }

    let chunk_of_input: Vec<usize> = {
        let mut out = vec![usize::MAX; n];
        for (ci, chunk) in chunks.iter().enumerate() {
            for &local in &chunk.txs {
                out[input_of(local)] = ci;
            }
        }
        out
    };
    for &(p, c) in edges {
        let cp = chunk_of_input[p as usize];
        let cc = chunk_of_input[c as usize];
        assert!(
            cp <= cc,
            "parent {} in chunk {} but child {} in earlier chunk {}",
            p,
            cp,
            c,
            cc
        );
    }

    for pair in chunks.windows(2) {
        assert!(
            pair[0].fee_rate() >= pair[1].fee_rate(),
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
        check_invariants(&fv, &edges, &cluster);
    }
}

#[test]
fn random_medium_clusters() {
    for seed in 0..50u64 {
        let n = 10 + (seed % 20) as usize;
        let (fv, edges) = random_cluster(n, seed.wrapping_add(100));
        let cluster = make_cluster(&fv, &edges);
        check_invariants(&fv, &edges, &cluster);
    }
}

#[test]
fn random_large_clusters() {
    for seed in 0..10u64 {
        let (fv, edges) = random_cluster(30, seed.wrapping_add(1000));
        let cluster = make_cluster(&fv, &edges);
        check_invariants(&fv, &edges, &cluster);
    }
}

#[test]
fn determinism_same_seed_same_output() {
    let (fv, edges) = random_cluster(15, 42);
    let cluster = make_cluster(&fv, &edges);
    let a: Vec<(Sats, VSize)> = run(&cluster).iter().map(|c| (c.fee, c.vsize)).collect();
    let b: Vec<(Sats, VSize)> = run(&cluster).iter().map(|c| (c.fee, c.vsize)).collect();
    assert_eq!(a, b);
}

#[test]
fn random_cluster_at_policy_limit() {
    for seed in 0..5u64 {
        let (fv, edges) = random_cluster(100, seed.wrapping_add(9000));
        let cluster = make_cluster(&fv, &edges);
        check_invariants(&fv, &edges, &cluster);
    }
}
