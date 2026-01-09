//! Tests for Group types from brk_computer/src/internal/group/
//!
//! Group types aggregate multiple Vec types into logical groupings.
//! Expected outputs (flat structures):
//! - MinMax -> { min: Leaf, max: Leaf }
//! - SumCum -> { sum: Leaf, cumulative: Leaf }
//! - Percentiles -> { pct10, pct25, median, pct75, pct90 }
//! - Distribution -> { average, min, max, percentiles: {...} }
//! - Full -> { average, min, max, percentiles, sum, cumulative }
//! - Stats -> { sum, cumulative, average, min, max }
//! - MinMaxAverage -> { average, min, max }

use brk_traversable::{Index, Traversable, TreeNode};
use brk_traversable_derive::Traversable;

use crate::common::*;

// ============================================================================
// MinMax - { min: Leaf, max: Leaf }
// ============================================================================

#[derive(Traversable)]
pub struct MockMinMax {
    #[traversable(flatten)]
    pub min: MockMinVec,
    #[traversable(flatten)]
    pub max: MockMaxVec,
}

#[test]
fn min_max_produces_two_leaves() {
    let value = MockMinMax {
        min: MockMinVec(MockVec::new("metric", Index::Height)),
        max: MockMaxVec(MockVec::new("metric", Index::Height)),
    };

    let tree = value.to_tree_node();

    assert_is_branch_with_keys(&tree, &["min", "max"]);
}

// ============================================================================
// SumCum - { sum: Leaf, cumulative: Leaf }
// ============================================================================

#[derive(Traversable)]
pub struct MockSumCum {
    #[traversable(flatten)]
    pub sum: MockSumVec,
    #[traversable(flatten)]
    pub cumulative: MockCumulativeVec,
}

#[test]
fn sum_cum_produces_two_leaves() {
    let value = MockSumCum {
        sum: MockSumVec(MockVec::new("metric", Index::Height)),
        cumulative: MockCumulativeVec(MockVec::new("metric", Index::Height)),
    };

    let tree = value.to_tree_node();

    assert_is_branch_with_keys(&tree, &["sum", "cumulative"]);
}

// ============================================================================
// Percentiles - { pct10, pct25, median, pct75, pct90 }
// ============================================================================

#[derive(Traversable)]
pub struct MockPercentiles {
    pub pct10: MockPct10Vec,
    pub pct25: MockPct25Vec,
    pub median: MockMedianVec,
    pub pct75: MockPct75Vec,
    pub pct90: MockPct90Vec,
}

#[test]
fn percentiles_produces_five_leaves() {
    let value = MockPercentiles {
        pct10: MockPct10Vec(MockVec::new("m", Index::Height)),
        pct25: MockPct25Vec(MockVec::new("m", Index::Height)),
        median: MockMedianVec(MockVec::new("m", Index::Height)),
        pct75: MockPct75Vec(MockVec::new("m", Index::Height)),
        pct90: MockPct90Vec(MockVec::new("m", Index::Height)),
    };

    let tree = value.to_tree_node();

    assert_is_branch_with_keys(&tree, &["pct10", "pct25", "median", "pct75", "pct90"]);
}

// ============================================================================
// Distribution - { average, min, max, percentiles: {...} }
// ============================================================================

#[derive(Traversable)]
pub struct MockDistribution {
    #[traversable(flatten)]
    pub average: MockAverageVec,
    #[traversable(flatten)]
    pub minmax: MockMinMax,
    pub percentiles: MockPercentiles,
}

