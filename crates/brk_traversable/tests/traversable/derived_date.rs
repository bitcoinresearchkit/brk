//! Tests for Derived Date types from brk_computer/src/internal/derived/date/
//!
//! Derived Date types aggregate metrics across multiple time periods (week, month, etc.).
//! With merge, all same-key leaves collapse across time periods.
//!
//! Expected outputs:
//! - DerivedDateLast -> Leaf (all indexes merged)
//! - DerivedDateSumCum -> { sum: Leaf(all), cumulative: Leaf(all) }
//! - DerivedDateFull -> { average, min, max, sum, cumulative } (all with merged indexes)
//! - etc.

use brk_traversable::{Index, Traversable, TreeNode};
use brk_traversable_derive::Traversable;

use crate::common::*;
use crate::lazy_aggregation::{MockLazyFull, MockLazySumCum};

// ============================================================================
// DerivedDateLast - Leaf (all same name → single leaf with all indexes)
// ============================================================================

#[derive(Traversable)]
#[traversable(merge)]
pub struct MockDerivedDateLast {
    pub weekindex: MockLazyLast,
    pub monthindex: MockLazyLast,
    pub quarterindex: MockLazyLast,
    pub semesterindex: MockLazyLast,
    pub yearindex: MockLazyLast,
    pub decadeindex: MockLazyLast,
}

#[test]
fn derived_date_last_collapses_to_leaf() {
    let value = MockDerivedDateLast {
        weekindex: MockLazyLast(MockVec::new("metric", Index::WeekIndex)),
        monthindex: MockLazyLast(MockVec::new("metric", Index::MonthIndex)),
        quarterindex: MockLazyLast(MockVec::new("metric", Index::QuarterIndex)),
        semesterindex: MockLazyLast(MockVec::new("metric", Index::SemesterIndex)),
        yearindex: MockLazyLast(MockVec::new("metric", Index::YearIndex)),
        decadeindex: MockLazyLast(MockVec::new("metric", Index::DecadeIndex)),
    };

    let tree = value.to_tree_node();

    // All same metric name → collapses to single Leaf with all indexes
    match &tree {
        TreeNode::Leaf(l) => {
            assert_eq!(l.name(), "metric");
            let indexes = l.indexes();
            assert!(indexes.contains(&Index::WeekIndex));
            assert!(indexes.contains(&Index::MonthIndex));
            assert!(indexes.contains(&Index::QuarterIndex));
            assert!(indexes.contains(&Index::SemesterIndex));
            assert!(indexes.contains(&Index::YearIndex));
            assert!(indexes.contains(&Index::DecadeIndex));
            assert_eq!(indexes.len(), 6);
        }
        TreeNode::Branch(map) => {
            panic!("Expected Leaf, got Branch: {:?}", map.keys().collect::<Vec<_>>());
        }
    }
}

// ============================================================================
// DerivedDateSumCum - { sum: Leaf(all), cumulative: Leaf(all) }
// ============================================================================

#[derive(Traversable)]
#[traversable(merge)]
pub struct MockDerivedDateSumCum {
    pub weekindex: MockLazySumCum,
    pub monthindex: MockLazySumCum,
    pub quarterindex: MockLazySumCum,
    pub yearindex: MockLazySumCum,
}

