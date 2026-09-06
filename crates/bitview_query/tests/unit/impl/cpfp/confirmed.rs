use brk_error::Result;
use brk_types::TxIndex;
use smallvec::SmallVec;
use vecdb::VecIndex;

use super::{walk_component, walk_direction};

fn adjacent(graph: &[Vec<usize>], tx: TxIndex) -> Result<SmallVec<[TxIndex; 2]>> {
    Ok(graph[tx.to_usize()]
        .iter()
        .copied()
        .map(TxIndex::from)
        .collect())
}

#[test]
fn component_includes_siblings_but_directional_walk_does_not() {
    let parents = vec![vec![], vec![0], vec![0]];
    let children = vec![vec![1, 2], vec![], vec![]];
    let seed = TxIndex::from(1usize);
    let mut parent = |tx| adjacent(&parents, tx);
    let mut child = |tx| adjacent(&children, tx);

    let mut component = walk_component(seed, &mut parent, &mut child, 64).unwrap();
    component.sort_unstable();
    assert_eq!(component, [0usize, 1, 2].map(TxIndex::from));

    assert_eq!(
        walk_direction(seed, &mut parent, 25).unwrap(),
        [TxIndex::from(0usize)]
    );
    assert!(walk_direction(seed, &mut child, 25).unwrap().is_empty());
}