#[test]
fn distribution_flattens_average_and_minmax() {
    let value = MockDistribution {
        average: MockAverageVec(MockVec::new("m", Index::Height)),
        minmax: MockMinMax {
            min: MockMinVec(MockVec::new("m", Index::Height)),
            max: MockMaxVec(MockVec::new("m", Index::Height)),
        },
        percentiles: MockPercentiles {
            pct10: MockPct10Vec(MockVec::new("m", Index::Height)),
            pct25: MockPct25Vec(MockVec::new("m", Index::Height)),
            median: MockMedianVec(MockVec::new("m", Index::Height)),
            pct75: MockPct75Vec(MockVec::new("m", Index::Height)),
            pct90: MockPct90Vec(MockVec::new("m", Index::Height)),
        },
    };

    let tree = value.to_tree_node();

    // average and minmax are flattened, percentiles stays grouped
    assert_is_branch_with_keys(&tree, &["average", "min", "max", "percentiles"]);

    // Verify percentiles is a branch with 5 keys
    if let TreeNode::Branch(map) = &tree {
        assert_is_branch_with_keys(
            map.get("percentiles").unwrap(),
            &["pct10", "pct25", "median", "pct75", "pct90"],
        );
    }
}

// ============================================================================
// Full - { average, min, max, percentiles, sum, cumulative }
// ============================================================================

#[derive(Traversable)]
pub struct MockFull {
    #[traversable(flatten)]
    pub distribution: MockDistribution,
    #[traversable(flatten)]
    pub sum_cum: MockSumCum,
}

#[test]
fn full_flattens_distribution_and_sum_cum() {
    let value = MockFull {
        distribution: MockDistribution {
            average: MockAverageVec(MockVec::new("m", Index::Height)),
            minmax: MockMinMax {
                min: MockMinVec(MockVec::new("m", Index::Height)),
                max: MockMaxVec(MockVec::new("m", Index::Height)),
            },
            percentiles: MockPercentiles {
                pct10: MockPct10Vec(MockVec::new("m", Index::Height)),
                pct25: MockPct25Vec(MockVec::new("m", Index::Height)),
                median: MockMedianVec(MockVec::new("m", Index::Height)),
                pct75: MockPct75Vec(MockVec::new("m", Index::Height)),
                pct90: MockPct90Vec(MockVec::new("m", Index::Height)),
            },
        },
        sum_cum: MockSumCum {
            sum: MockSumVec(MockVec::new("m", Index::Height)),
            cumulative: MockCumulativeVec(MockVec::new("m", Index::Height)),
        },
    };

    let tree = value.to_tree_node();

    // Everything flattened except percentiles
    assert_is_branch_with_keys(
        &tree,
        &["average", "min", "max", "percentiles", "sum", "cumulative"],
    );
}

// ============================================================================
// Stats - { sum, cumulative, average, min, max }
// ============================================================================

#[derive(Traversable)]
pub struct MockStats {
    #[traversable(flatten)]
    pub sum_cum: MockSumCum,
    #[traversable(flatten)]
    pub average: MockAverageVec,
    #[traversable(flatten)]
    pub minmax: MockMinMax,
}

#[test]
fn stats_flattens_all() {
    let value = MockStats {
        sum_cum: MockSumCum {
            sum: MockSumVec(MockVec::new("m", Index::Height)),
            cumulative: MockCumulativeVec(MockVec::new("m", Index::Height)),
        },
        average: MockAverageVec(MockVec::new("m", Index::Height)),
        minmax: MockMinMax {
            min: MockMinVec(MockVec::new("m", Index::Height)),
            max: MockMaxVec(MockVec::new("m", Index::Height)),
        },
    };

    let tree = value.to_tree_node();

    assert_is_branch_with_keys(&tree, &["sum", "cumulative", "average", "min", "max"]);
}

// ============================================================================
// MinMaxAverage - { average, min, max }
// ============================================================================

#[derive(Traversable)]
pub struct MockMinMaxAverage {
    pub average: MockAverageVec,
    #[traversable(flatten)]
    pub minmax: MockMinMax,
}

#[test]
fn min_max_average_flattens_minmax() {
    let value = MockMinMaxAverage {
        average: MockAverageVec(MockVec::new("m", Index::Height)),
        minmax: MockMinMax {
            min: MockMinVec(MockVec::new("m", Index::Height)),
            max: MockMaxVec(MockVec::new("m", Index::Height)),
        },
    };

    let tree = value.to_tree_node();

    assert_is_branch_with_keys(&tree, &["average", "min", "max"]);
}
