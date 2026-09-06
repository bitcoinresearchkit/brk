use serde_json::to_value;

use super::*;

#[path = "reference.rs"]
mod reference;

fn assert_parity(parents: &[Vec<CpfpClusterTxIndex>], fees: &[u64], sizes: &[u64]) {
    let items: Vec<_> = parents
        .iter()
        .zip(fees)
        .zip(sizes)
        .map(|((parents, &fee), &size)| ChunkInput {
            parents,
            fee: Sats::from(fee),
            vsize: VSize::from(size),
        })
        .collect();
    assert_eq!(
        to_value(linearize(&items)).unwrap(),
        to_value(reference::linearize(&items)).unwrap(),
        "parents={parents:?}, fees={fees:?}, sizes={sizes:?}",
    );
}

#[test]
fn all_five_node_graphs_preserve_chunk_selection() {
    assert!(linearize(&[]).is_empty());
    for graph in 0u32..(1 << 10) {
        let mut edge = 0;
        let parents: Vec<Vec<_>> = (0..5)
            .map(|child| {
                (0..child)
                    .filter_map(|parent| {
                        let present = graph & (1 << edge) != 0;
                        edge += 1;
                        present.then(|| CpfpClusterTxIndex::from(parent))
                    })
                    .collect()
            })
            .collect();
        for fees in [
            [0, 0, 0, 0, 0],
            [100, 100, 100, 100, 100],
            [500, 400, 300, 200, 100],
            [0, 99, 9, 1000, 9999],
        ] {
            assert_parity(&parents, &fees, &[100; 5]);
            assert_parity(&parents, &fees, &[101, 250, 99, 330, 1000]);
        }
    }
}

#[test]
fn larger_graphs_and_shared_ancestors_preserve_results() {
    let mut random = 0x12ab34cdu64;
    let mut next = || {
        random ^= random << 13;
        random ^= random >> 7;
        random ^= random << 17;
        random
    };
    for n in [8usize, 16, 32, 65] {
        for _ in 0..32 {
            let parents: Vec<Vec<_>> = (0..n)
                .map(|child| {
                    (0..child)
                        .filter_map(|parent| {
                            (next() % 5 == 0).then(|| CpfpClusterTxIndex::from(parent as u32))
                        })
                        .collect()
                })
                .collect();
            let fees: Vec<_> = (0..n).map(|_| next() % 10000).collect();
            let sizes: Vec<_> = (0..n).map(|_| 60 + next() % 1000).collect();
            assert_parity(&parents, &fees, &sizes);
        }
    }
}

#[test]
fn block_sized_decreasing_chain_keeps_every_transaction() {
    let n = 2048usize;
    let parents: Vec<Vec<_>> = (0..n)
        .map(|i| {
            i.checked_sub(1)
                .map(|parent| vec![CpfpClusterTxIndex::from(parent as u32)])
                .unwrap_or_default()
        })
        .collect();
    let items: Vec<_> = parents
        .iter()
        .enumerate()
        .map(|(i, parents)| ChunkInput {
            parents,
            fee: Sats::from(((n - i) * 100) as u64),
            vsize: VSize::from(100u64),
        })
        .collect();
    let chunks = linearize(&items);
    assert_eq!(chunks.len(), n);
    for (i, chunk) in chunks.iter().enumerate() {
        assert_eq!(chunk.txs, [CpfpClusterTxIndex::from(i as u32)]);
        assert_eq!(chunk.feerate, FeeRate::from((items[i].fee, items[i].vsize)));
    }
}
