//! Tree traversal utilities.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use brk_types::{Index, TreeNode};

use super::{PatternField, case::to_pascal_case, schema::schema_to_json_type};

/// Get the first leaf name from a tree node.
pub fn get_first_leaf_name(node: &TreeNode) -> Option<String> {
    match node {
        TreeNode::Leaf(leaf) => Some(leaf.name().to_string()),
        TreeNode::Branch(children) => children.values().find_map(get_first_leaf_name),
    }
}

/// Get the metric base for a pattern instance by analyzing the first leaf descendant.
pub fn get_pattern_instance_base(node: &TreeNode, field_name: &str) -> String {
    if let Some(leaf_name) = get_first_leaf_name(node)
        && leaf_name.contains(field_name)
    {
        return field_name.to_string();
    }
    field_name.to_string()
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
                    leaf.value_type().to_string(),
                    schema_to_json_type(&leaf.schema),
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
            }
        })
        .collect();
    fields.sort_by(|a, b| a.name.cmp(&b.name));
    fields
}

/// Get fields with child field information for generic pattern lookup.
/// Returns (field, child_fields) pairs where child_fields is Some for branches.
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
                    leaf.value_type().to_string(),
                    schema_to_json_type(&leaf.schema),
                    leaf.indexes().clone(),
                    None,
                ),
                TreeNode::Branch(grandchildren) => {
                    let child_fields = get_node_fields(grandchildren, pattern_lookup);
                    let pattern_name = pattern_lookup
                        .get(&child_fields)
                        .cloned()
                        .unwrap_or_else(|| format!("{}_{}", parent_name, to_pascal_case(name)));
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
                },
                child_fields,
            )
        })
        .collect()
}

/// Detect index patterns (sets of indexes that appear together on multiple metrics).
pub fn detect_index_patterns(tree: &TreeNode) -> (BTreeSet<Index>, Vec<super::IndexSetPattern>) {
    let mut used_indexes: BTreeSet<Index> = BTreeSet::new();
    let mut index_sets: Vec<BTreeSet<Index>> = Vec::new();

    collect_indexes_from_tree(tree, &mut used_indexes, &mut index_sets);

    // Count occurrences of each unique index set
    let mut index_set_counts: Vec<(BTreeSet<Index>, usize)> = Vec::new();
    for index_set in index_sets {
        if let Some(entry) = index_set_counts.iter_mut().find(|(s, _)| s == &index_set) {
            entry.1 += 1;
        } else {
            index_set_counts.push((index_set, 1));
        }
    }

    // Build patterns for index sets appearing 2+ times
    let mut patterns: Vec<super::IndexSetPattern> = index_set_counts
        .into_iter()
        .filter(|(indexes, count)| *count >= 2 && !indexes.is_empty())
        .enumerate()
        .map(|(i, (indexes, _))| super::IndexSetPattern {
            name: if i == 0 {
                "Indexes".to_string()
            } else {
                format!("Indexes{}", i + 1)
            },
            indexes,
        })
        .collect();

    patterns.sort_by(|a, b| b.indexes.len().cmp(&a.indexes.len()));
    (used_indexes, patterns)
}

fn collect_indexes_from_tree(
    node: &TreeNode,
    used_indexes: &mut BTreeSet<Index>,
    index_sets: &mut Vec<BTreeSet<Index>>,
) {
    match node {
        TreeNode::Leaf(leaf) => {
            used_indexes.extend(leaf.indexes().iter().cloned());
            index_sets.push(leaf.indexes().clone());
        }
        TreeNode::Branch(children) => {
            for child in children.values() {
                collect_indexes_from_tree(child, used_indexes, index_sets);
            }
        }
    }
}