#[test]
fn derived_date_sum_cum_merges_all_time_periods() {
    let value = MockDerivedDateSumCum {
        weekindex: MockLazySumCum {
            sum: MockLazySum(MockVec::new("metric_sum", Index::WeekIndex)),
            cumulative: MockLazyCumulative(MockVec::new("metric_cumulative", Index::WeekIndex)),
        },
        monthindex: MockLazySumCum {
            sum: MockLazySum(MockVec::new("metric_sum", Index::MonthIndex)),
            cumulative: MockLazyCumulative(MockVec::new("metric_cumulative", Index::MonthIndex)),
        },
        quarterindex: MockLazySumCum {
            sum: MockLazySum(MockVec::new("metric_sum", Index::QuarterIndex)),
            cumulative: MockLazyCumulative(MockVec::new("metric_cumulative", Index::QuarterIndex)),
        },
        yearindex: MockLazySumCum {
            sum: MockLazySum(MockVec::new("metric_sum", Index::YearIndex)),
            cumulative: MockLazyCumulative(MockVec::new("metric_cumulative", Index::YearIndex)),
        },
    };

    let tree = value.to_tree_node();

    // Merge lifts from all time periods → { sum: Leaf(all), cumulative: Leaf(all) }
    assert_is_branch_with_keys(&tree, &["sum", "cumulative"]);

    if let TreeNode::Branch(map) = &tree {
        // sum should have all 4 indexes
        let sum_indexes = get_leaf_indexes(map.get("sum").unwrap()).unwrap();
        assert!(sum_indexes.contains(&Index::WeekIndex));
        assert!(sum_indexes.contains(&Index::MonthIndex));
        assert!(sum_indexes.contains(&Index::QuarterIndex));
        assert!(sum_indexes.contains(&Index::YearIndex));
        assert_eq!(sum_indexes.len(), 4);

        // cumulative should have all 4 indexes
        let cum_indexes = get_leaf_indexes(map.get("cumulative").unwrap()).unwrap();
        assert_eq!(cum_indexes.len(), 4);
    }
}

// ============================================================================
// DerivedDateFull - { average, min, max, sum, cumulative } (all merged)
// ============================================================================

#[derive(Traversable)]
#[traversable(merge)]
pub struct MockDerivedDateFull {
    pub weekindex: MockLazyFull,
    pub monthindex: MockLazyFull,
    pub yearindex: MockLazyFull,
}

#[test]
fn derived_date_full_merges_all_stats() {
    let value = MockDerivedDateFull {
        weekindex: MockLazyFull {
            average: MockLazyAverage(MockVec::new("m_avg", Index::WeekIndex)),
            min: MockLazyMin(MockVec::new("m_min", Index::WeekIndex)),
            max: MockLazyMax(MockVec::new("m_max", Index::WeekIndex)),
            sum: MockLazySum(MockVec::new("m_sum", Index::WeekIndex)),
            cumulative: MockLazyCumulative(MockVec::new("m_cum", Index::WeekIndex)),
        },
        monthindex: MockLazyFull {
            average: MockLazyAverage(MockVec::new("m_avg", Index::MonthIndex)),
            min: MockLazyMin(MockVec::new("m_min", Index::MonthIndex)),
            max: MockLazyMax(MockVec::new("m_max", Index::MonthIndex)),
            sum: MockLazySum(MockVec::new("m_sum", Index::MonthIndex)),
            cumulative: MockLazyCumulative(MockVec::new("m_cum", Index::MonthIndex)),
        },
        yearindex: MockLazyFull {
            average: MockLazyAverage(MockVec::new("m_avg", Index::YearIndex)),
            min: MockLazyMin(MockVec::new("m_min", Index::YearIndex)),
            max: MockLazyMax(MockVec::new("m_max", Index::YearIndex)),
            sum: MockLazySum(MockVec::new("m_sum", Index::YearIndex)),
            cumulative: MockLazyCumulative(MockVec::new("m_cum", Index::YearIndex)),
        },
    };

    let tree = value.to_tree_node();

    // All same keys merge → { average, min, max, sum, cumulative }
    assert_is_branch_with_keys(&tree, &["average", "min", "max", "sum", "cumulative"]);

    if let TreeNode::Branch(map) = &tree {
        // Each should have 3 indexes (week, month, year)
        for key in ["average", "min", "max", "sum", "cumulative"] {
            let indexes = get_leaf_indexes(map.get(key).unwrap()).unwrap();
            assert_eq!(indexes.len(), 3, "{key} should have 3 indexes");
            assert!(indexes.contains(&Index::WeekIndex));
            assert!(indexes.contains(&Index::MonthIndex));
            assert!(indexes.contains(&Index::YearIndex));
        }
    }
}
