//! Tree traversal helpers for pattern analysis.
//!
//! This module provides utilities for working with the TreeNode structure,
//! including leaf name extraction and index pattern detection.

use std::collections::{BTreeMap, BTreeSet};

use bitview_catalog::{TreeNode, extract_json_type};
use brk_types::Index;
use indexmap::IndexMap;

use crate::{IndexSetPattern, PatternField, child_type_name};

use super::{find_common_prefix, find_common_suffix, normalize_prefix};

/// Get the shortest leaf name from a tree node.
///
/// This is useful for pattern base analysis where we want the "base" case
/// (e.g., the leaf without suffix like `_btc` or `_usd`).
pub fn get_shortest_leaf_name(node: &TreeNode) -> Option<String> {
    shortest_leaf_name(node).map(str::to_owned)
}

fn shortest_leaf_name(node: &TreeNode) -> Option<&str> {
    match node {
        TreeNode::Leaf(leaf) => Some(leaf.name()),
        TreeNode::Branch(children) => children
            .values()
            .filter_map(shortest_leaf_name)
            .min_by_key(|name| name.len()),
    }
}

/// Get the field signature for a branch node's children.
/// Fields are sorted alphabetically for consistent pattern matching.
pub fn get_node_fields(
    children: &IndexMap<String, TreeNode>,
    pattern_lookup: &BTreeMap<Vec<PatternField>, String>,
) -> Vec<PatternField> {
    let mut fields: Vec<PatternField> = children
        .iter()
        .map(|(name, node)| {
            let (rust_type, json_type, indexes) = match node {
                TreeNode::Leaf(leaf) => (
                    leaf.kind().to_string(),
                    extract_json_type(&leaf.schema),
                    leaf.indexes().clone(),
                ),
                TreeNode::Branch(grandchildren) => {
                    let child_fields = get_node_fields(grandchildren, pattern_lookup);
                    let pattern_name = pattern_lookup
                        .get(&child_fields)
                        .cloned()
                        .unwrap_or_else(|| "Unknown".to_string());
                    (pattern_name.clone(), pattern_name, BTreeSet::new())
                }
            };
            PatternField {
                name: name.clone(),
                rust_type,
                json_type,
                indexes,
                type_param: None,
            }
        })
        .collect();
    // Sort for consistent pattern matching (display order preserved in IndexMap)
    fields.sort_by(|a, b| a.name.cmp(&b.name));
    fields
}

/// Detect index patterns (sets of indexes that appear together on series).
pub fn detect_index_patterns(tree: &TreeNode) -> Vec<IndexSetPattern> {
    let mut unique_index_sets = BTreeSet::new();
    collect_index_sets_from_tree(tree, &mut unique_index_sets);

    // Sort by count (descending) then by first index name for deterministic ordering
    let mut sorted_sets: Vec<_> = unique_index_sets
        .into_iter()
        .filter(|indexes| !indexes.is_empty())
        .collect();
    sorted_sets.sort_by(|a, b| {
        b.len()
            .cmp(&a.len())
            .then_with(|| a.iter().next().cmp(&b.iter().next()))
    });

    // Assign unique sequential names
    sorted_sets
        .into_iter()
        .enumerate()
        .map(|(i, indexes)| IndexSetPattern {
            name: format!("SeriesPattern{}", i + 1),
            indexes: indexes.clone(),
        })
        .collect()
}

fn collect_index_sets_from_tree<'a>(
    node: &'a TreeNode,
    unique_index_sets: &mut BTreeSet<&'a BTreeSet<Index>>,
) {
    match node {
        TreeNode::Leaf(leaf) => {
            unique_index_sets.insert(leaf.indexes());
        }
        TreeNode::Branch(children) => {
            for child in children.values() {
                collect_index_sets_from_tree(child, unique_index_sets);
            }
        }
    }
}

