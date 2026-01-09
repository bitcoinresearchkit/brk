//! Common mock types and helpers for traversable tests.

use std::collections::BTreeSet;

use brk_traversable::{Index, MetricLeaf, MetricLeafWithSchema, Traversable, TreeNode};
use brk_traversable_derive::Traversable;

/// Mock leaf vec that produces a Leaf node with given name and index.
/// This simulates the behavior of EagerVec<PcoVec<I, T>>.
pub struct MockVec {
    pub name: String,
    pub index: Index,
}

impl MockVec {
    pub fn new(name: &str, index: Index) -> Self {
        Self {
            name: name.to_string(),
            index,
        }
    }
}

impl Traversable for MockVec {
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Leaf(MetricLeafWithSchema::new(
            MetricLeaf::new(
                self.name.clone(),
                "MockType".to_string(),
                BTreeSet::from([self.index]),
            ),
            serde_json::Value::Null,
        ))
    }

    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn vecdb::AnyExportableVec> {
        std::iter::empty()
    }
}

// ============================================================================
// Transparent Vec Types (matching real SumVec, CumulativeVec, MinVec, etc.)
// ============================================================================
// All real Vec types are now transparent - they delegate directly to inner.

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockSumVec(pub MockVec);

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockCumulativeVec(pub MockVec);

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockMinVec(pub MockVec);

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockMaxVec(pub MockVec);

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockAverageVec(pub MockVec);

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockMedianVec(pub MockVec);

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockPct10Vec(pub MockVec);

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockPct25Vec(pub MockVec);

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockPct75Vec(pub MockVec);

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockPct90Vec(pub MockVec);

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockLastVec(pub MockVec);

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockFirstVec(pub MockVec);

// ============================================================================
// Transparent Lazy Types (matching real LazySum, LazyCumulative, etc.)
// ============================================================================
// All real Lazy* types are now transparent.

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockLazySum(pub MockVec);

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockLazyCumulative(pub MockVec);

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockLazyMin(pub MockVec);

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockLazyMax(pub MockVec);

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockLazyAverage(pub MockVec);

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockLazyFirst(pub MockVec);

#[derive(Traversable)]
#[traversable(transparent)]
pub struct MockLazyLast(pub MockVec);

// ============================================================================
// Helper functions
// ============================================================================

pub fn get_leaf_indexes(node: &TreeNode) -> Option<&BTreeSet<Index>> {
    match node {
        TreeNode::Leaf(l) => Some(l.indexes()),
        _ => None,
    }
}

pub fn get_leaf_name(node: &TreeNode) -> Option<&str> {
    match node {
        TreeNode::Leaf(l) => Some(l.name()),
        _ => None,
    }
}

pub fn assert_is_leaf(node: &TreeNode, expected_name: &str) {
    match node {
        TreeNode::Leaf(l) => assert_eq!(l.name(), expected_name),
        TreeNode::Branch(map) => panic!("Expected Leaf, got Branch: {:?}", map.keys().collect::<Vec<_>>()),
    }
}

pub fn assert_is_branch_with_keys(node: &TreeNode, expected_keys: &[&str]) {
    match node {
        TreeNode::Branch(map) => {
            for key in expected_keys {
                assert!(map.contains_key(*key), "Missing key: {key}");
            }
            assert_eq!(map.len(), expected_keys.len(), "Got keys: {:?}", map.keys().collect::<Vec<_>>());
        }
        TreeNode::Leaf(l) => panic!("Expected Branch, got Leaf: {}", l.name()),
    }
}
