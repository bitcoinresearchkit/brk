//! Tests for Lazy Aggregation types from brk_computer/src/internal/aggregation/
//!
//! Lazy aggregation types compose multiple Lazy* types.
//! Expected outputs (flat structures):
//! - LazySumCum -> { sum: Leaf, cumulative: Leaf }
//! - LazyDistribution -> { average, min, max }
//! - LazyFull -> { average, min, max, sum, cumulative }

use brk_traversable::{Index, Traversable, TreeNode};
use brk_traversable_derive::Traversable;

use crate::common::*;

// ============================================================================
// LazySumCum - { sum: Leaf, cumulative: Leaf }
// ============================================================================

#[derive(Traversable)]
pub struct MockLazySumCum {
    #[traversable(flatten)]
    pub sum: MockLazySum,
    #[traversable(flatten)]
    pub cumulative: MockLazyCumulative,
}

#[test]
fn lazy_sum_cum_produces_two_leaves() {
    let value = MockLazySumCum {
        sum: MockLazySum(MockVec::new("metric", Index::WeekIndex)),
        cumulative: MockLazyCumulative(MockVec::new("metric", Index::WeekIndex)),
    };

    let tree = value.to_tree_node();

    assert_is_branch_with_keys(&tree, &["sum", "cumulative"]);
}

// ============================================================================
// LazyDistribution - { average, min, max }
// ============================================================================

#[derive(Traversable)]
pub struct MockLazyDistribution {
    #[traversable(flatten)]
    pub average: MockLazyAverage,
    #[traversable(flatten)]
    pub min: MockLazyMin,
    #[traversable(flatten)]
    pub max: MockLazyMax,
}

#[test]
fn lazy_distribution_produces_three_leaves() {
    let value = MockLazyDistribution {
        average: MockLazyAverage(MockVec::new("m", Index::WeekIndex)),
        min: MockLazyMin(MockVec::new("m", Index::WeekIndex)),
        max: MockLazyMax(MockVec::new("m", Index::WeekIndex)),
    };

    let tree = value.to_tree_node();

    assert_is_branch_with_keys(&tree, &["average", "min", "max"]);
}

// ============================================================================
// LazyFull - { average, min, max, sum, cumulative }
// ============================================================================

#[derive(Traversable)]
pub struct MockLazyFull {
    #[traversable(flatten)]
    pub average: MockLazyAverage,
    #[traversable(flatten)]
    pub min: MockLazyMin,
    #[traversable(flatten)]
    pub max: MockLazyMax,
    #[traversable(flatten)]
    pub sum: MockLazySum,
    #[traversable(flatten)]
    pub cumulative: MockLazyCumulative,
}

#[test]
fn lazy_full_produces_five_leaves() {
    let value = MockLazyFull {
        average: MockLazyAverage(MockVec::new("m", Index::DifficultyEpoch)),
        min: MockLazyMin(MockVec::new("m", Index::DifficultyEpoch)),
        max: MockLazyMax(MockVec::new("m", Index::DifficultyEpoch)),
        sum: MockLazySum(MockVec::new("m", Index::DifficultyEpoch)),
        cumulative: MockLazyCumulative(MockVec::new("m", Index::DifficultyEpoch)),
    };

    let tree = value.to_tree_node();

    assert_is_branch_with_keys(&tree, &["average", "min", "max", "sum", "cumulative"]);
}

// ============================================================================
// Merge behavior: Multiple time periods collapse to single leaves
// ============================================================================

#[derive(Traversable)]
#[traversable(merge)]
pub struct MockDerivedDateSumCum {
    pub weekindex: MockLazySumCum,
    pub monthindex: MockLazySumCum,
}

#[test]
fn derived_date_sum_cum_merges_time_periods() {
    let value = MockDerivedDateSumCum {
        weekindex: MockLazySumCum {
            sum: MockLazySum(MockVec::new("metric_sum", Index::WeekIndex)),
            cumulative: MockLazyCumulative(MockVec::new("metric_cumulative", Index::WeekIndex)),
        },
        monthindex: MockLazySumCum {
            sum: MockLazySum(MockVec::new("metric_sum", Index::MonthIndex)),
            cumulative: MockLazyCumulative(MockVec::new("metric_cumulative", Index::MonthIndex)),
        },
    };

    let tree = value.to_tree_node();

    // Merge lifts children from weekindex and monthindex
    // Same keys merge → { sum: Leaf(Week+Month), cumulative: Leaf(Week+Month) }
    assert_is_branch_with_keys(&tree, &["sum", "cumulative"]);

    if let TreeNode::Branch(map) = &tree {
        // sum should have both indexes merged
        let sum_indexes = get_leaf_indexes(map.get("sum").unwrap()).unwrap();
        assert!(sum_indexes.contains(&Index::WeekIndex));
        assert!(sum_indexes.contains(&Index::MonthIndex));

        // cumulative should have both indexes merged
        let cum_indexes = get_leaf_indexes(map.get("cumulative").unwrap()).unwrap();
        assert!(cum_indexes.contains(&Index::WeekIndex));
        assert!(cum_indexes.contains(&Index::MonthIndex));
    }
}

// ============================================================================
// Full merge: All same metric name → collapses to single Leaf
// ============================================================================

#[derive(Traversable)]
#[traversable(merge)]
pub struct MockDerivedDateLast {
    pub weekindex: MockLazyLast,
    pub monthindex: MockLazyLast,
    pub yearindex: MockLazyLast,
}

#[test]
fn derived_date_last_collapses_to_single_leaf() {
    let value = MockDerivedDateLast {
        weekindex: MockLazyLast(MockVec::new("metric", Index::WeekIndex)),
        monthindex: MockLazyLast(MockVec::new("metric", Index::MonthIndex)),
        yearindex: MockLazyLast(MockVec::new("metric", Index::YearIndex)),
    };

    let tree = value.to_tree_node();

    // All same metric name → collapses to single Leaf with all indexes
    match &tree {
        TreeNode::Leaf(l) => {
            assert_eq!(l.name(), "metric");
            let indexes = l.indexes();
            assert!(indexes.contains(&Index::WeekIndex));
            assert!(indexes.contains(&Index::MonthIndex));
            assert!(indexes.contains(&Index::YearIndex));
        }
        TreeNode::Branch(map) => {
            panic!("Expected Leaf, got Branch: {:?}", map.keys().collect::<Vec<_>>());
        }
    }
}