/// Result of analyzing a pattern instance's base.
#[derive(Debug, Clone)]
pub struct PatternBaseResult {
    /// The computed base name for the pattern.
    pub base: String,
    /// Whether an outlier child was excluded to find the pattern.
    /// If true, pattern factory should not be used.
    pub has_outlier: bool,
    /// Whether this instance uses suffix mode (common prefix) or prefix mode (common suffix).
    /// Used to check compatibility with the pattern's mode.
    pub is_suffix_mode: bool,
    /// The field parts (suffix in suffix mode, prefix in prefix mode) for each field.
    /// Used to check if instance field parts match the pattern's field parts.
    pub field_parts: BTreeMap<String, String>,
}

impl PatternBaseResult {
    /// Create a default result that forces inlining (has_outlier = true).
    /// Use when no pattern base could be computed during lookup.
    pub fn force_inline() -> Self {
        Self {
            base: String::new(),
            has_outlier: true,
            is_suffix_mode: true,
            field_parts: BTreeMap::new(),
        }
    }

    /// Create an empty result with no outlier.
    /// Use for root-level patterns or when children have no common pattern.
    pub fn empty() -> Self {
        Self {
            base: String::new(),
            has_outlier: false,
            is_suffix_mode: true,
            field_parts: BTreeMap::new(),
        }
    }
}

/// Get the series base for a pattern instance by analyzing direct children.
///
/// Uses the shortest leaf names from direct children to find common prefix/suffix.
///
/// If the initial analysis fails to find a common pattern, it tries excluding
/// each child one at a time to detect outliers (e.g., a mismatched "base" field
/// from indexer/computed tree merging).
///
/// Returns both the base and whether an outlier was detected.
pub fn get_pattern_instance_base(node: &TreeNode) -> PatternBaseResult {
    let child_names = get_direct_children_for_analysis(node);
    if child_names.is_empty() {
        return PatternBaseResult::empty();
    }

    // Try to find common base from leaf names
    if let Some(result) = try_find_base(&child_names, false) {
        return result;
    }

    // If no common pattern found and we have enough children, try excluding outliers
    if child_names.len() > 2 {
        for i in 0..child_names.len() {
            let filtered: Vec<_> = child_names
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, v)| *v)
                .collect();

            if let Some(result) = try_find_base(&filtered, true) {
                return result;
            }
        }
    }

    // Fallback: no common prefix/suffix found - this is a root-level pattern
    // Return empty base so series names are used directly
    PatternBaseResult::empty()
}

/// Try to find a common base from child names using prefix/suffix detection.
/// Returns the pattern base if found.
fn try_find_base(
    child_names: &[(&str, &str)],
    is_outlier_attempt: bool,
) -> Option<PatternBaseResult> {
    let leaf_names: Vec<&str> = child_names.iter().map(|(_, n)| *n).collect();

    // Try common prefix first (suffix mode)
    if let Some(prefix) = find_common_prefix(&leaf_names) {
        let base = prefix.trim_end_matches('_').to_string();
        let mut field_parts = BTreeMap::new();
        for (field_name, leaf_name) in child_names {
            // Compute the suffix part for this field
            let suffix = if *leaf_name == base {
                String::new()
            } else {
                leaf_name
                    .strip_prefix(&prefix)
                    .unwrap_or(leaf_name)
                    .to_string()
            };
            field_parts.insert((*field_name).to_owned(), suffix);
        }
        return Some(PatternBaseResult {
            base,
            has_outlier: is_outlier_attempt,
            is_suffix_mode: true,
            field_parts,
        });
    }

    // Try common suffix (prefix mode)
    if let Some(suffix) = find_common_suffix(&leaf_names) {
        let base = suffix.trim_start_matches('_').to_string();
        let mut field_parts = BTreeMap::new();
        for (field_name, leaf_name) in child_names {
            // Compute the prefix part for this field, normalized to end with _
            let prefix_part = leaf_name
                .strip_suffix(&suffix)
                .map(normalize_prefix)
                .unwrap_or_default();
            field_parts.insert((*field_name).to_owned(), prefix_part);
        }
        return Some(PatternBaseResult {
            base,
            has_outlier: is_outlier_attempt,
            is_suffix_mode: false,
            field_parts,
        });
    }

    None
}

