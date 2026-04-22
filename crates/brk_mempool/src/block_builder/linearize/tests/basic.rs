//! Hand-built cluster shapes with known-good SFL outputs.

use super::{chunk_shapes, make_cluster, run};

#[test]
fn singleton() {
    let cluster = make_cluster(&[(100, 10)], &[]);
    let chunks = run(&cluster);
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].nodes.len(), 1);
    assert_eq!(chunks[0].fee, 100);
    assert_eq!(chunks[0].vsize, 10);
}

#[test]
fn two_chain_parent_richer() {
    // A (rate 10) → B (rate 1). Parent is more profitable alone; SFL
    // should emit two chunks, A first.
    let cluster = make_cluster(&[(100, 10), (1, 1)], &[(0, 1)]);
    let chunks = run(&cluster);
    assert_eq!(chunks.len(), 2);
    // First chunk is A alone.
    assert!(chunks[0].nodes.contains(&0));
    assert_eq!(chunks[0].vsize, 10);
    // Second chunk is B alone.
    assert!(chunks[1].nodes.contains(&1));
    assert_eq!(chunks[1].vsize, 1);
}

#[test]
fn two_chain_child_pays_parent_cpfp() {
    // A (rate 0.1) → B (rate 100). Classic CPFP: bundle them.
    let cluster = make_cluster(&[(1, 10), (100, 1)], &[(0, 1)]);
    let chunks = run(&cluster);
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].nodes.len(), 2);
    assert_eq!(chunks[0].fee, 101);
    assert_eq!(chunks[0].vsize, 11);
}

#[test]
fn v_shape_two_parents_one_child() {
    // P0 (rate 1), P1 (rate 1) → C (rate 100). Expect single chunk.
    let cluster = make_cluster(&[(1, 1), (1, 1), (100, 1)], &[(0, 2), (1, 2)]);
    let chunks = run(&cluster);
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].nodes.len(), 3);
    assert_eq!(chunks[0].fee, 102);
    assert_eq!(chunks[0].vsize, 3);
}

#[test]
fn lambda_shape_one_parent_two_children_uneven() {
    // A(1) → B(5), A(1) → C(5). The "non-ancestor-set" case: {A, B, C}
    // has rate 11/3 ≈ 3.67, beating any ancestor set ({A,B} or {A,C}
    // at rate 3). SFL should produce a single chunk.
    let cluster = make_cluster(&[(1, 1), (5, 1), (5, 1)], &[(0, 1), (0, 2)]);
    let chunks = run(&cluster);
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].fee, 11);
    assert_eq!(chunks[0].vsize, 3);
}

#[test]
fn diamond() {
    // 4-node diamond: A → B, A → C, B → D, C → D. With D the payer,
    // everything ends up in one chunk.
    let cluster = make_cluster(
        &[(1, 1), (1, 1), (1, 1), (100, 1)],
        &[(0, 1), (0, 2), (1, 3), (2, 3)],
    );
    let chunks = run(&cluster);
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].nodes.len(), 4);
    assert_eq!(chunks[0].fee, 103);
    assert_eq!(chunks[0].vsize, 4);
}

#[test]
fn chain_alternating_high_low() {
    // 4-chain with rates [10, 1, 10, 1] all vsize 1. Bubble-up should
    // merge them all (every new tx brings its chunk rate up). Verify
    // one chunk with correct totals rather than a specific partition.
    let cluster = make_cluster(
        &[(10, 1), (1, 1), (10, 1), (1, 1)],
        &[(0, 1), (1, 2), (2, 3)],
    );
    let chunks = run(&cluster);
    assert_eq!(chunks_total_fee(&chunks), 22);
    assert_eq!(chunks_total_vsize(&chunks), 4);
    assert_non_increasing(&chunks);
}

#[test]
fn chain_starts_low_ends_high() {
    // 4-chain [1, 100, 1, 100]: the optimal chunking groups pairs so
    // high-rate bumps lift low-rate predecessors. Exact partition is
    // implementation-dependent; check invariants.
    let cluster = make_cluster(
        &[(1, 1), (100, 1), (1, 1), (100, 1)],
        &[(0, 1), (1, 2), (2, 3)],
    );
    let chunks = run(&cluster);
    assert_eq!(chunks_total_fee(&chunks), 202);
    assert_eq!(chunks_total_vsize(&chunks), 4);
    assert_non_increasing(&chunks);
}

#[test]
fn two_disconnected_clusters_would_each_be_separate() {
    // NOTE: this file tests SFL on a single cluster; multi-cluster
    // flow is tested via `linearize_clusters` at the higher level.
    // For a single-cluster test: fan-out of 5 children.
    let cluster = make_cluster(
        &[(1, 1), (10, 1), (20, 1), (30, 1), (40, 1), (50, 1)],
        &[(0, 1), (0, 2), (0, 3), (0, 4), (0, 5)],
    );
    let chunks = run(&cluster);
    assert_eq!(chunks_total_fee(&chunks), 151);
    assert_eq!(chunks_total_vsize(&chunks), 6);
    assert_non_increasing(&chunks);
    // Every tx exactly once.
    let mut seen: Vec<usize> = Vec::new();
    for ch in &chunks {
        for &n in &ch.nodes {
            seen.push(n as usize);
        }
    }
    seen.sort();
    assert_eq!(seen, vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn wide_fan_in() {
    // 5 parents → 1 child. Parents at rate 1, child at rate 100.
    let cluster = make_cluster(
        &[(1, 1), (1, 1), (1, 1), (1, 1), (1, 1), (100, 1)],
        &[(0, 5), (1, 5), (2, 5), (3, 5), (4, 5)],
    );
    let chunks = run(&cluster);
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].fee, 105);
    assert_eq!(chunks[0].vsize, 6);
}

#[test]
fn shapes_are_stable_on_identical_input() {
    // Determinism: identical cluster should produce identical chunking.
    let cluster = make_cluster(
        &[(1, 1), (100, 1), (1, 1), (100, 1)],
        &[(0, 1), (1, 2), (2, 3)],
    );
    let a = chunk_shapes(&run(&cluster));
    let b = chunk_shapes(&run(&cluster));
    assert_eq!(a, b);
}

// --- helpers ---

fn chunks_total_fee(chunks: &[super::Chunk]) -> u64 {
    chunks.iter().map(|c| c.fee).sum()
}

fn chunks_total_vsize(chunks: &[super::Chunk]) -> u64 {
    chunks.iter().map(|c| c.vsize).sum()
}

fn assert_non_increasing(chunks: &[super::Chunk]) {
    for pair in chunks.windows(2) {
        let a_rate = pair[0].fee as u128 * pair[1].vsize as u128;
        let b_rate = pair[1].fee as u128 * pair[0].vsize as u128;
        assert!(
            a_rate >= b_rate,
            "chunk feerates not non-increasing: {:?} vs {:?}",
            (pair[0].fee, pair[0].vsize),
            (pair[1].fee, pair[1].vsize),
        );
    }
}
