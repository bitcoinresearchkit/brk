//! Tree traversal helpers for pattern analysis.
//!
//! This module provides utilities for working with the TreeNode structure,
//! including leaf name extraction and index pattern detection.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use brk_types::{Index, TreeNode, extract_json_type};

use crate::{IndexSetPattern, PatternField, child_type_name};

use super::{find_common_prefix, find_common_suffix};

/// Get the first leaf name from a tree node.
pub fn get_first_leaf_name(node: &TreeNode) -> Option<String> {
    match node {
        TreeNode::Leaf(leaf) => Some(leaf.name().to_string()),
        TreeNode::Branch(children) => children.values().find_map(get_first_leaf_name),
    }
}

/// Get the shortest leaf name from a tree node.
///
/// This is useful for pattern base analysis where we want the "base" case
/// (e.g., the leaf without suffix like `_btc` or `_usd`).
fn get_shortest_leaf_name(node: &TreeNode) -> Option<String> {
    match node {
        TreeNode::Leaf(leaf) => Some(leaf.name().to_string()),
        TreeNode::Branch(children) => children
            .values()
            .filter_map(get_shortest_leaf_name)
            .min_by_key(|name| name.len()),
    }
}

/// Get all leaf names from a tree node.
pub fn get_all_leaf_names(node: &TreeNode) -> Vec<String> {
    match node {
        TreeNode::Leaf(leaf) => vec![leaf.name().to_string()],
        TreeNode::Branch(children) => children.values().flat_map(get_all_leaf_names).collect(),
    }
}

/// Get the field signature for a branch node's children.
pub fn get_node_fields(
    children: &BTreeMap<String, TreeNode>,
    pattern_lookup: &HashMap<Vec<PatternField>, String>,
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
    fields.sort_by(|a, b| a.name.cmp(&b.name));
    fields
}

/// Detect index patterns (sets of indexes that appear together on metrics).
pub fn detect_index_patterns(tree: &TreeNode) -> (BTreeSet<Index>, Vec<IndexSetPattern>) {
    let mut used_indexes: BTreeSet<Index> = BTreeSet::new();
    let mut unique_index_sets: BTreeSet<BTreeSet<Index>> = BTreeSet::new();

    collect_indexes_from_tree(tree, &mut used_indexes, &mut unique_index_sets);

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
    let patterns: Vec<IndexSetPattern> = sorted_sets
        .into_iter()
        .enumerate()
        .map(|(i, indexes)| IndexSetPattern {
            name: format!("MetricPattern{}", i + 1),
            indexes,
        })
        .collect();

    (used_indexes, patterns)
}

fn collect_indexes_from_tree(
    node: &TreeNode,
    used_indexes: &mut BTreeSet<Index>,
    unique_index_sets: &mut BTreeSet<BTreeSet<Index>>,
) {
    match node {
        TreeNode::Leaf(leaf) => {
            used_indexes.extend(leaf.indexes().iter().cloned());
            unique_index_sets.insert(leaf.indexes().clone());
        }
        TreeNode::Branch(children) => {
            for child in children.values() {
                collect_indexes_from_tree(child, used_indexes, unique_index_sets);
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
}

impl PatternBaseResult {
    /// Returns true if an inline type should be generated instead of using a pattern factory.
    ///
    /// This is the case when:
    /// - The child fields don't match a parameterizable pattern, OR
    /// - An outlier was detected during pattern analysis
    pub fn should_inline(&self, is_parameterizable: bool) -> bool {
        !is_parameterizable || self.has_outlier
    }
}

/// Get the metric base for a pattern instance by analyzing direct children.
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
        return PatternBaseResult {
            base: String::new(),
            has_outlier: false,
        };
    }

    // Try to find common base from leaf names
    if let Some((base, has_outlier)) = try_find_base(&child_names, false) {
        return PatternBaseResult { base, has_outlier };
    }

    // If no common pattern found and we have enough children, try excluding outliers
    if child_names.len() > 2 {
        for i in 0..child_names.len() {
            let filtered: Vec<_> = child_names
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, v)| v.clone())
                .collect();

            if let Some((base, _)) = try_find_base(&filtered, true) {
                return PatternBaseResult {
                    base,
                    has_outlier: true,
                };
            }
        }
    }

    // Fallback: no common prefix/suffix found - this is a root-level pattern
    // Return empty base so metric names are used directly
    PatternBaseResult {
        base: String::new(),
        has_outlier: false,
    }
}

/// Try to find a common base from child names using prefix/suffix detection.
/// Returns Some((base, has_outlier)) if found.
fn try_find_base(child_names: &[(String, String)], is_outlier_attempt: bool) -> Option<(String, bool)> {
    let leaf_names: Vec<&str> = child_names.iter().map(|(_, n)| n.as_str()).collect();

    // Try common prefix first (suffix mode)
    if let Some(prefix) = find_common_prefix(&leaf_names) {
        let base = prefix.trim_end_matches('_').to_string();
        return Some((base, is_outlier_attempt));
    }

    // Try common suffix (prefix mode)
    if let Some(suffix) = find_common_suffix(&leaf_names) {
        let base = suffix.trim_start_matches('_').to_string();
        return Some((base, is_outlier_attempt));
    }

    None
}

