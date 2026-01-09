//! Tests for Computed types from brk_computer/src/internal/computed/
//!
//! Computed types combine base vecs with derived aggregations.
//! With merge, all same-key leaves collapse.
//!
//! Expected outputs:
//! - ComputedDateLast -> Leaf (all same name → single leaf)
//! - ComputedBlockSumCum -> { base, sum, cumulative } (with merged indexes)
//! - DerivedComputedBlockSumCum -> { height_cumulative, sum, cumulative }

use brk_traversable::{Index, Traversable, TreeNode};
use brk_traversable_derive::Traversable;

use crate::common::*;
use crate::group_types::MockSumCum;
use crate::lazy_aggregation::MockLazySumCum;
use crate::derived_date::{MockDerivedDateLast, MockDerivedDateSumCum};

// ============================================================================
// ComputedDateLast - Leaf (dateindex + rest collapse to single leaf)
// ============================================================================

#[derive(Traversable)]
#[traversable(merge)]
pub struct MockComputedDateLast {
    pub dateindex: MockLastVec,  // transparent → Leaf
    #[traversable(flatten)]
    pub rest: MockDerivedDateLast,  // merge → Leaf (all same name)
}

#[test]
fn computed_date_last_collapses_to_leaf() {
    let value = MockComputedDateLast {
        dateindex: MockLastVec(MockVec::new("metric", Index::DateIndex)),
        rest: MockDerivedDateLast {
            weekindex: MockLazyLast(MockVec::new("metric", Index::WeekIndex)),
            monthindex: MockLazyLast(MockVec::new("metric", Index::MonthIndex)),
            quarterindex: MockLazyLast(MockVec::new("metric", Index::QuarterIndex)),
            semesterindex: MockLazyLast(MockVec::new("metric", Index::SemesterIndex)),
            yearindex: MockLazyLast(MockVec::new("metric", Index::YearIndex)),
            decadeindex: MockLazyLast(MockVec::new("metric", Index::DecadeIndex)),
        },
    };

    let tree = value.to_tree_node();

    // All same metric name → single Leaf with all indexes
    match &tree {
        TreeNode::Leaf(l) => {
            assert_eq!(l.name(), "metric");
            let indexes = l.indexes();
            assert!(indexes.contains(&Index::DateIndex));
            assert!(indexes.contains(&Index::WeekIndex));
            assert!(indexes.contains(&Index::MonthIndex));
            assert!(indexes.contains(&Index::QuarterIndex));
            assert!(indexes.contains(&Index::SemesterIndex));
            assert!(indexes.contains(&Index::YearIndex));
            assert!(indexes.contains(&Index::DecadeIndex));
            assert_eq!(indexes.len(), 7);
        }
        TreeNode::Branch(map) => {
            panic!("Expected Leaf, got Branch: {:?}", map.keys().collect::<Vec<_>>());
        }
    }
}

// ============================================================================
// DerivedComputedBlockSumCum - { height_cumulative, sum, cumulative }
// ============================================================================

// For merge to work correctly, all fields produce Branch output that merge will lift.
// height_cumulative is renamed to "cumulative" so it merges with other cumulative leaves.
// NO flatten used - rely entirely on merge to lift and merge same-key leaves.
#[derive(Traversable)]
#[traversable(merge)]
pub struct MockDerivedComputedBlockSumCum {
    #[traversable(rename = "cumulative")]  // rename to merge with other cumulative leaves
    pub height_cumulative: MockCumulativeVec,
    pub dateindex: MockSumCum,  // produces { sum, cumulative } - merge will lift
    pub dates: MockDerivedDateSumCum,  // produces { sum, cumulative } - merge will lift
    pub difficultyepoch: MockLazySumCum,  // produces { sum, cumulative } - merge will lift
}

