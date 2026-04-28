use brk_types::{Sats, VSize};

use super::{Chunk, chunk_shapes, make_cluster, run};

#[test]
fn singleton() {
    let cluster = make_cluster(&[(100, 10)], &[]);
    let chunks = run(&cluster);
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].nodes.len(), 1);
    assert_eq!(chunks[0].fee, Sats::from(100u64));
    assert_eq!(chunks[0].vsize, VSize::from(10u64));
}

#[test]
fn two_chain_parent_richer() {
    let cluster = make_cluster(&[(100, 10), (1, 1)], &[(0, 1)]);
    let chunks = run(&cluster);
    assert_eq!(chunks.len(), 2);
    assert!(chunks[0].nodes.contains(&0));
    assert_eq!(chunks[0].vsize, VSize::from(10u64));
    assert!(chunks[1].nodes.contains(&1));
    assert_eq!(chunks[1].vsize, VSize::from(1u64));
}

#[test]
fn two_chain_child_pays_parent_cpfp() {
    let cluster = make_cluster(&[(1, 10), (100, 1)], &[(0, 1)]);
    let chunks = run(&cluster);
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].nodes.len(), 2);
    assert_eq!(chunks[0].fee, Sats::from(101u64));
    assert_eq!(chunks[0].vsize, VSize::from(11u64));
}

#[test]
fn v_shape_two_parents_one_child() {
    let cluster = make_cluster(&[(1, 1), (1, 1), (100, 1)], &[(0, 2), (1, 2)]);
    let chunks = run(&cluster);
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].nodes.len(), 3);
    assert_eq!(chunks[0].fee, Sats::from(102u64));
    assert_eq!(chunks[0].vsize, VSize::from(3u64));
}

#[test]
fn lambda_shape_one_parent_two_children_uneven() {
    let cluster = make_cluster(&[(1, 1), (5, 1), (5, 1)], &[(0, 1), (0, 2)]);
    let chunks = run(&cluster);
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].fee, Sats::from(11u64));
    assert_eq!(chunks[0].vsize, VSize::from(3u64));
}

#[test]
fn diamond() {
    let cluster = make_cluster(
        &[(1, 1), (1, 1), (1, 1), (100, 1)],
        &[(0, 1), (0, 2), (1, 3), (2, 3)],
    );
    let chunks = run(&cluster);
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].nodes.len(), 4);
    assert_eq!(chunks[0].fee, Sats::from(103u64));
    assert_eq!(chunks[0].vsize, VSize::from(4u64));
}

#[test]
fn chain_alternating_high_low() {
    let cluster = make_cluster(
        &[(10, 1), (1, 1), (10, 1), (1, 1)],
        &[(0, 1), (1, 2), (2, 3)],
    );
    let chunks = run(&cluster);
    assert_eq!(chunks_total_fee(&chunks), Sats::from(22u64));
    assert_eq!(chunks_total_vsize(&chunks), VSize::from(4u64));
    assert_non_increasing(&chunks);
}

#[test]
fn chain_starts_low_ends_high() {
    let cluster = make_cluster(
        &[(1, 1), (100, 1), (1, 1), (100, 1)],
        &[(0, 1), (1, 2), (2, 3)],
    );
    let chunks = run(&cluster);
    assert_eq!(chunks_total_fee(&chunks), Sats::from(202u64));
    assert_eq!(chunks_total_vsize(&chunks), VSize::from(4u64));
    assert_non_increasing(&chunks);
}

#[test]
fn two_disconnected_clusters_would_each_be_separate() {
    let cluster = make_cluster(
        &[(1, 1), (10, 1), (20, 1), (30, 1), (40, 1), (50, 1)],
        &[(0, 1), (0, 2), (0, 3), (0, 4), (0, 5)],
    );
    let chunks = run(&cluster);
    assert_eq!(chunks_total_fee(&chunks), Sats::from(151u64));
    assert_eq!(chunks_total_vsize(&chunks), VSize::from(6u64));
    assert_non_increasing(&chunks);
    let mut seen: Vec<usize> = Vec::new();
    for ch in &chunks {
        for &n in &ch.nodes {
            seen.push(n as usize);
        }
    }
    seen.sort_unstable();
    assert_eq!(seen, vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn wide_fan_in() {
    let cluster = make_cluster(
        &[(1, 1), (1, 1), (1, 1), (1, 1), (1, 1), (100, 1)],
        &[(0, 5), (1, 5), (2, 5), (3, 5), (4, 5)],
    );
    let chunks = run(&cluster);
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].fee, Sats::from(105u64));
    assert_eq!(chunks[0].vsize, VSize::from(6u64));
}

#[test]
fn shapes_are_stable_on_identical_input() {
    let cluster = make_cluster(
        &[(1, 1), (100, 1), (1, 1), (100, 1)],
        &[(0, 1), (1, 2), (2, 3)],
    );
    let a = chunk_shapes(&run(&cluster));
    let b = chunk_shapes(&run(&cluster));
    assert_eq!(a, b);
}

fn chunks_total_fee(chunks: &[Chunk]) -> Sats {
    chunks.iter().map(|c| c.fee).sum()
}

fn chunks_total_vsize(chunks: &[Chunk]) -> VSize {
    chunks.iter().map(|c| c.vsize).sum()
}

fn assert_non_increasing(chunks: &[Chunk]) {
    for pair in chunks.windows(2) {
        assert!(
            pair[0].fee_rate() >= pair[1].fee_rate(),
            "chunk feerates not non-increasing: {:?} vs {:?}",
            (pair[0].fee, pair[0].vsize),
            (pair[1].fee, pair[1].vsize),
        );
    }
}
