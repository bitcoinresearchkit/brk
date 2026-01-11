//! Tree traversal helpers for pattern analysis.
//!
//! This module provides utilities for working with the TreeNode structure,
//! including leaf name extraction and index pattern detection.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use brk_types::{Index, TreeNode, extract_json_type};

use crate::{IndexSetPattern, PatternField, analysis::names::analyze_pattern_level, child_type_name};

/// Get the first leaf name from a tree node.
pub fn get_first_leaf_name(node: &TreeNode) -> Option<String> {
    match node {
        TreeNode::Leaf(leaf) => Some(leaf.name().to_string()),
        TreeNode::Branch(children) => children.values().find_map(get_first_leaf_name),
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

/// Get the metric base for a pattern instance by analyzing direct children.
///
/// Uses field names and first leaf names from direct children to determine
/// the common base via `analyze_pattern_level`.
pub fn get_pattern_instance_base(node: &TreeNode) -> String {
    let child_names = get_direct_children_for_analysis(node);
    if child_names.is_empty() {
        return String::new();
    }
    analyze_pattern_level(&child_names).base
}

/// Get (field_name, first_leaf_name) pairs for direct children of a branch node.
fn get_direct_children_for_analysis(node: &TreeNode) -> Vec<(String, String)> {
    match node {
        TreeNode::Leaf(leaf) => vec![(leaf.name().to_string(), leaf.name().to_string())],
        TreeNode::Branch(children) => children
            .iter()
            .filter_map(|(field_name, child)| {
                get_first_leaf_name(child).map(|leaf_name| (field_name.clone(), leaf_name))
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
