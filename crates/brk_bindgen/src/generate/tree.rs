//! Shared tree generation helpers.

use std::collections::{BTreeMap, BTreeSet};

use brk_types::TreeNode;

use crate::{
    ClientMetadata, PatternBaseResult, PatternField, child_type_name, get_fields_with_child_info,
};

/// Build a child path by appending a child name to a parent path.
/// Uses "/" as separator. If parent is empty, returns just the child name.
#[inline]
pub fn build_child_path(parent: &str, child: &str) -> String {
    if parent.is_empty() {
        child.to_string()
    } else {
        format!("{}/{}", parent, child)
    }
}

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
///
/// The `path` parameter is the tree path to this node (e.g., "distribution/utxoCohorts").
/// It's used to look up pre-computed PatternBaseResult from the analysis phase.
pub fn prepare_tree_node<'a>(
    node: &'a TreeNode,
    name: &str,
    path: &str,
    pattern_lookup: &BTreeMap<Vec<PatternField>, String>,
    metadata: &ClientMetadata,
    generated: &mut BTreeSet<String>,
) -> Option<TreeNodeContext<'a>> {
    let TreeNode::Branch(branch_children) = node else {
        return None;
    };

    let fields_with_child_info = get_fields_with_child_info(branch_children, name, pattern_lookup);
    let fields: Vec<PatternField> = fields_with_child_info
        .iter()
        .map(|(f, _)| f.clone())
        .collect();

    // Look up the pre-computed base result, or use a default that forces inlining
    let base_result = metadata
        .get_node_base(path)
        .cloned()
        .unwrap_or_else(PatternBaseResult::force_inline);

    // Skip if this matches a parameterizable pattern AND has no outlier AND field parts match
    let pattern_compatible = pattern_lookup
        .get(&fields)
        .and_then(|name| metadata.find_pattern(name))
        .is_none_or(|p| {
            p.is_suffix_mode() == base_result.is_suffix_mode
                && p.field_parts_match(&base_result.field_parts)
        });
    if let Some(pattern_name) = pattern_lookup.get(&fields)
        && pattern_name != name
        && metadata.is_parameterizable(pattern_name)
        && !base_result.has_outlier
        && pattern_compatible
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

            // Build child path and look up its pre-computed base result
            let child_path = build_child_path(path, child_name);
            let base_result = metadata
                .get_node_base(&child_path)
                .cloned()
                .unwrap_or_else(PatternBaseResult::force_inline);

            // For type annotations: use pattern type if ANY pattern matches
            let matches_any_pattern = child_fields
                .as_ref()
                .is_some_and(|cf| metadata.matches_pattern(cf));

            // Check if the pattern mode AND field parts match the instance
            // Uses is_none_or so that "no pattern" doesn't trigger inlining
            let pattern_compatible = child_fields
                .as_ref()
                .and_then(|cf| metadata.find_pattern_by_fields(cf))
                .is_none_or(|p| {
                    p.is_suffix_mode() == base_result.is_suffix_mode
                        && p.field_parts_match(&base_result.field_parts)
                });

            // should_inline determines if we generate an inline struct type
            // We inline if: it's a branch AND (doesn't match any pattern OR pattern incompatible OR has outlier)
            let should_inline = !is_leaf
                && (!matches_any_pattern || !pattern_compatible || base_result.has_outlier);

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
