//! Integration tests for the Traversable derive macro.
//!
//! Tests struct patterns similar to those in brk_computer to ensure
//! the derive macro produces correct tree structures.

use std::collections::BTreeSet;

use brk_traversable::{Index, MetricLeaf, MetricLeafWithSchema, Traversable, TreeNode};
use brk_traversable_derive::Traversable;

// ============================================================================
// Mock vec types for testing
// ============================================================================

/// Mock leaf vec that produces a Leaf node with given name and index
struct MockVec {
    name: String,
    index: Index,
}

impl MockVec {
    fn new(name: &str, index: Index) -> Self {
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
// Helper functions
// ============================================================================

fn get_leaf_indexes(node: &TreeNode) -> Option<&BTreeSet<Index>> {
    match node {
        TreeNode::Leaf(l) => Some(l.indexes()),
        _ => None,
    }
}

fn get_leaf_name(node: &TreeNode) -> Option<&str> {
    match node {
        TreeNode::Leaf(l) => Some(l.name()),
        _ => None,
    }
}

// ============================================================================
// Case 1: LazyBlockValue pattern
// ============================================================================
// LazyBlockValue (no merge on struct):
//   - sats: wrap="sats"
//   - rest: LazyDerivedBlockValue with flatten
//     - bitcoin: plain field
//     - dollars: Option, plain field

#[derive(Traversable)]
struct MockLazyDerivedBlockValue {
    pub bitcoin: MockVec,
    pub dollars: Option<MockVec>,
}

#[derive(Traversable)]
struct MockLazyBlockValue {
    #[traversable(rename = "sats")]
    pub sats: MockVec,
    #[traversable(flatten)]
    pub rest: MockLazyDerivedBlockValue,
}

#[test]
fn lazy_block_value_flat_denomination_siblings() {
    let value = MockLazyBlockValue {
        sats: MockVec::new("metric", Index::Height),
        rest: MockLazyDerivedBlockValue {
            bitcoin: MockVec::new("metric_btc", Index::Height),
            dollars: Some(MockVec::new("metric_usd", Index::Height)),
        },
    };

    let tree = value.to_tree_node();

    // rename="sats" keeps the leaf directly as sats: Leaf
    // flatten on rest lifts bitcoin, dollars as direct leaves
    // Result: { sats: Leaf, bitcoin: Leaf, dollars: Leaf }
    match &tree {
        TreeNode::Branch(map) => {
            assert_eq!(map.len(), 3, "Expected sats, bitcoin, dollars. Got: {:?}", map.keys().collect::<Vec<_>>());
            assert!(matches!(map.get("sats"), Some(TreeNode::Leaf(_))));
            assert!(matches!(map.get("bitcoin"), Some(TreeNode::Leaf(_))));
            assert!(matches!(map.get("dollars"), Some(TreeNode::Leaf(_))));
        }
        _ => panic!("Expected branch"),
    }
}

#[test]
fn lazy_block_value_without_dollars() {
    let value = MockLazyBlockValue {
        sats: MockVec::new("metric", Index::Height),
        rest: MockLazyDerivedBlockValue {
            bitcoin: MockVec::new("metric_btc", Index::Height),
            dollars: None,
        },
    };

    let tree = value.to_tree_node();

    // Expected: { sats: Leaf, bitcoin: Leaf } (no dollars)
    match &tree {
        TreeNode::Branch(map) => {
            assert_eq!(map.len(), 2);
            assert!(map.contains_key("sats"));
            assert!(map.contains_key("bitcoin"));
            assert!(!map.contains_key("dollars"));
        }
        _ => panic!("Expected branch"),
    }
}

// ============================================================================
// Case 2: DerivedDateLast pattern (merge collapses to Leaf)
// ============================================================================
// DerivedDateLast<T> (merge):
//   - weekindex: LazyLast with wrap="last"
//   - monthindex: LazyLast with wrap="last"
//   - yearindex: LazyLast with wrap="last"
// All produce { last: Leaf } → merge collapses to single Leaf

#[derive(Traversable)]
#[traversable(wrap = "last")]
struct MockLazyLast(MockVec);

#[derive(Traversable)]
#[traversable(merge)]
struct MockDerivedDateLast {
    pub weekindex: MockLazyLast,
    pub monthindex: MockLazyLast,
    pub yearindex: MockLazyLast,
}

#[test]
fn derived_date_last_collapses_to_leaf() {
    let value = MockDerivedDateLast {
        weekindex: MockLazyLast(MockVec::new("metric", Index::WeekIndex)),
        monthindex: MockLazyLast(MockVec::new("metric", Index::MonthIndex)),
        yearindex: MockLazyLast(MockVec::new("metric", Index::YearIndex)),
    };

    let tree = value.to_tree_node();

    // All fields produce { last: Leaf } with same metric name
    // Merge lifts all "last" keys → same name → collapses to single Leaf
    match &tree {
        TreeNode::Leaf(leaf) => {
            assert_eq!(leaf.name(), "metric");
            let indexes = leaf.indexes();
            assert!(indexes.contains(&Index::WeekIndex));
            assert!(indexes.contains(&Index::MonthIndex));
            assert!(indexes.contains(&Index::YearIndex));
        }
        TreeNode::Branch(map) => {
            panic!("Expected Leaf, got Branch: {:?}", map.keys().collect::<Vec<_>>());
        }
    }
}

// ============================================================================
// Case 3: ComputedDateLast pattern (base + collapsed rest)
// ============================================================================
// ComputedDateLast<T> (merge):
//   - dateindex: wrap="base"
//   - rest: DerivedDateLast with flatten → collapses to Leaf
// If all same metric name → collapses to single Leaf

#[derive(Traversable)]
#[traversable(merge)]
struct MockComputedDateLast {
    #[traversable(wrap = "base")]
    pub dateindex: MockVec,
    #[traversable(flatten)]
    pub rest: MockDerivedDateLast,
}

#[test]
fn computed_date_last_collapses_to_leaf() {
    let value = MockComputedDateLast {
        dateindex: MockVec::new("metric", Index::DateIndex),
        rest: MockDerivedDateLast {
            weekindex: MockLazyLast(MockVec::new("metric", Index::WeekIndex)),
            monthindex: MockLazyLast(MockVec::new("metric", Index::MonthIndex)),
            yearindex: MockLazyLast(MockVec::new("metric", Index::YearIndex)),
        },
    };

    let tree = value.to_tree_node();

    // dateindex produces { base: Leaf("metric", DateIndex) }
    // rest (flatten) produces Leaf("metric", Week+Month+Year) → inserted as "rest" key
    // All same metric name → collapses to single Leaf
    match &tree {
        TreeNode::Leaf(leaf) => {
            assert_eq!(leaf.name(), "metric");
            let indexes = leaf.indexes();
            assert!(indexes.contains(&Index::DateIndex));
            assert!(indexes.contains(&Index::WeekIndex));
            assert!(indexes.contains(&Index::MonthIndex));
            assert!(indexes.contains(&Index::YearIndex));
        }
        TreeNode::Branch(map) => {
            panic!("Expected Leaf, got Branch: {:?}", map.keys().collect::<Vec<_>>());
        }
    }
}

// ============================================================================
// Case 4: ValueDerivedDateLast pattern (denomination siblings)
// ============================================================================
// ValueDerivedDateLast (no merge):
//   - sats: DerivedDateLast<Sats> (merge) → Leaf
//   - bitcoin: LazyDateLast<Bitcoin> (merge) → Leaf
//   - dollars: Option<ComputedDateLast<Dollars>> (merge) → Leaf

#[derive(Traversable)]
struct MockValueDerivedDateLast {
    pub sats: MockDerivedDateLast,
    pub bitcoin: MockDerivedDateLast,
    pub dollars: Option<MockComputedDateLast>,
}

#[test]
fn value_derived_date_last_denomination_leaves() {
    let value = MockValueDerivedDateLast {
        sats: MockDerivedDateLast {
            weekindex: MockLazyLast(MockVec::new("metric", Index::WeekIndex)),
            monthindex: MockLazyLast(MockVec::new("metric", Index::MonthIndex)),
            yearindex: MockLazyLast(MockVec::new("metric", Index::YearIndex)),
        },
        bitcoin: MockDerivedDateLast {
            weekindex: MockLazyLast(MockVec::new("metric_btc", Index::WeekIndex)),
            monthindex: MockLazyLast(MockVec::new("metric_btc", Index::MonthIndex)),
            yearindex: MockLazyLast(MockVec::new("metric_btc", Index::YearIndex)),
        },
        dollars: Some(MockComputedDateLast {
            dateindex: MockVec::new("metric_usd", Index::DateIndex),
            rest: MockDerivedDateLast {
                weekindex: MockLazyLast(MockVec::new("metric_usd", Index::WeekIndex)),
                monthindex: MockLazyLast(MockVec::new("metric_usd", Index::MonthIndex)),
                yearindex: MockLazyLast(MockVec::new("metric_usd", Index::YearIndex)),
            },
        }),
    };

    let tree = value.to_tree_node();

    // Each inner type has merge → collapses to Leaf
    // Outer has no merge → stays as Branch with denomination keys
    match &tree {
        TreeNode::Branch(map) => {
            assert_eq!(map.len(), 3);

            // Each denomination is a collapsed Leaf
            match map.get("sats") {
                Some(TreeNode::Leaf(l)) => assert_eq!(l.name(), "metric"),
                _ => panic!("Expected sats to be Leaf"),
            }
            match map.get("bitcoin") {
                Some(TreeNode::Leaf(l)) => assert_eq!(l.name(), "metric_btc"),
                _ => panic!("Expected bitcoin to be Leaf"),
            }
            match map.get("dollars") {
                Some(TreeNode::Leaf(l)) => assert_eq!(l.name(), "metric_usd"),
                _ => panic!("Expected dollars to be Leaf"),
            }
        }
        _ => panic!("Expected branch"),
    }
}

// ============================================================================
// Case 5: SumCum pattern (sum + cumulative leaves)
// ============================================================================
// SumCum produces { sum: Leaf, cumulative: Leaf } via wrap attributes

#[derive(Traversable)]
#[traversable(wrap = "sum")]
struct MockSumVec(MockVec);

#[derive(Traversable)]
#[traversable(wrap = "cumulative")]
struct MockCumulativeVec(MockVec);

#[derive(Traversable)]
#[traversable(merge)]
struct MockSumCum {
    pub sum: MockSumVec,
    pub cumulative: MockCumulativeVec,
}

#[test]
fn sum_cum_produces_two_leaves() {
    let value = MockSumCum {
        sum: MockSumVec(MockVec::new("metric_sum", Index::DateIndex)),
        cumulative: MockCumulativeVec(MockVec::new("metric_cumulative", Index::DateIndex)),
    };

    let tree = value.to_tree_node();

    // { sum: Leaf, cumulative: Leaf } - different metric names, no collapse
    match &tree {
        TreeNode::Branch(map) => {
            assert_eq!(map.len(), 2);
            assert_eq!(get_leaf_name(map.get("sum").unwrap()), Some("metric_sum"));
            assert_eq!(get_leaf_name(map.get("cumulative").unwrap()), Some("metric_cumulative"));
        }
        _ => panic!("Expected branch with sum and cumulative"),
    }
}

// ============================================================================
// Case 6: DerivedDateSumCum pattern (multiple time periods merge)
// ============================================================================
// DerivedDateSumCum (merge):
//   - weekindex: SumCum → { sum: Leaf, cumulative: Leaf }
//   - monthindex: SumCum → { sum: Leaf, cumulative: Leaf }
// Merge lifts all → merges same keys

#[derive(Traversable)]
#[traversable(merge)]
struct MockDerivedDateSumCum {
    pub weekindex: MockSumCum,
    pub monthindex: MockSumCum,
}

#[test]
fn derived_date_sum_cum_merges_time_periods() {
    let value = MockDerivedDateSumCum {
        weekindex: MockSumCum {
            sum: MockSumVec(MockVec::new("metric_sum", Index::WeekIndex)),
            cumulative: MockCumulativeVec(MockVec::new("metric_cumulative", Index::WeekIndex)),
        },
        monthindex: MockSumCum {
            sum: MockSumVec(MockVec::new("metric_sum", Index::MonthIndex)),
            cumulative: MockCumulativeVec(MockVec::new("metric_cumulative", Index::MonthIndex)),
        },
    };

    let tree = value.to_tree_node();

    // Each SumCum produces { sum, cumulative }
    // Merge lifts from weekindex and monthindex
    // Same keys merge → { sum: Leaf(Week+Month), cumulative: Leaf(Week+Month) }
    match &tree {
        TreeNode::Branch(map) => {
            assert_eq!(map.len(), 2, "Expected sum, cumulative. Got: {:?}", map.keys().collect::<Vec<_>>());

            let sum_indexes = get_leaf_indexes(map.get("sum").unwrap()).unwrap();
            assert!(sum_indexes.contains(&Index::WeekIndex));
            assert!(sum_indexes.contains(&Index::MonthIndex));

            let cum_indexes = get_leaf_indexes(map.get("cumulative").unwrap()).unwrap();
            assert!(cum_indexes.contains(&Index::WeekIndex));
            assert!(cum_indexes.contains(&Index::MonthIndex));
        }
        _ => panic!("Expected branch"),
    }
}

// ============================================================================
// Case 7: ComputedBlockSumCum pattern (base + rest with flatten)
// ============================================================================
// ComputedBlockSumCum<T> (merge):
//   - height: wrap="base"
//   - rest: DerivedComputedBlockSumCum with flatten
//     - height_cumulative: CumulativeVec
//     - dateindex: SumCum
//     - dates: DerivedDateSumCum with flatten
//     - difficultyepoch: SumCum

#[derive(Traversable)]
#[traversable(merge)]
struct MockDerivedComputedBlockSumCum {
    pub height_cumulative: MockCumulativeVec,
    pub dateindex: MockSumCum,
    #[traversable(flatten)]
    pub dates: MockDerivedDateSumCum,
}

#[derive(Traversable)]
#[traversable(merge)]
struct MockComputedBlockSumCum {
    #[traversable(wrap = "base")]
    pub height: MockVec,
    #[traversable(flatten)]
    pub rest: MockDerivedComputedBlockSumCum,
}

#[test]
fn computed_block_sum_cum_base_sum_cumulative() {
    let value = MockComputedBlockSumCum {
        height: MockVec::new("metric", Index::Height),
        rest: MockDerivedComputedBlockSumCum {
            height_cumulative: MockCumulativeVec(MockVec::new("metric_cumulative", Index::Height)),
            dateindex: MockSumCum {
                sum: MockSumVec(MockVec::new("metric_sum", Index::DateIndex)),
                cumulative: MockCumulativeVec(MockVec::new("metric_cumulative", Index::DateIndex)),
            },
            dates: MockDerivedDateSumCum {
                weekindex: MockSumCum {
                    sum: MockSumVec(MockVec::new("metric_sum", Index::WeekIndex)),
                    cumulative: MockCumulativeVec(MockVec::new("metric_cumulative", Index::WeekIndex)),
                },
                monthindex: MockSumCum {
                    sum: MockSumVec(MockVec::new("metric_sum", Index::MonthIndex)),
                    cumulative: MockCumulativeVec(MockVec::new("metric_cumulative", Index::MonthIndex)),
                },
            },
        },
    };

    let tree = value.to_tree_node();

    // Expected: { base, sum, cumulative }
    match &tree {
        TreeNode::Branch(map) => {
            assert_eq!(map.len(), 3, "Expected base, sum, cumulative. Got: {:?}", map.keys().collect::<Vec<_>>());

            // base: only Height
            let base_indexes = get_leaf_indexes(map.get("base").unwrap()).unwrap();
            assert!(base_indexes.contains(&Index::Height));
            assert_eq!(base_indexes.len(), 1);

            // sum: DateIndex, WeekIndex, MonthIndex (NOT Height)
            let sum_indexes = get_leaf_indexes(map.get("sum").unwrap()).unwrap();
            assert!(!sum_indexes.contains(&Index::Height));
            assert!(sum_indexes.contains(&Index::DateIndex));
            assert!(sum_indexes.contains(&Index::WeekIndex));
            assert!(sum_indexes.contains(&Index::MonthIndex));

            // cumulative: Height + all time indexes
            let cum_indexes = get_leaf_indexes(map.get("cumulative").unwrap()).unwrap();
            assert!(cum_indexes.contains(&Index::Height));
            assert!(cum_indexes.contains(&Index::DateIndex));
            assert!(cum_indexes.contains(&Index::WeekIndex));
            assert!(cum_indexes.contains(&Index::MonthIndex));
        }
        _ => panic!("Expected branch"),
    }
}

// ============================================================================
// Case 8: ValueBlockSumCum pattern (denominations with nested aggregations)
// ============================================================================
// ValueBlockSumCum (no merge):
//   - sats: ComputedBlockSumCum (merge) → { base, sum, cumulative }
//   - bitcoin: ComputedBlockSumCum (merge) → { base, sum, cumulative }
//   - dollars: Option<ComputedBlockSumCum> (merge) → { base, sum, cumulative }

#[derive(Traversable)]
struct MockValueBlockSumCum {
    pub sats: MockComputedBlockSumCum,
    pub bitcoin: MockComputedBlockSumCum,
    pub dollars: Option<MockComputedBlockSumCum>,
}

#[test]
fn value_block_sum_cum_denominations_with_aggregations() {
    let make_computed = |prefix: &str| MockComputedBlockSumCum {
        height: MockVec::new(prefix, Index::Height),
        rest: MockDerivedComputedBlockSumCum {
            height_cumulative: MockCumulativeVec(MockVec::new(&format!("{prefix}_cumulative"), Index::Height)),
            dateindex: MockSumCum {
                sum: MockSumVec(MockVec::new(&format!("{prefix}_sum"), Index::DateIndex)),
                cumulative: MockCumulativeVec(MockVec::new(&format!("{prefix}_cumulative"), Index::DateIndex)),
            },
            dates: MockDerivedDateSumCum {
                weekindex: MockSumCum {
                    sum: MockSumVec(MockVec::new(&format!("{prefix}_sum"), Index::WeekIndex)),
                    cumulative: MockCumulativeVec(MockVec::new(&format!("{prefix}_cumulative"), Index::WeekIndex)),
                },
                monthindex: MockSumCum {
                    sum: MockSumVec(MockVec::new(&format!("{prefix}_sum"), Index::MonthIndex)),
                    cumulative: MockCumulativeVec(MockVec::new(&format!("{prefix}_cumulative"), Index::MonthIndex)),
                },
            },
        },
    };

    let value = MockValueBlockSumCum {
        sats: make_computed("metric"),
        bitcoin: make_computed("metric_btc"),
        dollars: Some(make_computed("metric_usd")),
    };

    let tree = value.to_tree_node();

    // Outer has no merge → denominations as branches
    // Each inner has merge → { base, sum, cumulative }
    match &tree {
        TreeNode::Branch(map) => {
            assert_eq!(map.len(), 3);

            for denom in ["sats", "bitcoin", "dollars"] {
                match map.get(denom) {
                    Some(TreeNode::Branch(inner)) => {
                        assert_eq!(inner.len(), 3, "Expected base, sum, cumulative for {denom}");
                        assert!(inner.contains_key("base"));
                        assert!(inner.contains_key("sum"));
                        assert!(inner.contains_key("cumulative"));
                    }
                    _ => panic!("Expected branch for {denom}"),
                }
            }
        }
        _ => panic!("Expected branch"),
    }
}

// ============================================================================
// Case 9: Flatten of collapsed Leaf (edge case)
// ============================================================================
// When flatten encounters a Leaf (from collapsed inner type),
// it inserts with field name as key. Merge on outer lifts wrapped values.

#[derive(Traversable)]
#[traversable(merge)]
struct MockFlattenCollapsedLeaf {
    #[traversable(wrap = "base")]
    pub primary: MockVec,
    #[traversable(flatten)]
    pub collapsed: MockDerivedDateLast, // This collapses to Leaf
}

#[test]
fn flatten_collapsed_leaf_uses_field_name() {
    let value = MockFlattenCollapsedLeaf {
        primary: MockVec::new("metric", Index::DateIndex),
        collapsed: MockDerivedDateLast {
            weekindex: MockLazyLast(MockVec::new("other_metric", Index::WeekIndex)),
            monthindex: MockLazyLast(MockVec::new("other_metric", Index::MonthIndex)),
            yearindex: MockLazyLast(MockVec::new("other_metric", Index::YearIndex)),
        },
    };

    let tree = value.to_tree_node();

    // With merge on outer:
    // - primary with wrap="base" → { base: Leaf } → merge lifts to base: Leaf
    // - collapsed (already merged to Leaf) → flatten inserts as "collapsed": Leaf
    // Result: { base: Leaf, collapsed: Leaf }
    match &tree {
        TreeNode::Branch(map) => {
            assert_eq!(map.len(), 2, "Expected base, collapsed. Got: {:?}", map.keys().collect::<Vec<_>>());
            assert!(map.contains_key("base"));
            assert!(map.contains_key("collapsed"));

            // Verify collapsed is a Leaf with the right indexes
            match map.get("collapsed") {
                Some(TreeNode::Leaf(l)) => {
                    assert_eq!(l.name(), "other_metric");
                    let indexes = l.indexes();
                    assert!(indexes.contains(&Index::WeekIndex));
                    assert!(indexes.contains(&Index::MonthIndex));
                    assert!(indexes.contains(&Index::YearIndex));
                }
                _ => panic!("Expected collapsed to be Leaf"),
            }
        }
        _ => panic!("Expected branch"),
    }
}

// ============================================================================
// Case 10: Rename attribute
// ============================================================================

#[derive(Traversable)]
struct MockRename {
    #[traversable(rename = "custom_name")]
    pub field: MockVec,
}

#[test]
fn rename_attribute_changes_key() {
    let value = MockRename {
        field: MockVec::new("metric", Index::Height),
    };

    let tree = value.to_tree_node();

    match &tree {
        TreeNode::Branch(map) => {
            assert_eq!(map.len(), 1);
            assert!(map.contains_key("custom_name"));
            assert!(!map.contains_key("field"));
        }
        _ => panic!("Expected branch"),
    }
}

// ============================================================================
// Case 11: Skip attribute
// ============================================================================

#[derive(Traversable)]
struct MockSkip {
    pub included: MockVec,
    #[traversable(skip)]
    pub skipped: MockVec,
}

#[test]
fn skip_attribute_excludes_field() {
    let value = MockSkip {
        included: MockVec::new("metric", Index::Height),
        skipped: MockVec::new("should_not_appear", Index::Height),
    };

    let tree = value.to_tree_node();

    match &tree {
        TreeNode::Branch(map) => {
            assert_eq!(map.len(), 1);
            assert!(map.contains_key("included"));
            assert!(!map.contains_key("skipped"));
        }
        _ => panic!("Expected branch"),
    }
}

// ============================================================================
// Case 12: Transparent attribute (single-field delegation)
// ============================================================================

#[derive(Traversable)]
#[traversable(transparent)]
struct MockTransparent {
    pub inner: MockVec,
}

#[test]
fn transparent_delegates_to_inner() {
    let value = MockTransparent {
        inner: MockVec::new("metric", Index::Height),
    };

    let tree = value.to_tree_node();

    // Should produce Leaf directly, not Branch { inner: Leaf }
    match &tree {
        TreeNode::Leaf(l) => {
            assert_eq!(l.name(), "metric");
        }
        _ => panic!("Expected Leaf (transparent delegation)"),
    }
}

// ============================================================================
// Case 13: ValueBlockFull pattern (Full stats with multiple denominations)
// ============================================================================
// ValueBlockFull pattern:
//   - sats: ComputedBlockFull<Sats> (merge)
//   - bitcoin: LazyBlockFull<Bitcoin> (merge)
//   - dollars: Option<ComputedBlockFull<Dollars>> (merge)
//
// Each ComputedBlockFull (merge):
//   - height: wrap="base"
//   - rest: DerivedComputedBlockFull (merge, flatten)
//     - height_cumulative: CumulativeVec
//     - dateindex: Full (distribution + sum_cum)
//     - dates: DerivedDateFull (merge, flatten)
//     - difficultyepoch: LazyFull

// Mock for wrap="avg"
#[derive(Traversable)]
#[traversable(wrap = "avg")]
struct MockAvgVec(MockVec);

// Mock for wrap="min"
#[derive(Traversable)]
#[traversable(wrap = "min")]
struct MockMinVec(MockVec);

// Mock for wrap="max"
#[derive(Traversable)]
#[traversable(wrap = "max")]
struct MockMaxVec(MockVec);

// Mock for wrap="median"
#[derive(Traversable)]
#[traversable(wrap = "median")]
struct MockMedianVec(MockVec);

// MinMax struct
#[derive(Traversable)]
struct MockMinMax {
    #[traversable(flatten)]
    pub min: MockMinVec,
    #[traversable(flatten)]
    pub max: MockMaxVec,
}

// Percentiles struct (simplified to just median for test)
#[derive(Traversable)]
struct MockPercentiles {
    pub median: MockMedianVec,
}

// Distribution = average + minmax + percentiles
#[derive(Traversable)]
struct MockDistribution {
    pub average: MockAvgVec,
    #[traversable(flatten)]
    pub minmax: MockMinMax,
    pub percentiles: MockPercentiles,
}

// Full = Distribution + SumCum
#[derive(Traversable)]
struct MockFull {
    pub distribution: MockDistribution,
    pub sum_cum: MockSumCum,
}

// LazyFull - all flattened
#[derive(Traversable)]
struct MockLazyFull {
    #[traversable(flatten)]
    pub avg: MockAvgVec,
    #[traversable(flatten)]
    pub min: MockMinVec,
    #[traversable(flatten)]
    pub max: MockMaxVec,
    #[traversable(flatten)]
    pub sum: MockSumVec,
    #[traversable(flatten)]
    pub cumulative: MockCumulativeVec,
}

// DerivedDateFull (merge) - time periods with LazyFull
#[derive(Traversable)]
#[traversable(merge)]
struct MockDerivedDateFull {
    pub weekindex: MockLazyFull,
    pub monthindex: MockLazyFull,
}

// DerivedComputedBlockFull (merge)
#[derive(Traversable)]
#[traversable(merge)]
struct MockDerivedComputedBlockFull {
    pub height_cumulative: MockCumulativeVec,
    pub dateindex: MockFull,
    #[traversable(flatten)]
    pub dates: MockDerivedDateFull,
    pub difficultyepoch: MockLazyFull,
}

// ComputedBlockFull (merge)
#[derive(Traversable)]
#[traversable(merge)]
struct MockComputedBlockFull {
    #[traversable(wrap = "base")]
    pub height: MockVec,
    #[traversable(flatten)]
    pub rest: MockDerivedComputedBlockFull,
}

// ValueBlockFull - no merge (denominations as branches)
#[derive(Traversable)]
struct MockValueBlockFull {
    pub sats: MockComputedBlockFull,
    pub bitcoin: MockComputedBlockFull,
    pub dollars: Option<MockComputedBlockFull>,
}

fn make_lazy_full(name: &str, index: Index) -> MockLazyFull {
    MockLazyFull {
        avg: MockAvgVec(MockVec::new(&format!("{name}_avg"), index)),
        min: MockMinVec(MockVec::new(&format!("{name}_min"), index)),
        max: MockMaxVec(MockVec::new(&format!("{name}_max"), index)),
        sum: MockSumVec(MockVec::new(&format!("{name}_sum"), index)),
        cumulative: MockCumulativeVec(MockVec::new(&format!("{name}_cumulative"), index)),
    }
}

fn make_computed_block_full(name: &str) -> MockComputedBlockFull {
    MockComputedBlockFull {
        height: MockVec::new(name, Index::Height),
        rest: MockDerivedComputedBlockFull {
            height_cumulative: MockCumulativeVec(MockVec::new(&format!("{name}_cumulative"), Index::Height)),
            dateindex: MockFull {
                distribution: MockDistribution {
                    average: MockAvgVec(MockVec::new(&format!("{name}_avg"), Index::DateIndex)),
                    minmax: MockMinMax {
                        min: MockMinVec(MockVec::new(&format!("{name}_min"), Index::DateIndex)),
                        max: MockMaxVec(MockVec::new(&format!("{name}_max"), Index::DateIndex)),
                    },
                    percentiles: MockPercentiles {
                        median: MockMedianVec(MockVec::new(&format!("{name}_median"), Index::DateIndex)),
                    },
                },
                sum_cum: MockSumCum {
                    sum: MockSumVec(MockVec::new(&format!("{name}_sum"), Index::DateIndex)),
                    cumulative: MockCumulativeVec(MockVec::new(&format!("{name}_cumulative"), Index::DateIndex)),
                },
            },
            dates: MockDerivedDateFull {
                weekindex: make_lazy_full(name, Index::WeekIndex),
                monthindex: make_lazy_full(name, Index::MonthIndex),
            },
            difficultyepoch: make_lazy_full(name, Index::DifficultyEpoch),
        },
    }
}

#[test]
fn value_block_full_denominations_as_branches() {
    let value = MockValueBlockFull {
        sats: make_computed_block_full("metric"),
        bitcoin: make_computed_block_full("metric_btc"),
        dollars: Some(make_computed_block_full("metric_usd")),
    };

    let tree = value.to_tree_node();

    // Without merge on outer, denominations are branches
    match &tree {
        TreeNode::Branch(map) => {
            assert_eq!(map.len(), 3, "Expected sats, bitcoin, dollars");
            assert!(map.contains_key("sats"));
            assert!(map.contains_key("bitcoin"));
            assert!(map.contains_key("dollars"));

            // Each denomination should have deeply merged inner structure
            // The merge recursively flattens: dateindex, difficultyepoch become flat metric keys
            for denom in ["sats", "bitcoin", "dollars"] {
                match map.get(denom) {
                    Some(TreeNode::Branch(inner)) => {
                        // Inner merge produces flat structure:
                        // base, cumulative, avg, average, min, max, sum, minmax, percentiles
                        assert!(inner.contains_key("base"), "{denom} missing base");
                        assert!(inner.contains_key("cumulative"), "{denom} missing cumulative");
                        assert!(inner.contains_key("avg"), "{denom} missing avg");
                        assert!(inner.contains_key("sum"), "{denom} missing sum");
                        // dateindex and difficultyepoch are merged in, not separate branches
                        assert!(!inner.contains_key("dateindex"), "{denom} should NOT have dateindex branch");
                        assert!(!inner.contains_key("difficultyepoch"), "{denom} should NOT have difficultyepoch branch");
                    }
                    _ => panic!("Expected branch for {denom}"),
                }
            }
        }
        _ => panic!("Expected branch at top level"),
    }
}

