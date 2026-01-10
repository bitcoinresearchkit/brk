use std::collections::{BTreeMap, BTreeSet};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::Index;

/// Leaf node containing metric metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub struct MetricLeaf {
    /// The metric name/identifier
    pub name: String,
    /// The Rust type (e.g., "Sats", "StoredF64")
    pub kind: String,
    /// Available indexes for this metric
    pub indexes: BTreeSet<Index>,
}

impl MetricLeaf {
    pub fn new(name: String, kind: String, indexes: BTreeSet<Index>) -> Self {
        Self {
            name,
            kind,
            indexes,
        }
    }

    /// Merge another leaf's indexes into this one (union)
    pub fn merge_indexes(&mut self, other: &MetricLeaf) {
        self.indexes.extend(other.indexes.iter().copied());
    }
}

/// MetricLeaf with JSON Schema for client generation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MetricLeafWithSchema {
    /// The core metric metadata
    #[serde(flatten)]
    pub leaf: MetricLeaf,
    /// JSON Schema type (e.g., "integer", "number", "string", "boolean", "array", "object")
    #[serde(rename = "type")]
    pub openapi_type: String,
    /// JSON Schema for the value type
    #[serde(skip)]
    pub schema: serde_json::Value,
}

/// Extract JSON type from a schema, following $ref if needed.
pub fn extract_json_type(schema: &serde_json::Value) -> String {
    // Direct type field
    if let Some(t) = schema.get("type").and_then(|v| v.as_str()) {
        return t.to_string();
    }

    // Handle $ref - look up in definitions
    if let Some(ref_path) = schema.get("$ref").and_then(|v| v.as_str()) {
        if let Some(def_name) = ref_path.rsplit('/').next() {
            // Check both "$defs" (draft 2020-12) and "definitions" (older drafts)
            for defs_key in &["$defs", "definitions"] {
                if let Some(defs) = schema.get(defs_key) {
                    if let Some(def) = defs.get(def_name) {
                        return extract_json_type(def);
                    }
                }
            }
        }
    }

    // Handle allOf with single element
    if let Some(all_of) = schema.get("allOf").and_then(|v| v.as_array()) {
        if all_of.len() == 1 {
            return extract_json_type(&all_of[0]);
        }
    }

    "object".to_string()
}

impl MetricLeafWithSchema {
    pub fn new(leaf: MetricLeaf, schema: serde_json::Value) -> Self {
        let openapi_type = extract_json_type(&schema);
        Self {
            leaf,
            openapi_type,
            schema,
        }
    }

    /// The OpenAPI/JSON Schema type
    pub fn openapi_type(&self) -> &str {
        &self.openapi_type
    }

    /// The metric name/identifier
    pub fn name(&self) -> &str {
        &self.leaf.name
    }

    /// The Rust type (e.g., "Sats", "StoredF64")
    pub fn kind(&self) -> &str {
        &self.leaf.kind
    }

    /// Available indexes for this metric
    pub fn indexes(&self) -> &BTreeSet<Index> {
        &self.leaf.indexes
    }

    /// Check if this leaf refers to the same metric as another
    pub fn is_same_metric(&self, other: &MetricLeafWithSchema) -> bool {
        self.leaf.name == other.leaf.name
    }

    /// Merge another leaf's indexes into this one (union)
    pub fn merge_indexes(&mut self, other: &MetricLeafWithSchema) {
        self.leaf.merge_indexes(&other.leaf);
    }
}

impl PartialEq for MetricLeafWithSchema {
    fn eq(&self, other: &Self) -> bool {
        self.leaf == other.leaf
    }
}

impl Eq for MetricLeafWithSchema {}

/// Hierarchical tree node for organizing metrics into categories
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum TreeNode {
    /// Branch node containing subcategories
    Branch(BTreeMap<String, TreeNode>),
    /// Leaf node containing metric metadata with schema
    Leaf(MetricLeafWithSchema),
}

const BASE: &str = "base";

impl TreeNode {
    pub fn is_empty(&self) -> bool {
        if let Self::Branch(tree) = self {
            tree.is_empty()
        } else {
            false
        }
    }

    pub fn as_mut_branch(&mut self) -> &mut BTreeMap<String, TreeNode> {
        match self {
            Self::Branch(b) => b,
            _ => panic!(),
        }
    }

    /// Wraps a node in a Branch with the given key.
    /// Used by #[traversable(wrap = "...")] to produce Branch { key: inner }.
    pub fn wrap(key: &str, inner: Self) -> Self {
        let mut map = BTreeMap::new();
        map.insert(key.to_string(), inner);
        Self::Branch(map)
    }

    /// Merges all first-level branches into a single flattened structure (consuming version).
    /// Direct leaves use their key (use #[traversable(rename = "...")] to control).
    /// Branch children are lifted with their keys.
    /// If all resulting children are leaves with the same metric name, collapses to a single leaf.
    /// Returns None if conflicts are found (same key with incompatible values).
    pub fn merge_branches(self) -> Option<Self> {
        let Self::Branch(tree) = self else {
            return Some(self);
        };

        let mut merged: BTreeMap<String, TreeNode> = BTreeMap::new();

        for (key, node) in tree {
            match node {
                Self::Leaf(leaf) => {
                    // Direct leaves use their key (which may be renamed via attribute)
                    Self::merge_node(&mut merged, key, Self::Leaf(leaf))?;
                }
                Self::Branch(inner) => {
                    // Lift children from branches with their keys
                    for (inner_key, inner_node) in inner {
                        Self::merge_node(&mut merged, inner_key, inner_node)?;
                    }
                }
            }
        }

        // If all children are leaves with the same metric name, collapse into single leaf
        Some(Self::try_collapse_same_name_leaves(merged))
    }