#[test]
fn derived_computed_block_sum_cum_merges_all() {
    let value = MockDerivedComputedBlockSumCum {
        height_cumulative: MockCumulativeVec(MockVec::new("metric_cumulative", Index::Height)),
        dateindex: MockSumCum {
            sum: MockSumVec(MockVec::new("metric_sum", Index::DateIndex)),
            cumulative: MockCumulativeVec(MockVec::new("metric_cumulative", Index::DateIndex)),
        },
        dates: MockDerivedDateSumCum {
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
        },
        difficultyepoch: MockLazySumCum {
            sum: MockLazySum(MockVec::new("metric_sum", Index::DifficultyEpoch)),
            cumulative: MockLazyCumulative(MockVec::new("metric_cumulative", Index::DifficultyEpoch)),
        },
    };

    let tree = value.to_tree_node();

    // Debug: print tree structure
    println!("Tree: {tree:#?}");

    // height_cumulative renamed to "cumulative" → merges with other cumulative leaves
    // sum merges from dateindex + dates + difficultyepoch
    // cumulative merges from height_cumulative (renamed) + dateindex + dates + difficultyepoch
    assert_is_branch_with_keys(&tree, &["sum", "cumulative"]);

    if let TreeNode::Branch(map) = &tree {
        // sum: DateIndex + Week + Month + Quarter + Year + DifficultyEpoch
        let sum_indexes = get_leaf_indexes(map.get("sum").unwrap()).unwrap();
        assert!(sum_indexes.contains(&Index::DateIndex));
        assert!(sum_indexes.contains(&Index::WeekIndex));
        assert!(sum_indexes.contains(&Index::MonthIndex));
        assert!(sum_indexes.contains(&Index::QuarterIndex));
        assert!(sum_indexes.contains(&Index::YearIndex));
        assert!(sum_indexes.contains(&Index::DifficultyEpoch));
        assert_eq!(sum_indexes.len(), 6);

        // cumulative: Height + DateIndex + Week + Month + Quarter + Year + DifficultyEpoch
        let cum_indexes = get_leaf_indexes(map.get("cumulative").unwrap()).unwrap();
        assert!(cum_indexes.contains(&Index::Height), "cumulative SHOULD have Height from renamed height_cumulative");
        assert!(cum_indexes.contains(&Index::DateIndex));
        assert!(cum_indexes.contains(&Index::WeekIndex));
        assert!(cum_indexes.contains(&Index::MonthIndex));
        assert!(cum_indexes.contains(&Index::QuarterIndex));
        assert!(cum_indexes.contains(&Index::YearIndex));
        assert!(cum_indexes.contains(&Index::DifficultyEpoch));
        assert_eq!(cum_indexes.len(), 7);
    }
}

// ============================================================================
// ComputedBlockSumCum - { base, sum, cumulative }
// ============================================================================

#[derive(Traversable)]
#[traversable(merge)]
pub struct MockComputedBlockSumCum {
    #[traversable(wrap = "base")]
    pub height: MockSumVec,  // wrap="base" → { base: Leaf }
    pub rest: MockDerivedComputedBlockSumCum,  // merge will lift from rest
}

#[test]
fn computed_block_sum_cum_produces_base_sum_cumulative() {
    let value = MockComputedBlockSumCum {
        height: MockSumVec(MockVec::new("metric", Index::Height)),
        rest: MockDerivedComputedBlockSumCum {
            height_cumulative: MockCumulativeVec(MockVec::new("metric_cumulative", Index::Height)),
            dateindex: MockSumCum {
                sum: MockSumVec(MockVec::new("metric_sum", Index::DateIndex)),
                cumulative: MockCumulativeVec(MockVec::new("metric_cumulative", Index::DateIndex)),
            },
            dates: MockDerivedDateSumCum {
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
            },
            difficultyepoch: MockLazySumCum {
                sum: MockLazySum(MockVec::new("metric_sum", Index::DifficultyEpoch)),
                cumulative: MockLazyCumulative(MockVec::new("metric_cumulative", Index::DifficultyEpoch)),
            },
        },
    };

    let tree = value.to_tree_node();

    // base: Height only (from wrap="base")
    // sum: all indexes except Height
    // cumulative: all indexes INCLUDING Height (from renamed height_cumulative)
    assert_is_branch_with_keys(&tree, &["base", "sum", "cumulative"]);

    if let TreeNode::Branch(map) = &tree {
        // base: Height only
        let base_indexes = get_leaf_indexes(map.get("base").unwrap()).unwrap();
        assert!(base_indexes.contains(&Index::Height));
        assert_eq!(base_indexes.len(), 1);

        // sum: DateIndex + all dates + DifficultyEpoch (NOT Height)
        let sum_indexes = get_leaf_indexes(map.get("sum").unwrap()).unwrap();
        assert!(!sum_indexes.contains(&Index::Height), "sum should NOT have Height");
        assert!(sum_indexes.contains(&Index::DateIndex));
        assert!(sum_indexes.contains(&Index::DifficultyEpoch));
        assert_eq!(sum_indexes.len(), 6);

        // cumulative: Height + DateIndex + all dates + DifficultyEpoch
        let cum_indexes = get_leaf_indexes(map.get("cumulative").unwrap()).unwrap();
        assert!(cum_indexes.contains(&Index::Height), "cumulative SHOULD have Height");
        assert!(cum_indexes.contains(&Index::DateIndex));
        assert!(cum_indexes.contains(&Index::DifficultyEpoch));
        assert_eq!(cum_indexes.len(), 7);
    }
}