/// Get (field_name, shortest_leaf_name) pairs for direct children of a branch node.
///
/// Uses the shortest leaf name from each child subtree to find the "base" case
/// (the leaf without suffix modifiers like `_btc` or `_usd`).
fn get_direct_children_for_analysis(node: &TreeNode) -> Vec<(&str, &str)> {
    match node {
        TreeNode::Leaf(leaf) => vec![(leaf.name(), leaf.name())],
        TreeNode::Branch(children) => children
            .iter()
            .filter_map(|(field_name, child)| {
                shortest_leaf_name(child).map(|leaf_name| (field_name.as_str(), leaf_name))
            })
            .collect(),
    }
}

/// Infer the accumulated name for a child node based on a descendant leaf name.
pub fn infer_accumulated_name(parent_acc: &str, field_name: &str, descendant_leaf: &str) -> String {
    if descendant_leaf.starts_with(field_name) {
        return field_name.to_string();
    }

    if parent_acc.is_empty() {
        field_name.to_string()
    } else {
        format!("{}_{}", parent_acc, field_name)
    }
}

/// Get fields with child field information for generic pattern lookup.
pub fn get_fields_with_child_info(
    children: &IndexMap<String, TreeNode>,
    parent_name: &str,
    pattern_lookup: &BTreeMap<Vec<PatternField>, String>,
) -> Vec<(PatternField, Option<Vec<PatternField>>)> {
    children
        .iter()
        .map(|(name, node)| {
            let (rust_type, json_type, indexes, child_fields) = match node {
                TreeNode::Leaf(leaf) => (
                    leaf.kind().to_string(),
                    extract_json_type(&leaf.schema),
                    leaf.indexes().clone(),
                    None,
                ),
                TreeNode::Branch(grandchildren) => {
                    let child_fields = get_node_fields(grandchildren, pattern_lookup);
                    let pattern_name = pattern_lookup
                        .get(&child_fields)
                        .cloned()
                        .unwrap_or_else(|| child_type_name(parent_name, name));
                    (
                        pattern_name.clone(),
                        pattern_name,
                        BTreeSet::new(),
                        Some(child_fields),
                    )
                }
            };
            (
                PatternField {
                    name: name.clone(),
                    rust_type,
                    json_type,
                    indexes,
                    type_param: None,
                },
                child_fields,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitview_catalog::{SeriesLeaf, SeriesLeafWithSchema, TreeNode};

    fn make_leaf(name: &str) -> TreeNode {
        let leaf = SeriesLeaf {
            name: name.to_string(),
            kind: "TestType".to_string(),
            indexes: BTreeSet::new(),
            description: None,
        };
        TreeNode::Leaf(SeriesLeafWithSchema::new(leaf, serde_json::json!({})))
    }

    fn make_branch(children: Vec<(&str, TreeNode)>) -> TreeNode {
        let map: IndexMap<String, TreeNode> = children
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        TreeNode::branch(map)
    }

    #[test]
    fn shortest_name_borrows_the_first_tied_leaf_and_skips_empty_branches() {
        let tree = make_branch(vec![
            ("empty", make_branch(vec![])),
            (
                "nested",
                make_branch(vec![
                    ("long", make_leaf("long_name")),
                    ("first", make_leaf("é")),
                ]),
            ),
            ("last", make_leaf("aa")),
        ]);
        assert_eq!(get_shortest_leaf_name(&tree).as_deref(), Some("é"));
        let TreeNode::Branch(root) = &tree else {
            unreachable!()
        };
        let TreeNode::Branch(nested) = &root["nested"] else {
            unreachable!()
        };
        let TreeNode::Leaf(first) = &nested["first"] else {
            unreachable!()
        };
        assert!(std::ptr::eq(
            shortest_leaf_name(&tree).unwrap().as_ptr(),
            first.name().as_ptr()
        ));
        assert!(get_shortest_leaf_name(&make_branch(vec![])).is_none());
    }

    #[test]
    fn index_patterns_deduplicate_by_value_and_keep_names_after_catalog_drop() {
        let with_indexes = |name: &str, indexes: BTreeSet<Index>| {
            let mut node = make_leaf(name);
            let TreeNode::Leaf(leaf) = &mut node else {
                unreachable!()
            };
            leaf.leaf.indexes = indexes;
            node
        };
        let both = BTreeSet::from([Index::Height, Index::Day1]);
        let height = BTreeSet::from([Index::Height]);
        let tree = make_branch(vec![
            ("height", with_indexes("height", height.clone())),
            ("both", with_indexes("both", both.clone())),
            ("again", with_indexes("again", both.clone())),
            ("empty", make_leaf("empty")),
        ]);
        let patterns = detect_index_patterns(&tree);
        drop(tree);
        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns[0].name, "SeriesPattern1");
        assert_eq!(patterns[0].indexes, both);
        assert_eq!(patterns[1].name, "SeriesPattern2");
        assert_eq!(patterns[1].indexes, height);
    }

    #[test]
    fn test_get_pattern_instance_base_with_base_field() {
        // Simulates vbytes tree: has base field with block_vbytes leaf
        let tree = make_branch(vec![
            (
                "base",
                make_branch(vec![("day1", make_leaf("block_vbytes"))]),
            ),
            (
                "average",
                make_branch(vec![("day1", make_leaf("block_vbytes_average"))]),
            ),
            (
                "sum",
                make_branch(vec![("day1", make_leaf("block_vbytes_sum"))]),
            ),
        ]);

        let result = get_pattern_instance_base(&tree);
        assert_eq!(result.base, "block_vbytes");
        assert!(!result.has_outlier);
    }

    #[test]
    fn test_get_pattern_instance_base_without_base_field() {
        // Simulates weight tree: NO base field, only suffixed series
        let tree = make_branch(vec![
            (
                "average",
                make_branch(vec![("day1", make_leaf("block_weight_average"))]),
            ),
            (
                "sum",
                make_branch(vec![("day1", make_leaf("block_weight_sum"))]),
            ),
            (
                "cumulative",
                make_branch(vec![("day1", make_leaf("block_weight_cumulative"))]),
            ),
            (
                "max",
                make_branch(vec![("day1", make_leaf("block_weight_max"))]),
            ),
            (
                "min",
                make_branch(vec![("day1", make_leaf("block_weight_min"))]),
            ),
        ]);

        let result = get_pattern_instance_base(&tree);
        assert_eq!(result.base, "block_weight");
        assert!(!result.has_outlier);
    }

    #[test]
    fn test_get_pattern_instance_base_with_duplicate_base_field() {
        // What if there's a "base" field that points to the same leaf as "average"?
        // This could happen if the tree generation creates a base field that shares leaves with average
        let tree = make_branch(vec![
            (
                "base",
                make_branch(vec![("day1", make_leaf("block_weight_average"))]),
            ),
            (
                "average",
                make_branch(vec![("day1", make_leaf("block_weight_average"))]),
            ),
            (
                "sum",
                make_branch(vec![("day1", make_leaf("block_weight_sum"))]),
            ),
        ]);

        let result = get_pattern_instance_base(&tree);
        // Common prefix among all children is "block_weight_"
        assert_eq!(result.base, "block_weight");
        assert!(!result.has_outlier);
    }

    #[test]
    fn test_get_pattern_instance_base_with_mismatched_base_name() {
        // Simulates the actual bug: indexed tree's "base" field has name "weight"
        // but computed tree's derived series use "block_weight_*" prefix.
        // After tree merge, we get a base field with mismatched naming.
        let tree = make_branch(vec![
            ("base", make_leaf("weight")), // Outlier - doesn't match pattern
            ("average", make_leaf("block_weight_average")),
            ("sum", make_leaf("block_weight_sum")),
            ("cumulative", make_leaf("block_weight_cumulative")),
            ("max", make_leaf("block_weight_max")),
            ("min", make_leaf("block_weight_min")),
        ]);

        let result = get_pattern_instance_base(&tree);
        // Should detect "weight" as outlier and find common prefix from others
        assert_eq!(result.base, "block_weight");
        assert!(result.has_outlier); // Pattern factory should NOT be used
    }

    #[test]
    fn test_get_pattern_instance_base_root_level_no_common_pattern() {
        // Simulates root-level pattern with series that have no common prefix/suffix.
        // These names have no shared prefix or suffix, even when excluding any one.
        // In this case, we should return empty base so series names are used directly.
        let tree = make_branch(vec![
            ("alpha", make_leaf("foo_series")),
            ("beta", make_leaf("bar_value")),
            ("gamma", make_leaf("baz_count")),
        ]);

        let result = get_pattern_instance_base(&tree);
        // No common prefix or suffix - return empty base
        assert_eq!(result.base, "");
        assert!(!result.has_outlier);
    }

    #[test]
    fn test_get_pattern_instance_base_two_children_no_pattern() {
        // Two children with no common pattern - should still return empty base
        let tree = make_branch(vec![
            ("foo", make_leaf("alpha")),
            ("bar", make_leaf("beta")),
        ]);

        let result = get_pattern_instance_base(&tree);
        assert_eq!(result.base, "");
        assert!(!result.has_outlier);
    }

    #[test]
    fn test_get_pattern_instance_base_with_outlier_excluded() {
        // Simulates the realized pattern: adjusted_sopr, sopr, asopr.
        // When "asopr" is excluded as outlier, "adjusted_sopr" and "sopr" share suffix "_sopr".
        // The outlier detection should find base="sopr" with has_outlier=true.
        let tree = make_branch(vec![
            ("adjustedSopr", make_leaf("adjusted_sopr")),
            ("sopr", make_leaf("sopr")),
            ("asopr", make_leaf("asopr")),
        ]);

        let result = get_pattern_instance_base(&tree);
        // Outlier detected - pattern base found by excluding "asopr"
        assert_eq!(result.base, "sopr");
        assert!(result.has_outlier); // Pattern factory should NOT be used (inline instead)
    }

    #[test]
    fn test_get_pattern_instance_base_suffix_mode_price_ago() {
        // Simulates price_ago pattern: price_24h_ago, price_1w_ago, price_10y_ago
        // Common prefix is "price_", so this is suffix mode
        let tree = make_branch(vec![
            ("_24h", make_leaf("price_24h_ago")),
            ("_1w", make_leaf("price_1w_ago")),
            ("_1m", make_leaf("price_1m_ago")),
            ("_10y", make_leaf("price_10y_ago")),
        ]);

        let result = get_pattern_instance_base(&tree);
        assert_eq!(result.base, "price");
        assert!(result.is_suffix_mode); // Suffix mode: _m(base, "24h_ago")
        assert!(!result.has_outlier);
    }

    #[test]
    fn test_get_pattern_instance_base_prefix_mode_price_returns() {
        // Simulates price_returns pattern: 24h_price_returns, 1w_price_returns, 10y_price_returns
        // Common suffix is "_price_returns", so this is prefix mode
        let tree = make_branch(vec![
            ("_24h", make_leaf("24h_price_returns")),
            ("_1w", make_leaf("1w_price_returns")),
            ("_1m", make_leaf("1m_price_returns")),
            ("_10y", make_leaf("10y_price_returns")),
        ]);

        let result = get_pattern_instance_base(&tree);
        assert_eq!(result.base, "price_returns");
        assert!(!result.is_suffix_mode); // Prefix mode: _p("24h_", base)
        assert!(!result.has_outlier);
    }

    #[test]
    fn test_mode_detection_distinguishes_similar_structures() {
        // Two patterns with identical structure but different naming conventions
        // should have different modes detected

        // Suffix mode pattern
        let suffix_tree = make_branch(vec![
            ("_1y", make_leaf("lump_sum_1y")),
            ("_2y", make_leaf("lump_sum_2y")),
            ("_5y", make_leaf("lump_sum_5y")),
        ]);
        let suffix_result = get_pattern_instance_base(&suffix_tree);
        assert_eq!(suffix_result.base, "lump_sum");
        assert!(suffix_result.is_suffix_mode);

        // Prefix mode pattern (same structure, different naming)
        let prefix_tree = make_branch(vec![
            ("_1y", make_leaf("1y_returns")),
            ("_2y", make_leaf("2y_returns")),
            ("_5y", make_leaf("5y_returns")),
        ]);
        let prefix_result = get_pattern_instance_base(&prefix_tree);
        assert_eq!(prefix_result.base, "returns");
        assert!(!prefix_result.is_suffix_mode);
    }
}