    /// If all entries in the map are leaves with the same metric name,
    /// collapse them into a single leaf with merged indexes.
    fn try_collapse_same_name_leaves(map: BTreeMap<String, TreeNode>) -> Self {
        if map.is_empty() {
            return Self::Branch(map);
        }

        // Check if all entries are leaves with the same name
        let mut first_leaf: Option<&MetricLeafWithSchema> = None;
        let mut merged_indexes = BTreeSet::new();

        for node in map.values() {
            match node {
                Self::Leaf(leaf) => {
                    if let Some(first) = &first_leaf {
                        if leaf.name() != first.name() {
                            // Different names - can't collapse
                            return Self::Branch(map);
                        }
                    } else {
                        first_leaf = Some(leaf);
                    }
                    merged_indexes.extend(leaf.indexes().iter().copied());
                }
                Self::Branch(_) => {
                    // Has non-leaf entries - can't collapse
                    return Self::Branch(map);
                }
            }
        }

        // All entries were leaves with the same name
        let first = first_leaf.unwrap();
        Self::Leaf(MetricLeafWithSchema::new(
            MetricLeaf::new(
                first.name().to_string(),
                first.kind().to_string(),
                merged_indexes,
            ),
            first.schema.clone(),
        ))
    }

    /// Merges a node into the target map at the given key (consuming version).
    /// Returns None if there's a conflict.
    pub fn merge_node(
        target: &mut BTreeMap<String, TreeNode>,
        key: String,
        node: TreeNode,
    ) -> Option<()> {
        match target.get_mut(&key) {
            None => {
                target.insert(key, node);
                Some(())
            }
            Some(existing) => {
                match (existing, node) {
                    (Self::Leaf(a), Self::Leaf(b)) if a.is_same_metric(&b) => {
                        a.merge_indexes(&b);
                        Some(())
                    }
                    (Self::Leaf(a), Self::Leaf(b)) => {
                        eprintln!("Conflict: Different leaf values for key '{key}'");
                        eprintln!("  Existing: {a:?}");
                        eprintln!("  New: {b:?}");
                        None
                    }
                    (existing @ Self::Leaf(_), Self::Branch(branch)) => {
                        let Self::Leaf(leaf) =
                            std::mem::replace(existing, Self::Branch(BTreeMap::new()))
                        else {
                            unreachable!()
                        };
                        let Self::Branch(new_branch) = existing else {
                            unreachable!()
                        };
                        new_branch.insert(BASE.to_string(), Self::Leaf(leaf));

                        for (k, v) in branch {
                            Self::merge_node(new_branch, k, v)?;
                        }
                        Some(())
                    }
                    (Self::Branch(existing_branch), Self::Leaf(leaf)) => {
                        Self::merge_node(existing_branch, BASE.to_string(), Self::Leaf(leaf))?;
                        Some(())
                    }
                    // Both branches: merge recursively
                    (Self::Branch(existing_branch), Self::Branch(new_inner)) => {
                        for (k, v) in new_inner {
                            Self::merge_node(existing_branch, k, v)?;
                        }
                        Some(())
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leaf(name: &str, index: Index) -> TreeNode {
        TreeNode::Leaf(MetricLeafWithSchema {
            leaf: MetricLeaf {
                name: name.to_string(),
                kind: "TestType".to_string(),
                indexes: BTreeSet::from([index]),
            },
            openapi_type: "object".to_string(),
            schema: serde_json::Value::Null,
        })
    }

    fn branch(children: Vec<(&str, TreeNode)>) -> TreeNode {
        TreeNode::Branch(
            children
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        )
    }

    fn get_leaf_indexes(node: &TreeNode) -> Option<&BTreeSet<Index>> {
        match node {
            TreeNode::Leaf(l) => Some(&l.leaf.indexes),
            _ => None,
        }
    }

    // ========== Leaf passthrough ==========

    #[test]
    fn merge_leaf_passthrough() {
        let tree = leaf("metric", Index::Height);
        let merged = tree.merge_branches().unwrap();
        assert!(matches!(merged, TreeNode::Leaf(_)));
    }

    #[test]
    fn merge_empty_branch() {
        let tree = branch(vec![]);
        let merged = tree.merge_branches().unwrap();
        match merged {
            TreeNode::Branch(map) => assert!(map.is_empty()),
            _ => panic!("Expected empty branch"),
        }
    }

    // ========== Direct leaves keep their keys ==========

    #[test]
    fn merge_direct_leaves_keep_keys() {
        // Direct leaves with different keys stay separate
        let tree = branch(vec![
            ("sum", leaf("metric_sum", Index::Height)),
            ("cumulative", leaf("metric_cumulative", Index::Height)),
        ]);
        let merged = tree.merge_branches().unwrap();

        match merged {
            TreeNode::Branch(map) => {
                assert_eq!(map.len(), 2);
                assert!(map.contains_key("sum"));
                assert!(map.contains_key("cumulative"));
            }
            _ => panic!("Expected branch"),
        }
    }

    // ========== Branch lifting ==========

    #[test]
    fn merge_lifts_branch_children() {
        // Branch children are lifted to top level with their keys
        let tree = branch(vec![(
            "weekindex",
            branch(vec![
                ("sum", leaf("metric_sum", Index::WeekIndex)),
                ("cumulative", leaf("metric_cumulative", Index::WeekIndex)),
            ]),
        )]);
        let merged = tree.merge_branches().unwrap();

        match merged {
            TreeNode::Branch(map) => {
                assert!(map.contains_key("sum"));
                assert!(map.contains_key("cumulative"));
                assert!(!map.contains_key("weekindex")); // Parent key gone
            }
            _ => panic!("Expected branch"),
        }
    }

    #[test]
    fn merge_multiple_branches_combines_indexes() {
        // Multiple branches with same child keys → indexes are merged
        let tree = branch(vec![
            (
                "weekindex",
                branch(vec![
                    ("sum", leaf("metric_sum", Index::WeekIndex)),
                    ("cumulative", leaf("metric_cumulative", Index::WeekIndex)),
                ]),
            ),
            (
                "monthindex",
                branch(vec![
                    ("sum", leaf("metric_sum", Index::MonthIndex)),
                    ("cumulative", leaf("metric_cumulative", Index::MonthIndex)),
                ]),
            ),
        ]);
        let merged = tree.merge_branches().unwrap();

        match merged {
            TreeNode::Branch(map) => {
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

    // ========== Mixed leaves and branches ==========

    #[test]
    fn merge_leaf_merges_with_lifted_branch_child() {
        // Direct leaf with key "cumulative" merges with lifted "cumulative" from branch
        // This simulates: height_cumulative (renamed) + dateindex branch
        let tree = branch(vec![
            ("cumulative", leaf("metric_cumulative", Index::Height)),
            (
                "dateindex",
                branch(vec![
                    ("sum", leaf("metric_sum", Index::DateIndex)),
                    ("cumulative", leaf("metric_cumulative", Index::DateIndex)),
                ]),
            ),
        ]);
        let merged = tree.merge_branches().unwrap();

        match merged {
            TreeNode::Branch(map) => {
                assert_eq!(map.len(), 2);

                // cumulative merged: Height + DateIndex
                let cum_indexes = get_leaf_indexes(map.get("cumulative").unwrap()).unwrap();
                assert!(cum_indexes.contains(&Index::Height));
                assert!(cum_indexes.contains(&Index::DateIndex));

                // sum only has DateIndex
                let sum_indexes = get_leaf_indexes(map.get("sum").unwrap()).unwrap();
                assert!(sum_indexes.contains(&Index::DateIndex));
                assert!(!sum_indexes.contains(&Index::Height));
            }
            _ => panic!("Expected branch"),
        }
    }

    #[test]
    fn merge_derived_computed_block_sum_cum_pattern() {
        // Simulates DerivedComputedBlockSumCum:
        // - height_cumulative (renamed to "cumulative") → direct leaf at Height
        // - dateindex → branch with sum/cumulative at DateIndex
        // - weekindex (flattened from dates) → branch with sum/cumulative at WeekIndex
        // - difficultyepoch → branch with sum/cumulative at DifficultyEpoch
        let tree = branch(vec![
            ("cumulative", leaf("metric_cumulative", Index::Height)),
            (
                "dateindex",
                branch(vec![
                    ("sum", leaf("metric_sum", Index::DateIndex)),
                    ("cumulative", leaf("metric_cumulative", Index::DateIndex)),
                ]),
            ),
            (
                "weekindex",
                branch(vec![
                    ("sum", leaf("metric_sum", Index::WeekIndex)),
                    ("cumulative", leaf("metric_cumulative", Index::WeekIndex)),
                ]),
            ),
            (
                "difficultyepoch",
                branch(vec![
                    ("sum", leaf("metric_sum", Index::DifficultyEpoch)),
                    (
                        "cumulative",
                        leaf("metric_cumulative", Index::DifficultyEpoch),
                    ),
                ]),
            ),
        ]);

        let merged = tree.merge_branches().unwrap();

        match merged {
            TreeNode::Branch(map) => {
                assert_eq!(map.len(), 2);

                // sum: DateIndex, WeekIndex, DifficultyEpoch (NOT Height)
                let sum_indexes = get_leaf_indexes(map.get("sum").unwrap()).unwrap();
                assert!(!sum_indexes.contains(&Index::Height));
                assert!(sum_indexes.contains(&Index::DateIndex));
                assert!(sum_indexes.contains(&Index::WeekIndex));
                assert!(sum_indexes.contains(&Index::DifficultyEpoch));

                // cumulative: Height, DateIndex, WeekIndex, DifficultyEpoch
                let cum_indexes = get_leaf_indexes(map.get("cumulative").unwrap()).unwrap();
                assert!(cum_indexes.contains(&Index::Height));
                assert!(cum_indexes.contains(&Index::DateIndex));
                assert!(cum_indexes.contains(&Index::WeekIndex));
                assert!(cum_indexes.contains(&Index::DifficultyEpoch));
            }
            _ => panic!("Expected branch"),
        }
    }

    // ========== Conflict detection ==========

    #[test]
    fn merge_conflict_from_lifted_branches() {
        // Two branches lifting children with same key but different metric names → conflict
        let tree = branch(vec![
            ("a", branch(vec![("data", leaf("metric_a", Index::Height))])),
            (
                "b",
                branch(vec![("data", leaf("metric_b", Index::DateIndex))]),
            ),
        ]);
        let result = tree.merge_branches();
        assert!(result.is_none(), "Should detect conflict");
    }

    #[test]
    fn merge_no_conflict_same_metric_different_indexes() {
        // Same key, same metric name, different indexes → merges indexes → collapses to Leaf
        let tree = branch(vec![
            (
                "a",
                branch(vec![("sum", leaf("metric_sum", Index::Height))]),
            ),
            (
                "b",
                branch(vec![("sum", leaf("metric_sum", Index::DateIndex))]),
            ),
        ]);
        let result = tree.merge_branches();
        assert!(result.is_some(), "Should merge successfully");

        let merged = result.unwrap();
        match merged {
            TreeNode::Leaf(leaf) => {
                assert_eq!(leaf.name(), "metric_sum");
                let indexes = leaf.indexes();
                assert!(indexes.contains(&Index::Height));
                assert!(indexes.contains(&Index::DateIndex));
            }
            _ => panic!("Expected collapsed Leaf"),
        }
    }

    // ========== Nested branches ==========

    #[test]
    fn merge_nested_branches_flattens_one_level() {
        // Merge only flattens one level - nested branches stay as branches
        let tree = branch(vec![(
            "outer",
            branch(vec![(
                "inner",
                branch(vec![("leaf", leaf("metric", Index::Height))]),
            )]),
        )]);
        let merged = tree.merge_branches().unwrap();

        // "inner" is lifted to top level but stays as a branch
        match merged {
            TreeNode::Branch(map) => {
                assert!(map.contains_key("inner"));
                assert!(matches!(map.get("inner"), Some(TreeNode::Branch(_))));
            }
            _ => panic!("Expected branch"),
        }
    }

    // ========== Collapse same-name leaves ==========

    #[test]
    fn collapse_direct_leaf_with_lifted_branches_same_name() {
        // ComputedVecsDateLast pattern:
        // - dateindex: direct leaf (field name as key)
        // - rest (flattened): DerivedDateLast → branches with "last" children
        // All leaves have same metric name → collapse to single Leaf
        let tree = branch(vec![
            // Direct leaf from dateindex field (no wrap attribute)
            ("dateindex", leaf("1m_block_count", Index::DateIndex)),
            // Flattened from rest: DerivedDateLast
            (
                "weekindex",
                branch(vec![("last", leaf("1m_block_count", Index::WeekIndex))]),
            ),
            (
                "monthindex",
                branch(vec![("last", leaf("1m_block_count", Index::MonthIndex))]),
            ),
        ]);

        let merged = tree.merge_branches().unwrap();

        // All leaves have same name "1m_block_count" → collapses to single Leaf
        match &merged {
            TreeNode::Leaf(leaf) => {
                assert_eq!(leaf.name(), "1m_block_count");
                let indexes = leaf.indexes();
                assert!(indexes.contains(&Index::DateIndex));
                assert!(indexes.contains(&Index::WeekIndex));
                assert!(indexes.contains(&Index::MonthIndex));
            }
            TreeNode::Branch(map) => {
                panic!(
                    "Expected collapsed leaf, got branch: {:?}",
                    map.keys().collect::<Vec<_>>()
                );
            }
        }
    }

    // ========== Case 1: DerivedDateLast (all same metric name) ==========

    #[test]
    fn case1_derived_date_last() {
        // All leaves have the same metric name, all wrapped as "last"
        // All branches lift to same key → collapses to single Leaf
        let tree = branch(vec![
            (
                "weekindex",
                branch(vec![("last", leaf("1m_block_count", Index::WeekIndex))]),
            ),
            (
                "monthindex",
                branch(vec![("last", leaf("1m_block_count", Index::MonthIndex))]),
            ),
            (
                "quarterindex",
                branch(vec![("last", leaf("1m_block_count", Index::QuarterIndex))]),
            ),
            (
                "yearindex",
                branch(vec![("last", leaf("1m_block_count", Index::YearIndex))]),
            ),
        ]);

        let merged = tree.merge_branches().unwrap();

        match &merged {
            TreeNode::Leaf(leaf) => {
                assert_eq!(leaf.name(), "1m_block_count");
                let indexes = leaf.indexes();
                assert!(indexes.contains(&Index::WeekIndex));
                assert!(indexes.contains(&Index::MonthIndex));
                assert!(indexes.contains(&Index::QuarterIndex));
                assert!(indexes.contains(&Index::YearIndex));
            }
            _ => panic!("Expected collapsed Leaf"),
        }
    }

    // ========== Case 2: SumCum (different aggregations via wrap) ==========

    #[test]
    fn case2_sum_cum() {
        // SumVec/CumulativeVec use wrap to produce branches
        // Multiple time periods, each producing { "sum": Leaf, "cumulative": Leaf }
        // These should merge into { "sum": Leaf(all indexes), "cumulative": Leaf(all indexes) }
        let tree = branch(vec![
            (
                "dateindex",
                branch(vec![
                    ("sum", leaf("metric_sum", Index::DateIndex)),
                    ("cumulative", leaf("metric_cumulative", Index::DateIndex)),
                ]),
            ),
            (
                "weekindex",
                branch(vec![
                    ("sum", leaf("metric_sum", Index::WeekIndex)),
                    ("cumulative", leaf("metric_cumulative", Index::WeekIndex)),
                ]),
            ),
        ]);

        let merged = tree.merge_branches().unwrap();

        // DESIRED: { "sum": Leaf, "cumulative": Leaf } with merged indexes
        match merged {
            TreeNode::Branch(map) => {
                assert_eq!(map.len(), 2);

                let sum_indexes = get_leaf_indexes(map.get("sum").unwrap()).unwrap();
                assert!(sum_indexes.contains(&Index::DateIndex));
                assert!(sum_indexes.contains(&Index::WeekIndex));

                let cum_indexes = get_leaf_indexes(map.get("cumulative").unwrap()).unwrap();
                assert!(cum_indexes.contains(&Index::DateIndex));
                assert!(cum_indexes.contains(&Index::WeekIndex));
            }
            _ => panic!("Expected branch with sum and cumulative"),
        }
    }

    // ========== Case 3: ComputedBlockSum (base + sum) ==========

    #[test]
    fn case3_computed_block_sum() {
        // ComputedBlockSum:
        // - height: wrap="base" → Branch { "base": Leaf(height) }
        // - rest (flatten): DerivedComputedBlockSum → branches with "sum" children
        let tree = branch(vec![
            // height wrapped as "base"
            (
                "height",
                branch(vec![("base", leaf("metric", Index::Height))]),
            ),
            // rest (flattened) produces branches
            (
                "dateindex",
                branch(vec![("sum", leaf("metric_sum", Index::DateIndex))]),
            ),
            (
                "weekindex",
                branch(vec![("sum", leaf("metric_sum", Index::WeekIndex))]),
            ),
        ]);

        let merged = tree.merge_branches().unwrap();

        // DESIRED: { "base": Leaf(height), "sum": Leaf(dateindex, weekindex) }
        match &merged {
            TreeNode::Branch(map) => {
                assert_eq!(
                    map.len(),
                    2,
                    "Expected 2 keys 'base' and 'sum', got: {:?}",
                    map.keys().collect::<Vec<_>>()
                );

                // base should have Height only
                let base_indexes = get_leaf_indexes(map.get("base").unwrap()).unwrap();
                assert!(base_indexes.contains(&Index::Height));
                assert_eq!(base_indexes.len(), 1);

                // sum should have DateIndex and WeekIndex
                let sum_indexes = get_leaf_indexes(map.get("sum").unwrap()).unwrap();
                assert!(!sum_indexes.contains(&Index::Height));
                assert!(sum_indexes.contains(&Index::DateIndex));
                assert!(sum_indexes.contains(&Index::WeekIndex));
            }
            _ => panic!("Expected branch"),
        }
    }

    // ========== Case 4: ComputedBlockLast (base + last) ==========

    #[test]
    fn case4_computed_block_last() {
        // ComputedBlockLast:
        // - height: wrap="base" → Branch { "base": Leaf(height) }
        // - rest (flatten): DerivedComputedBlockLast → branches with "last" children
        let tree = branch(vec![
            // height wrapped as "base"
            (
                "height",
                branch(vec![("base", leaf("metric", Index::Height))]),
            ),
            // rest (flattened) produces branches with "last" key
            (
                "dateindex",
                branch(vec![("last", leaf("metric_last", Index::DateIndex))]),
            ),
            (
                "weekindex",
                branch(vec![("last", leaf("metric_last", Index::WeekIndex))]),
            ),
        ]);

        let merged = tree.merge_branches().unwrap();

        // DESIRED: { "base": Leaf(height), "last": Leaf(dateindex, weekindex) }
        match &merged {
            TreeNode::Branch(map) => {
                assert_eq!(
                    map.len(),
                    2,
                    "Expected 2 keys 'base' and 'last', got: {:?}",
                    map.keys().collect::<Vec<_>>()
                );

                // base should have Height only
                let base_indexes = get_leaf_indexes(map.get("base").unwrap()).unwrap();
                assert!(base_indexes.contains(&Index::Height));
                assert_eq!(base_indexes.len(), 1);

                // last should have DateIndex and WeekIndex
                let last_indexes = get_leaf_indexes(map.get("last").unwrap()).unwrap();
                assert!(!last_indexes.contains(&Index::Height));
                assert!(last_indexes.contains(&Index::DateIndex));
                assert!(last_indexes.contains(&Index::WeekIndex));
            }
            _ => panic!("Expected branch"),
        }
    }

    // ========== Case 5: ComputedBlockFull (most complex) ==========

    #[test]
    fn case5_computed_block_full() {
        // ComputedBlockFull has:
        // - height: wrapped as "base" (raw values, not aggregated)
        // - rest (flatten): DerivedComputedBlockFull {
        //     height_cumulative: CumulativeVec → Branch{"cumulative": Leaf}
        //     dateindex: Full → Branch{avg, min, max, sum, cumulative}
        //     dates (flatten): more aggregation branches
        //   }
        let tree = branch(vec![
            // height wrapped as "base" (raw values at height granularity)
            (
                "height",
                branch(vec![("base", leaf("metric", Index::Height))]),
            ),
            // height_cumulative wrapped as cumulative
            (
                "height_cumulative",
                branch(vec![(
                    "cumulative",
                    leaf("metric_cumulative", Index::Height),
                )]),
            ),
            // dateindex Full
            (
                "dateindex",
                branch(vec![
                    ("average", leaf("metric_average", Index::DateIndex)),
                    ("min", leaf("metric_min", Index::DateIndex)),
                    ("max", leaf("metric_max", Index::DateIndex)),
                    ("sum", leaf("metric_sum", Index::DateIndex)),
                    ("cumulative", leaf("metric_cumulative", Index::DateIndex)),
                ]),
            ),
            // weekindex (from flattened dates)
            (
                "weekindex",
                branch(vec![
                    ("average", leaf("metric_average", Index::WeekIndex)),
                    ("min", leaf("metric_min", Index::WeekIndex)),
                    ("max", leaf("metric_max", Index::WeekIndex)),
                    ("sum", leaf("metric_sum", Index::WeekIndex)),
                    ("cumulative", leaf("metric_cumulative", Index::WeekIndex)),
                ]),
            ),
        ]);

        let merged = tree.merge_branches().unwrap();

        // DESIRED: { base, average, min, max, sum, cumulative } each with merged indexes
        match &merged {
            TreeNode::Branch(map) => {
                assert_eq!(
                    map.len(),
                    6,
                    "Expected 6 keys, got: {:?}",
                    map.keys().collect::<Vec<_>>()
                );

                // base should have Height only
                let base_indexes = get_leaf_indexes(map.get("base").unwrap()).unwrap();
                assert!(base_indexes.contains(&Index::Height));
                assert_eq!(base_indexes.len(), 1);

                // cumulative should include Height (from height_cumulative)
                let cum_indexes = get_leaf_indexes(map.get("cumulative").unwrap()).unwrap();
                assert!(
                    cum_indexes.contains(&Index::Height),
                    "cumulative should include Height"
                );
                assert!(cum_indexes.contains(&Index::DateIndex));
                assert!(cum_indexes.contains(&Index::WeekIndex));

                // average, min, max, sum should have DateIndex and WeekIndex only
                for key in ["average", "min", "max", "sum"] {
                    let indexes = get_leaf_indexes(map.get(key).unwrap()).unwrap();
                    assert!(!indexes.contains(&Index::Height));
                    assert!(indexes.contains(&Index::DateIndex));
                    assert!(indexes.contains(&Index::WeekIndex));
                }
            }
            _ => panic!("Expected branch"),
        }
    }

    // ========== Case 6: LazyDateLast (all branches with same inner key) ==========

    #[test]
    fn case6_lazy_date_last_all_branches_same_key_collapses() {
        // LazyDateLast pattern: All fields are branches with same inner key "last"
        // All leaves have the same metric name → should collapse to single Leaf
        let tree = branch(vec![
            (
                "dateindex",
                branch(vec![("last", leaf("price_200d_sma", Index::DateIndex))]),
            ),
            (
                "weekindex",
                branch(vec![("last", leaf("price_200d_sma", Index::WeekIndex))]),
            ),
            (
                "monthindex",
                branch(vec![("last", leaf("price_200d_sma", Index::MonthIndex))]),
            ),
            (
                "quarterindex",
                branch(vec![("last", leaf("price_200d_sma", Index::QuarterIndex))]),
            ),
            (
                "yearindex",
                branch(vec![("last", leaf("price_200d_sma", Index::YearIndex))]),
            ),
        ]);

        let merged = tree.merge_branches().unwrap();

        // All branches lifted to same "last" key, all same metric name → collapse to Leaf
        match &merged {
            TreeNode::Leaf(leaf) => {
                assert_eq!(leaf.name(), "price_200d_sma");
                let indexes = leaf.indexes();
                assert!(indexes.contains(&Index::DateIndex));
                assert!(indexes.contains(&Index::WeekIndex));
                assert!(indexes.contains(&Index::MonthIndex));
                assert!(indexes.contains(&Index::QuarterIndex));
                assert!(indexes.contains(&Index::YearIndex));
            }
            TreeNode::Branch(map) => {
                panic!(
                    "Expected collapsed Leaf, got Branch with keys: {:?}",
                    map.keys().collect::<Vec<_>>()
                );
            }
        }
    }

    // ========== Case 7: LazyBlockValue ==========
    // LazyBlockValue (no merge):
    //   - sats: LazyVecFrom1 with wrap="sats"
    //   - rest: LazyDerivedBlockValue with flatten
    //     - bitcoin: LazyVecFrom1 (plain field)
    //     - dollars: Option<LazyVecFrom2> (plain field)

    #[test]
    fn case7_lazy_block_value() {
        // Simulates the tree produced by LazyBlockValue
        // sats wrapped, rest flattened with bitcoin/dollars as plain leaves
        let tree = branch(vec![
            // sats with wrap="sats" produces Branch { sats: Leaf }
            (
                "sats",
                branch(vec![("sats", leaf("metric", Index::Height))]),
            ),
            // rest with flatten: LazyDerivedBlockValue fields lifted
            (
                "rest",
                branch(vec![
                    ("bitcoin", leaf("metric_btc", Index::Height)),
                    ("dollars", leaf("metric_usd", Index::Height)),
                ]),
            ),
        ]);

        // After merge_branches: lifts children, flattens rest
        let merged = tree.merge_branches().unwrap();

        match &merged {
            TreeNode::Branch(map) => {
                assert_eq!(
                    map.len(),
                    3,
                    "Expected sats, bitcoin, dollars. Got: {:?}",
                    map.keys().collect::<Vec<_>>()
                );
                assert!(matches!(map.get("sats"), Some(TreeNode::Leaf(_))));
                assert!(matches!(map.get("bitcoin"), Some(TreeNode::Leaf(_))));
                assert!(matches!(map.get("dollars"), Some(TreeNode::Leaf(_))));
            }
            _ => panic!("Expected branch"),
        }
    }

    // ========== Case 8: BinaryBlockSumCum ==========
    // After derive applies all inner merges and flatten, before parent merge:
    // - height wrapped as "base" → { base: Leaf(Height) }
    // - height_cumulative wrapped as "cumulative" → { cumulative: Leaf(Height) }
    // - rest (flatten): children from already-merged inner struct inserted directly
    //
    // The key insight: inner types have merge applied BEFORE flatten lifts them.
    // So rest.to_tree_node() returns merged { sum, cumulative } directly.

    #[test]
    fn case8_binary_block_sum_cum() {
        // Tree AFTER derive applies inner merges, flatten lifts rest's children:
        let tree = branch(vec![
            // height with wrap="base"
            (
                "height",
                branch(vec![("base", leaf("metric", Index::Height))]),
            ),
            // height_cumulative with wrap="cumulative"
            (
                "height_cumulative",
                branch(vec![(
                    "cumulative",
                    leaf("metric_cumulative", Index::Height),
                )]),
            ),
            // From rest (flatten) - inner struct already merged to { sum, cumulative }
            // Each leaf has merged indexes from all time periods
            (
                "sum",
                leaf("metric_sum", Index::DateIndex), // Would have all time indexes
            ),
            (
                "cumulative",
                leaf("metric_cumulative", Index::DateIndex), // Would have all time indexes
            ),
        ]);

        let merged = tree.merge_branches().unwrap();

        // Expected: { base, sum, cumulative }
        match &merged {
            TreeNode::Branch(map) => {
                assert_eq!(
                    map.len(),
                    3,
                    "Expected base, sum, cumulative. Got: {:?}",
                    map.keys().collect::<Vec<_>>()
                );

                // base: only Height
                let base_indexes = get_leaf_indexes(map.get("base").unwrap()).unwrap();
                assert_eq!(base_indexes.len(), 1);
                assert!(base_indexes.contains(&Index::Height));

                // sum: from flattened rest
                assert!(matches!(map.get("sum"), Some(TreeNode::Leaf(_))));

                // cumulative: Height merged with rest's cumulative
                let cum_indexes = get_leaf_indexes(map.get("cumulative").unwrap()).unwrap();
                assert!(cum_indexes.contains(&Index::Height));
                assert!(cum_indexes.contains(&Index::DateIndex));
            }
            _ => panic!("Expected branch"),
        }
    }

    // ========== Case 9: ValueBlockSumCum (outer no merge, inner has merge) ==========
    // ValueBlockSumCum (no merge):
    //   - sats: ComputedBlockSumCum<Sats> (merge) → { base, sum, cumulative }
    //   - bitcoin: LazyBlockSumCum<Bitcoin> (merge) → { base, sum, cumulative }
    //   - dollars: Option<ComputedBlockSumCum<Dollars>> (merge) → { base, sum, cumulative }

    #[test]
    fn case9_value_block_sum_cum() {
        // Each denomination has already been merged internally
        // Simulating the output after inner merge
        let sats_merged = branch(vec![
            ("base", leaf("metric", Index::Height)),
            ("sum", leaf("metric_sum", Index::DateIndex)),
            ("cumulative", leaf("metric_cumulative", Index::Height)),
        ]);

        let bitcoin_merged = branch(vec![
            ("base", leaf("metric_btc", Index::Height)),
            ("sum", leaf("metric_btc_sum", Index::DateIndex)),
            ("cumulative", leaf("metric_btc_cumulative", Index::Height)),
        ]);

        let dollars_merged = branch(vec![
            ("base", leaf("metric_usd", Index::Height)),
            ("sum", leaf("metric_usd_sum", Index::DateIndex)),
            ("cumulative", leaf("metric_usd_cumulative", Index::Height)),
        ]);

        // Outer struct has no merge, so denominations stay as branches
        let tree = branch(vec![
            ("sats", sats_merged),
            ("bitcoin", bitcoin_merged),
            ("dollars", dollars_merged),
        ]);

        match &tree {
            TreeNode::Branch(map) => {
                assert_eq!(map.len(), 3);

                for denom in ["sats", "bitcoin", "dollars"] {
                    match map.get(denom) {
                        Some(TreeNode::Branch(inner)) => {
                            assert_eq!(inner.len(), 3);
                            assert!(inner.contains_key("base"));
                            assert!(inner.contains_key("sum"));
                            assert!(inner.contains_key("cumulative"));
                        }
                        _ => panic!("Expected branch for {}", denom),
                    }
                }
            }
            _ => panic!("Expected branch"),
        }
    }

    // ========== Case 10: ValueDateLast structure ==========
    // Testing individual components of ValueDateLast

    #[test]
    fn case10_derived_date_last_collapses_to_leaf() {
        // DerivedDateLast<T> with merge: all fields have wrap="last"
        // weekindex: { last: Leaf }, monthindex: { last: Leaf }, etc.
        // After merge: all "last" keys merge, same metric name → collapses to Leaf
        let tree = branch(vec![
            (
                "weekindex",
                branch(vec![("last", leaf("metric", Index::WeekIndex))]),
            ),
            (
                "monthindex",
                branch(vec![("last", leaf("metric", Index::MonthIndex))]),
            ),
            (
                "yearindex",
                branch(vec![("last", leaf("metric", Index::YearIndex))]),
            ),
        ]);

        let merged = tree.merge_branches().unwrap();

        // Should collapse to single Leaf with all indexes
        match &merged {
            TreeNode::Leaf(leaf) => {
                let indexes = leaf.indexes();
                assert!(indexes.contains(&Index::WeekIndex));
                assert!(indexes.contains(&Index::MonthIndex));
                assert!(indexes.contains(&Index::YearIndex));
            }
            TreeNode::Branch(map) => {
                panic!(
                    "Expected Leaf, got Branch: {:?}",
                    map.keys().collect::<Vec<_>>()
                );
            }
        }
    }

    #[test]
    fn case10_computed_date_last_collapses_to_leaf() {
        // ComputedDateLast<T> with merge:
        //   - dateindex with wrap="base" → { base: Leaf }
        //   - rest (flatten): DerivedDateLast already merged to Leaf
        //     → flatten inserts with field name "rest" as key
        //
        // Both have same metric name → collapses to single Leaf
        let tree = branch(vec![
            // dateindex with wrap="base"
            (
                "dateindex",
                branch(vec![("base", leaf("metric", Index::DateIndex))]),
            ),
            // rest (flatten): DerivedDateLast merged to Leaf
            // Same metric name as base
            ("rest", leaf("metric", Index::WeekIndex)),
        ]);

        let merged = tree.merge_branches().unwrap();

        // Same metric name → collapses to single Leaf with all indexes
        match &merged {
            TreeNode::Leaf(leaf) => {
                let indexes = leaf.indexes();
                assert!(indexes.contains(&Index::DateIndex));
                assert!(indexes.contains(&Index::WeekIndex));
            }
            TreeNode::Branch(map) => {
                panic!(
                    "Expected Leaf, got Branch: {:?}",
                    map.keys().collect::<Vec<_>>()
                );
            }
        }
    }

    // ========== Case 11: ValueDateLast conflict detection ==========

    #[test]
    fn case11_value_date_last_sats_key_conflict() {
        // ValueDateLast has a structural issue:
        // - sats_dateindex with wrap="sats" produces key "sats"
        // - rest (flatten) has field "sats" (DerivedDateLast<Sats>)
        // Both try to use the same "sats" key!

        // Simulating the pre-merge structure
        let tree = branch(vec![
            // From sats_dateindex with wrap="sats"
            (
                "sats_dateindex",
                branch(vec![("sats", leaf("metric", Index::DateIndex))]),
            ),
            // From rest (flatten): ValueDerivedDateLast
            (
                "rest",
                branch(vec![
                    // sats field: DerivedDateLast merged to Leaf
                    ("sats", leaf("metric", Index::WeekIndex)), // Same metric name!
                    ("bitcoin", leaf("metric_btc", Index::DateIndex)),
                    ("dollars", leaf("metric_usd", Index::DateIndex)),
                ]),
            ),
        ]);

        let merged = tree.merge_branches();

        // Should succeed because both "sats" have the same metric name
        // Indexes should be merged
        match merged {
            Some(TreeNode::Branch(map)) => {
                let sats_indexes = get_leaf_indexes(map.get("sats").unwrap()).unwrap();
                assert!(sats_indexes.contains(&Index::DateIndex));
                assert!(sats_indexes.contains(&Index::WeekIndex));
            }
            Some(_) => panic!("Expected branch"),
            None => panic!("Unexpected conflict"),
        }
    }

    // ========== Case 12: ValueDateLast ideal output ==========

    #[test]
    fn case12_value_date_last_ideal_output() {
        // The IDEAL output for ValueDateLast:
        // { sats: Leaf(all indexes), bitcoin: Leaf(all indexes), dollars: Leaf(all indexes) }
        //
        // This requires:
        // 1. Each denomination collapses its time indexes into one Leaf
        // 2. Denominations stay as separate siblings

        // Simulating final merged output
        let tree = branch(vec![
            ("sats", leaf("metric", Index::DateIndex)), // placeholder, would have all indexes
            ("bitcoin", leaf("metric_btc", Index::DateIndex)),
            ("dollars", leaf("metric_usd", Index::DateIndex)),
        ]);

        match &tree {
            TreeNode::Branch(map) => {
                assert_eq!(map.len(), 3);
                assert!(matches!(map.get("sats"), Some(TreeNode::Leaf(_))));
                assert!(matches!(map.get("bitcoin"), Some(TreeNode::Leaf(_))));
                assert!(matches!(map.get("dollars"), Some(TreeNode::Leaf(_))));
            }
            _ => panic!("Expected branch with 3 denomination leaves"),
        }
    }
}
