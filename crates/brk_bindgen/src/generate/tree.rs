//! Shared tree generation helpers.

use std::collections::{HashMap, HashSet};

use brk_types::TreeNode;

use crate::{
    ClientMetadata, PatternBaseResult, PatternField, child_type_name, get_fields_with_child_info,
    get_pattern_instance_base,
};

/// Pre-computed context for a single child node.
pub struct ChildContext<'a> {
    /// The child's field name in the tree.
    pub name: &'a str,
    /// The child node.
    pub node: &'a TreeNode,
    /// The field info for this child.
    pub field: PatternField,
    /// Child fields if this is a branch (for pattern lookup).
    pub child_fields: Option<Vec<PatternField>>,
    /// Pattern analysis result.
    pub base_result: PatternBaseResult,
    /// Whether this is a leaf node.
    pub is_leaf: bool,
    /// Whether to use an inline type instead of a pattern type (only meaningful for branches).
    pub should_inline: bool,
    /// The type name to use for inline branches.
    pub inline_type_name: String,
}

/// Context for generating a tree node, returned by `prepare_tree_node`.
pub struct TreeNodeContext<'a> {
    /// Pre-computed context for each child.
    pub children: Vec<ChildContext<'a>>,
}

/// Prepare a tree node for generation.
/// Returns None if the node should be skipped (not a branch, already generated,
/// or matches a parameterizable pattern).
pub fn prepare_tree_node<'a>(
    node: &'a TreeNode,
    name: &str,
    pattern_lookup: &HashMap<Vec<PatternField>, String>,
    metadata: &ClientMetadata,
    generated: &mut HashSet<String>,
) -> Option<TreeNodeContext<'a>> {
    let TreeNode::Branch(branch_children) = node else {
        return None;
    };

    let fields_with_child_info = get_fields_with_child_info(branch_children, name, pattern_lookup);
    let fields: Vec<PatternField> = fields_with_child_info
        .iter()
        .map(|(f, _)| f.clone())
        .collect();

    // Skip if this matches a parameterizable pattern AND has no outlier AND mode matches
    let base_result = get_pattern_instance_base(node);
    let mode_matches = pattern_lookup
        .get(&fields)
        .and_then(|name| metadata.find_pattern(name))
        .is_none_or(|p| p.is_suffix_mode() == base_result.is_suffix_mode);
    if let Some(pattern_name) = pattern_lookup.get(&fields)
        && pattern_name != name
        && metadata.is_parameterizable(pattern_name)
        && !base_result.has_outlier
        && mode_matches
    {
        return None;
    }

    // Skip if already generated
    if generated.contains(name) {
        return None;
    }
    generated.insert(name.to_string());

    // Build child contexts with pre-computed decisions
    let children: Vec<ChildContext<'a>> = branch_children
        .iter()
        .zip(fields_with_child_info)
        .map(|((child_name, child_node), (field, child_fields))| {
            let is_leaf = matches!(child_node, TreeNode::Leaf(_));
            let base_result = get_pattern_instance_base(child_node);

            // For type annotations: use pattern type if ANY pattern matches
            let matches_any_pattern = child_fields
                .as_ref()
                .is_some_and(|cf| metadata.matches_pattern(cf));

            // Check if the pattern mode matches the instance mode
            let mode_matches = child_fields
                .as_ref()
                .and_then(|cf| metadata.find_pattern_by_fields(cf))
                .is_none_or(|p| p.is_suffix_mode() == base_result.is_suffix_mode);

            // should_inline determines if we generate an inline struct type
            // We inline if: it's a branch AND (doesn't match any pattern OR mode doesn't match OR has outlier)
            let should_inline =
                !is_leaf && (!matches_any_pattern || !mode_matches || base_result.has_outlier);

            // Inline type name (only used when should_inline is true)
            let inline_type_name = if should_inline {
                child_type_name(name, child_name)
            } else {
                String::new()
            };

            ChildContext {
                name: child_name,
                node: child_node,
                field,
                child_fields,
                base_result,
                is_leaf,
                should_inline,
                inline_type_name,
            }
        })
        .collect();

    Some(TreeNodeContext { children })
}