/// Get (field_name, shortest_leaf_name) pairs for direct children of a branch node.
///
/// Uses the shortest leaf name from each child subtree to find the "base" case
/// (the leaf without suffix modifiers like `_btc` or `_usd`).
fn get_direct_children_for_analysis(node: &TreeNode) -> Vec<(String, String)> {
    match node {
        TreeNode::Leaf(leaf) => vec![(leaf.name().to_string(), leaf.name().to_string())],
        TreeNode::Branch(children) => children
            .iter()
            .filter_map(|(field_name, child)| {
                get_shortest_leaf_name(child).map(|leaf_name| (field_name.clone(), leaf_name))
            })
            .collect(),
    }
}

/// Infer the accumulated name for a child node based on a descendant leaf name.
pub fn infer_accumulated_name(parent_acc: &str, field_name: &str, descendant_leaf: &str) -> String {
    if let Some(pos) = descendant_leaf.find(field_name) {
        if pos == 0 {
            return field_name.to_string();
        }
        if pos > 0 && descendant_leaf.chars().nth(pos - 1) == Some('_') {
            return if parent_acc.is_empty() {
                field_name.to_string()
            } else {
                format!("{}_{}", parent_acc, field_name)
            };
        }
    }

    if parent_acc.is_empty() {
        field_name.to_string()
    } else {
        format!("{}_{}", parent_acc, field_name)
    }
}

/// Get fields with child field information for generic pattern lookup.
pub fn get_fields_with_child_info(
    children: &BTreeMap<String, TreeNode>,
    parent_name: &str,
    pattern_lookup: &HashMap<Vec<PatternField>, String>,
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
    use brk_types::{MetricLeaf, MetricLeafWithSchema, TreeNode};
    use std::collections::BTreeMap;

    fn make_leaf(name: &str) -> TreeNode {
        let leaf = MetricLeaf {
            name: name.to_string(),
            kind: "TestType".to_string(),
            indexes: BTreeSet::new(),
        };
        TreeNode::Leaf(MetricLeafWithSchema::new(leaf, serde_json::json!({})))
    }

    fn make_branch(children: Vec<(&str, TreeNode)>) -> TreeNode {
        let map: BTreeMap<String, TreeNode> = children
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        TreeNode::Branch(map)
    }

    #[test]
    fn test_get_pattern_instance_base_with_base_field() {
        // Simulates vbytes tree: has base field with block_vbytes leaf
        let tree = make_branch(vec![
            ("base", make_branch(vec![("dateindex", make_leaf("block_vbytes"))])),
            ("average", make_branch(vec![("dateindex", make_leaf("block_vbytes_average"))])),
            ("sum", make_branch(vec![("dateindex", make_leaf("block_vbytes_sum"))])),
        ]);

        let result = get_pattern_instance_base(&tree);
        assert_eq!(result.base, "block_vbytes");
        assert!(!result.has_outlier);
    }

    #[test]
    fn test_get_pattern_instance_base_without_base_field() {
        // Simulates weight tree: NO base field, only suffixed metrics
        let tree = make_branch(vec![
            ("average", make_branch(vec![("dateindex", make_leaf("block_weight_average"))])),
            ("sum", make_branch(vec![("dateindex", make_leaf("block_weight_sum"))])),
            ("cumulative", make_branch(vec![("dateindex", make_leaf("block_weight_cumulative"))])),
            ("max", make_branch(vec![("dateindex", make_leaf("block_weight_max"))])),
            ("min", make_branch(vec![("dateindex", make_leaf("block_weight_min"))])),
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
            ("base", make_branch(vec![("dateindex", make_leaf("block_weight_average"))])),
            ("average", make_branch(vec![("dateindex", make_leaf("block_weight_average"))])),
            ("sum", make_branch(vec![("dateindex", make_leaf("block_weight_sum"))])),
        ]);

        let result = get_pattern_instance_base(&tree);
        // Common prefix among all children is "block_weight_"
        assert_eq!(result.base, "block_weight");
        assert!(!result.has_outlier);
    }

    #[test]
    fn test_get_pattern_instance_base_with_mismatched_base_name() {
        // Simulates the actual bug: indexed tree's "base" field has name "weight"
        // but computed tree's derived metrics use "block_weight_*" prefix.
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
        // Simulates root-level pattern with metrics that have no common prefix/suffix.
        // These names have no shared prefix or suffix, even when excluding any one.
        // In this case, we should return empty base so metric names are used directly.
        let tree = make_branch(vec![
            ("alpha", make_leaf("foo_metric")),
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
}
